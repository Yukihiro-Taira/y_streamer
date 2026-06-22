# PLAN: Media Probe Enrichment — More Data, More Checks

> PRIORITY 1 — do this before mp4_atoms box parsing (see PLAN__MP4_BOX_DIAGNOSTIC.md).
>
> Yukihiro W3 call key quote:
> "All of this is actually very important. That's why it's hard to say we don't need anything,
>  because everything is relevant here."
> "Yes, there is a possibility that it is not reading it correctly — it just has to test a lot."
>
> He also showed DaVinci vs Premiere vs FCPX files → same settings, are metadata tags identical?
> Answer: we don't know, because we're not displaying or checking half the data we already have.

---

## Context: What already exists

- `media_read_page.rs` has a `raw_json_pretty` panel — full ffprobe dump as `<pre>` block.
  Users can already see ALL raw data there. But it's unstructured, not checkable.
- `MediaStreamInfo` already has most fields, already populated by `ffprobe_mapper.rs`.

---

## The Real Problem

`MediaStreamInfo` already has all the fields. `ffprobe_mapper.rs` already populates them.
**Nothing uses them. Nothing checks them. Nothing displays them structured.**

```
FIELDS WE PARSE BUT NEVER CHECK OR DISPLAY:
  sample_aspect_ratio     → SAR 1:1 (square) vs 16:15 (anamorphic)?
  has_b_frames            → 0, 1, 2 — B-frame structure
  color_primaries         → bt709, bt2020, smpte431 (HDR flag)
  color_transfer          → bt709, smpte2084 (PQ=HDR), arib-std-b67 (HLG)
  color_space             → bt709, bt2020nc
  color_range             → tv (limited) vs pc (full range)
  chroma_location         → left, center, topleft
  frame_count             → exact frame count (nb_frames)
  refs                    → reference frames count
  tags                    → encoder, creation_time, language, handler_name...
  disposition             → default, forced, hearing_impaired, visual_impaired...
  chapters                → chapter markers (we parse but never show)
  format_tags             → title, encoder, major_brand, creation_time...
```

---

## MISSING from FfprobeStream: side_data_list

**`side_data_list`** is NOT in `FfprobeStream` struct. This is where HDR metadata actually lives.

ffprobe outputs this per-stream when HDR content detected:
```json
"side_data_list": [
  {
    "side_data_type": "Mastering display metadata",
    "red_x": "17000/50000", "red_y": "8000/50000",
    "green_x": "13250/50000", "green_y": "34500/50000",
    "blue_x": "7500/50000", "blue_y": "3000/50000",
    "white_point_x": "15635/50000", "white_point_y": "16450/50000",
    "min_luminance": "50/10000", "max_luminance": "10000000/10000"
  },
  {
    "side_data_type": "Content light level metadata",
    "max_content": 1000,
    "max_average": 400
  },
  {
    "side_data_type": "DOVI configuration record",  ← Dolby Vision
    "dv_version_major": 1, "dv_version_minor": 0,
    "dv_profile": 8, "dv_level": 6
  },
  {
    "side_data_type": "Spherical Mapping",  ← 360° video
    "projection": "equirectangular"
  },
  {
    "side_data_type": "Stereo 3D",  ← 3D video
    "type": "side by side"
  }
]
```

**What to add:**
```rust
// FfprobeStream — add field:
side_data_list: Option<Value>,   // keep as raw Value, flatten to Vec<MediaKeyValue>

// MediaStreamInfo — add field:
pub side_data: Vec<MediaKeyValue>,  // type → value pairs
// e.g. [("Mastering display metadata", "max_lum=1000nit min_lum=0.005nit"), ...]
```

**New checks this unlocks:**
```
check_hdr_metadata(side_data)
  "Mastering display metadata" present  → INFO  "HDR10: master display nit range present"
  "Content light level metadata"        → INFO  "MaxCLL: {max_content}nit MaxFALL: {avg}nit"
  "DOVI configuration record"           → INFO  "Dolby Vision profile {dv_profile}"
  "Spherical Mapping"                   → WARN  "360° video — verify player support"
  "Stereo 3D"                           → WARN  "3D video — verify player support"
```

---

## What to Build

### 1. New Diagnostic Rules (in `diagnostic_rules.rs`)

```
check_sample_aspect_ratio(sar)
  "1:1" | "0:1" | ""   → PASS  "Square pixels"
  other                 → WARN  "Anamorphic SAR {sar} — verify display scaling"

check_color_space(primaries, transfer, space, range)
  primaries="bt709" transfer="bt709"           → PASS  "BT.709 — standard HD"
  transfer="smpte2084"                          → WARN  "PQ/HDR10 — verify player support"
  transfer="arib-std-b67"                       → WARN  "HLG HDR — broadcast HDR"
  primaries="bt2020"                            → WARN  "BT.2020 wide gamut — HDR content"
  range="pc"                                    → WARN  "Full range — may crush on TV display"
  primaries="" AND transfer=""                  → WARN  "Color space untagged — assume BT.709"

check_b_frames(has_b_frames)
  "0"                  → PASS  "No B-frames — compatible with all decoders"
  "1" | "2"            → INFO  "B-frames: {n} — verify decoder support on target"
  >2                   → WARN  "B-frames: {n} — may cause issues on low-power devices"

check_chroma_location(chroma_location)
  "left" | ""          → PASS  "Chroma location: left (H.264 standard)"
  "center"             → WARN  "Chroma center — unusual for H.264, verify encoder"
  "topleft"            → INFO  "Chroma topleft — interlaced convention"

check_forced_subtitles(disposition vec)
  forced=1             → INFO  "Forced subtitle stream present"

check_encoder_tag(format_tags)
  tag "encoder" present  → INFO  "Encoded with: {value}"  (useful for NLE comparison)
  tag "creation_time"    → INFO  "Created: {value}"
```

---

### 2. Raw Data Inspector Panel (UI — `diagnostic_page.rs`)

New collapsible section below the diagnostic checks:

```
┌─────────────────────────────────────────────────────────────────┐
│ VIDEO DIAGNOSTIC                                                 │
├──────────────────────────────┬──────────────────────────────────┤
│ CHECKS (existing)            │ CHECKS (new rules above)         │
│ ✅ Container: mp4            │ ✅ Square pixels (1:1)           │
│ ✅ Codec: H.264              │ ⚠️  HDR: PQ/HDR10 detected       │
│ ⚠️  VFR detected             │ ✅ BT.709 color space            │
│ ✅ Audio: AAC stereo         │ ℹ️  B-frames: 2                  │
│ ✅ Sample rate: 48kHz        │ ℹ️  Encoded with: DaVinci Resolve│
└──────────────────────────────┴──────────────────────────────────┘

▼ RAW STREAM DATA  (collapsible — new)
┌─────────────────────────────────────────────────────────────────┐
│ VIDEO STREAM                                                     │
│  Codec          h264 (H.264 / AVC / MPEG-4 AVC)                 │
│  Profile        High                                             │
│  Resolution     1920 × 1080  (coded: 1920 × 1088)               │
│  SAR / DAR      1:1  /  16:9                                     │
│  Pixel format   yuv420p                                          │
│  Color          bt709 / bt709 / bt709  range: tv                 │
│  Frame rate     23.976 fps (avg)  r: 24000/1001                  │
│  Frame count    1723 frames                                      │
│  B-frames       2                                                │
│  Chroma loc     left                                             │
│  Bitrate        8 432 kbps                                       │
│  Level          4.0    Refs: 4    NAL: 4    is_avc: true         │
├─────────────────────────────────────────────────────────────────┤
│ AUDIO STREAM                                                     │
│  Codec          aac (AAC LC)                                     │
│  Sample rate    48000 Hz                                         │
│  Channels       stereo (FL+FR)                                   │
│  Bit depth      fltp (32-bit float planar)                       │
│  Bitrate        192 kbps                                         │
├─────────────────────────────────────────────────────────────────┤
│ CONTAINER / FORMAT                                               │
│  Format         mov,mp4,m4a,3gp (QuickTime / MOV)               │
│  Duration       1:12.433                                         │
│  File size      89.2 MB                                          │
│  Bitrate        9 871 kbps                                       │
│  Probe score    100                                              │
├─────────────────────────────────────────────────────────────────┤
│ TAGS                                                             │
│  encoder        DaVinci Resolve Studio 18.6                      │
│  creation_time  2024-11-03T14:22:11Z                             │
│  major_brand    qt                                               │
├─────────────────────────────────────────────────────────────────┤
│ CHAPTERS  (2)                                                    │
│  00:00:00 → 00:00:42  "Act 1"                                    │
│  00:00:42 → 00:01:12  "Act 2"                                    │
└─────────────────────────────────────────────────────────────────┘
```

---

### 3. NLE Comparison Goal (Testing, not code)

Yukihiro's actual concern: same export settings in DaVinci / Premiere / FCPX — does metadata match?

Test matrix to build manually once UI is done:

```
FILE              NLE              CODEC    SAR   COLOR         ENCODER TAG
davinci_h264.mp4  DaVinci Resolve  H.264    1:1   bt709/bt709   DaVinci Resolve Studio x.x
premiere_h264.mp4 Premiere Pro     H.264    1:1   bt709/bt709   Adobe Premiere x.x
fcpx_h264.mov     Final Cut Pro X  H.264    1:1   bt709/bt709   ?
iphone.mov        iPhone (HEVC)    HEVC     1:1   bt2020/hdr10  com.apple.quicktime
davinci_prores.mov DaVinci Resolve  ProRes   1:1   bt709/bt709   DaVinci Resolve Studio x.x
```

Goal: identify what differs between NLEs. Feed findings back into diagnostic rules.

---

## Implementation Steps

### Phase 1 — Capture missing ffprobe fields (struct + mapper)

Add to `FfprobeStream` + map to `MediaStreamInfo`:
- [ ] `side_data_list: Option<Value>` → `side_data: Vec<MediaKeyValue>` (flatten type → value)
- [ ] `initial_padding: Option<i64>` → `initial_padding: String`
- [ ] `closed_captions: Option<i64>` → `closed_captions: String`
- [ ] `max_bit_rate: Option<String>` → `max_bit_rate: String`
- [ ] `nb_read_frames: Option<String>` → `nb_read_frames: String`
- [ ] `extradata_size: Option<i64>` → `extradata_size: String`
- [ ] `codec_time_base: Option<String>` → `codec_time_base: String`

### Phase 2 — New diagnostic rules
- [ ] `check_sample_aspect_ratio(sar)` in `diagnostic_rules.rs`
- [ ] `check_color_space(primaries, transfer, space, range)` — BT.709 / HDR10 / HLG / full-range
- [ ] `check_hdr_metadata(side_data)` — mastering display nits, MaxCLL, DoVi, 360°, Stereo3D
- [ ] `check_b_frames(has_b_frames)`
- [ ] `check_chroma_location(chroma_location)`
- [ ] `check_closed_captions(closed_captions)`
- [ ] `check_encoder_tag(format_tags)` — NLE fingerprint
- [ ] Wire all new checks into `run()` function
- [ ] Unit tests per check (fixture: DaVinci ProRes, iPhone HEVC, web H264)

### Phase 3 — Raw Data Inspector UI
- [ ] New collapsible `StreamInfoPanel` component in `diagnostic_page.rs`
- [ ] Video stream section — all fields (codec, profile, SAR/DAR, color, fps, frame count, B-frames, level, refs, NAL, side_data)
- [ ] Audio stream section — codec, rate, channels, bit depth, initial_padding
- [ ] Container / format section — format, duration, size, bitrate, probe_score
- [ ] Tags section — key/value table (format_tags + per-stream tags)
- [ ] Chapters section (if any)
- [ ] Disposition section (non-default values only)
- [ ] Side data section (HDR, 360°, DoVi — if present)
- [ ] Collapsible (default: collapsed)

### Phase 4 — Testing with real files (NLE matrix)
- [ ] DaVinci ProRes MOV → color tags, encoder tag, side_data
- [ ] DaVinci H.264 MP4 → same settings, compare tags
- [ ] Premiere Pro H.264 → encoder tag differs? SAR? B-frames?
- [ ] Final Cut Pro X MOV → major_brand=qt, encoder tag?
- [ ] iPhone HEVC MOV → color_transfer=smpte2084, side_data MaxCLL/MaxFALL
- [ ] iPhone HEVC (Dolby Vision) → side_data DOVI record present?
- [ ] 360° video → side_data Spherical Mapping
- [ ] MKV file → disposition fields, forced subtitles
- [ ] Note any fields empty when they should not be → feed back into mapper fixes

---

## Complete Field Audit: ffprobe → our struct

### FfprobeStream — fields NOT yet captured

```
MISSING IN FfprobeStream / MediaStreamInfo:
  side_data_list          → HDR mastering display, MaxCLL, DoVi, 360°, Stereo3D
  nb_read_frames          → actual decoded frames (vs declared nb_frames)
  nb_read_packets         → packets read
  extradata_size          → codec private data size (bytes)
  initial_padding         → audio encoder delay samples (important for AAC gapless)
  closed_captions         → 0/1, relevant for broadcast
  film_grain              → AV1 film grain params
  codec_time_base         → internal codec timebase (not same as time_base)
  max_bit_rate            → declared max bitrate (VBR ceiling)
  bits_per_coded_sample   → codec bits per sample (not same as bits_per_raw_sample)
```

### FfprobeFormat — fields NOT yet captured

```
MISSING IN FfprobeFormat:
  filename                → original file path as ffprobe sees it
  nb_streams              → we compute from len() but not stored raw
```

### What we CAN'T get from ffprobe but mp4_atoms could give us (Phase 2):
```
  GOP structure           → stss box (exact keyframe sample numbers)
  Edit list               → elst box (A/V sync delay)
  Fragmented MP4          → moof box presence
  Composition offsets     → ctts box (B-frame DTS/PTS deltas)
  Sample sizes            → stsz box (per-frame byte sizes)
```

---

## Files to Modify

| Action | File | What |
|---|---|---|
| MODIFY | `src/domain/media_read/service/ffprobe_mapper.rs` | Add `side_data_list`, `initial_padding`, `closed_captions`, `max_bit_rate` to `FfprobeStream` |
| MODIFY | `src/domain/media_read/data/media_probe_report.rs` | Add `side_data`, `initial_padding`, `closed_captions`, `max_bit_rate` to `MediaStreamInfo` |
| MODIFY | `src/domain/diagnostic/service/diagnostic_rules.rs` | Add 6 new checks |
| MODIFY | `src/domain/diagnostic/routing/diagnostic_page.rs` | Add raw inspector panel |

---

## Non-Goals

- Waveform display (separate feature)
- Scene detection (separate feature)
- MP4 box tree (see PLAN__MP4_BOX_DIAGNOSTIC.md — Phase 2)
- Storing probe results in database
