# PLAN: MP4 Box-Level Diagnostic Extension

> ⚠️  PRIORITY NOTE (from W3 call — Yukihiro)
>
> This plan is PHASE 2. Do PLAN__MEDIA_PROBE_ENRICHMENT.md first.
> Yukihiro's main point: "all data needs to be correct and complete."
> Box-level parsing (GOP, edit list) is useful but secondary to having
> accurate ffprobe fields (SAR, HDR, B-frames, tags) from real-world files.
>
> ---
>
> Extends the existing `diagnostic` domain with binary MP4 atom parsing (WASM-side).
> No Python dependency. Pure Rust byte parsing of the uploaded file.

---

## Motivation

Current diagnostic uses **ffprobe output** (server-side) for all checks.
mp4analyzer (Python) showed us what box-level parsing unlocks:

| What ffprobe gives us | What box parsing adds |
|---|---|
| Codec, container, VFR | Exact keyframe positions (stss) |
| Audio channels, sample rate | GOP interval in frames AND seconds |
| A/V sync (start_time diff) | Edit list presence + offset (elst) |
| Duration mismatch | Fragmented MP4 detection (moof) |
| Pixel format, color depth | Box tree structure (visual) |

---

## Architecture Overview

```
FILE UPLOAD (browser)
        │
        ▼
  [Dioxus WASM]
        │
  ┌─────┴──────────────────────────────────────┐
  │  EXISTING FLOW                             │
  │  upload_file() → POST /api/media/probe     │
  │  → ffprobe → MediaProbeReport              │
  │  → diagnostic_rules::run() → DiagnosticReport│
  └─────┬──────────────────────────────────────┘
        │
  ┌─────┴──────────────────────────────────────┐
  │  NEW FLOW (WASM, no server round-trip)     │
  │  file_bytes → mp4_box_parser::parse()      │
  │  → Mp4BoxTree                              │
  │  → box_diagnostic_rules::run()             │
  │  → BoxDiagnosticReport                     │
  └────────────────────────────────────────────┘
        │
        ▼
  DiagnosticPage renders BOTH reports side by side
```

---

## Domain Map (current + new)

```
src/domain/
├── mod.rs               ← add `pub mod mp4_atoms`
│
├── auth/                ← existing
├── bugreports/          ← existing
├── media_read/          ← existing (ffprobe server-side)
├── media_write/         ← existing (ffmpeg jobs)
├── observability/       ← existing
├── test/                ← existing
│
├── diagnostic/          ← existing — ffprobe-based rules
│   ├── mod.rs
│   ├── data/
│   │   ├── diagnostic_report.rs
│   │   └── platform_profile.rs
│   ├── routing/
│   │   └── diagnostic_page.rs   ← UI extended (Phase 3)
│   └── service/
│       └── diagnostic_rules.rs
│
└── mp4_atoms/           ← NEW — binary box parsing, 100% WASM client-side
    ├── mod.rs
    ├── data/
    │   ├── mod.rs
    │   ├── box_tree.rs          ← Mp4Box, Mp4BoxTree structs
    │   └── box_diagnostic.rs    ← BoxDiagnosticReport (reuses DiagnosticCheck)
    └── service/
        ├── mod.rs
        ├── box_parser.rs        ← parse raw bytes → Mp4BoxTree
        └── box_rules.rs         ← GOP, edit list, fMP4 checks
```

---

## Data Structures

```rust
// box_tree.rs
pub struct Mp4Box {
    pub name: [u8; 4],      // "moov", "trak", "stss", etc.
    pub size: u64,
    pub offset: u64,         // byte offset in file
    pub children: Vec<Mp4Box>,
    pub payload: BoxPayload, // parsed content for known boxes
}

pub enum BoxPayload {
    Moov,
    Mvhd { duration: u32, timescale: u32 },
    Stss { sample_numbers: Vec<u32> },    // keyframe table
    Stts { entries: Vec<(u32, u32)> },    // time-to-sample
    Elst { entries: Vec<ElstEntry> },     // edit list
    Moof,                                  // fragment header
    Unknown,
}

pub struct ElstEntry {
    pub segment_duration: u64,
    pub media_time: i64,     // negative = empty edit (A/V sync delay)
    pub media_rate: f32,
}

pub struct Mp4BoxTree {
    pub boxes: Vec<Mp4Box>,
    pub timescale: u32,
    pub duration_secs: f64,
}
```

```rust
// box_diagnostic.rs  (mirrors diagnostic_report.rs pattern)
pub struct BoxDiagnosticReport {
    pub checks: Vec<DiagnosticCheck>,  // reuse existing DiagnosticCheck
    pub keyframe_positions: Vec<u32>,  // sample numbers from stss
    pub gop_avg_frames: f32,
    pub gop_avg_secs: f32,
    pub is_fragmented: bool,
    pub edit_list_offset_ms: Option<i64>,
}
```

---

## New Diagnostic Checks

### 1. GOP / Keyframe Interval

```
stss box → keyframe sample numbers → compute gaps

Example stss: [1, 120, 240, 360, ...]
Gap = 120 frames @ 24fps = 5.0 seconds

Rule:
  Web/Mobile:  gap > 96 frames (~4s @ 24fps)  → WARN
               gap > 240 frames (~10s)          → FAIL
  Broadcast:   gap > 2s                         → WARN
```

### 2. Edit List (elst) Detection

```
elst entry where media_time < 0  →  empty edit = intentional delay

Common cause: QuickTime adds -1 edit on iPhone videos.
ffprobe reports A/V sync OK but browser players desync.

Rule:
  elst absent                    → PASS  "No edit list"
  elst present, media_time == 0  → WARN  "Edit list present, offset 0ms"
  elst present, media_time < 0   → FAIL  "Edit list delay: -{N}ms — A/V sync risk"
```

### 3. Fragmented MP4 Detection

```
moof box present at top level → fMP4

Rule:
  no moof       → PASS  "Standard MP4"
  moof present  → INFO  "Fragmented MP4 (fMP4) — good for HLS/DASH streaming"
```

---

## UI Extension: DiagnosticPage

```
┌─────────────────────────────────────────────────────────────────┐
│ VIDEO DIAGNOSTIC              Platform: [Web] [Broadcast][Mobile]│
├─────────────────────────────────────────────────────────────────┤
│ DROP ZONE                                                        │
│ ┌──────────────────────────────────────────────────────────────┐│
│ │           Drop MP4 / MOV / MKV here                          ││
│ └──────────────────────────────────────────────────────────────┘│
├──────────────────────────┬──────────────────────────────────────┤
│ FFPROBE CHECKS           │ BOX-LEVEL CHECKS  (new panel)        │
│ ✅ Container: mp4        │ ✅ No edit list                       │
│ ✅ Codec: H.264          │ ⚠️  GOP: 120f = 5.0s (warn >4s)      │
│ ⚠️  VFR detected         │ ✅ Standard MP4 (not fragmented)      │
│ ✅ Audio: AAC stereo     │                                       │
│ ✅ Sample rate: 48kHz    │ GOP TIMELINE                          │
│ ✅ A/V sync OK           │ [I]░░░░░[I]░░░░░[I]░░░░░[I]░░░░░    │
│                          │  0s     5s     10s    15s    20s      │
│                          │ Keyframes: 12  Avg interval: 5.0s     │
├──────────────────────────┴──────────────────────────────────────┤
│ MP4 BOX TREE  (collapsible)                                      │
│ ▼ ftyp  (size: 24)                                               │
│ ▼ moov  (size: 1.2MB)                                            │
│   ├─ mvhd  duration: 30.5s  timescale: 90000                    │
│   ▼ trak  [video]                                                │
│   │  ├─ tkhd  1920×1080                                          │
│   │  ▼ mdia                                                      │
│   │    ▼ minf                                                     │
│   │      ▼ stbl                                                   │
│   │        ├─ stsd  avc1                                          │
│   │        ├─ stss  ← 12 keyframes                               │
│   │        └─ stts  ← sample timing table                        │
│   ▼ trak  [audio]                                                │
│     └─ ...                                                       │
│ ▶ mdat  (size: 16.8MB)  [click to collapse]                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Implementation Phases

### Phase 1 — Parser Core (no UI)
- [ ] `box_parser.rs`: read 8-byte box header loop (size + name)
- [ ] Parse `moov`, `trak`, `mdia`, `minf`, `stbl` recursively
- [ ] Parse payload: `stss`, `stts`, `elst`, detect `moof`
- [ ] Unit tests with real fixture .mp4 files

### Phase 2 — Box Rules
- [ ] `box_rules.rs`: GOP check, edit list check, fMP4 check
- [ ] Reuse `DiagnosticCheck` / `DiagnosticStatus` structs
- [ ] Unit tests per rule

### Phase 3 — UI Integration
- [ ] `BoxDiagnosticReport` signal in `DiagnosticPage`
- [ ] Run box parser in `spawn()` alongside ffprobe call (parallel)
- [ ] Render "Box-Level Checks" panel
- [ ] GOP timeline bar component
- [ ] Collapsible MP4 box tree component

---

## WASM Byte Parsing Strategy

```
File arrives as Vec<u8> from dioxus FileData.

Box header format:
  bytes 0..3  →  size: u32 big-endian
  bytes 4..7  →  name: [u8; 4]
  if size == 1:
    bytes 8..15 → extended_size: u64

Parse loop:
  offset = 0
  while offset < file.len():
    header = parse_header(file, offset)
    if header.name in CONTAINER_BOXES:
      recurse into children
    else:
      parse_payload(header.name, file, offset+8, header.size-8)
    offset += header.size

CONTAINER_BOXES = [moov, trak, mdia, minf, stbl, moof, traf, udta, meta]
```

---

## Files to Create / Modify

| Action | File |
|---|---|
| CREATE | `src/domain/mp4_atoms/mod.rs` |
| CREATE | `src/domain/mp4_atoms/data/mod.rs` |
| CREATE | `src/domain/mp4_atoms/data/box_tree.rs` |
| CREATE | `src/domain/mp4_atoms/data/box_diagnostic.rs` |
| CREATE | `src/domain/mp4_atoms/service/mod.rs` |
| CREATE | `src/domain/mp4_atoms/service/box_parser.rs` |
| CREATE | `src/domain/mp4_atoms/service/box_rules.rs` |
| MODIFY | `src/domain/mod.rs` — add `pub mod mp4_atoms` |
| MODIFY | `src/domain/diagnostic/routing/diagnostic_page.rs` — add box panels |

---

## Non-Goals

- No video playback / frame preview (out of scope)
- No writing/patching MP4 boxes
- No support for `.ts`, `.webm`, `.mkv` box parsing (MP4/MOV only)
- No server-side box parsing (stays 100% WASM client-side)
