# Video Debugger — Pivot Plan

## Vision

Drop video file → instant playback + full diagnostic report.  
One tool replacing MediaInfo + VLC + ffprobe CLI. No paywall, no cookies, cross-platform.  
Rust + ffmpeg = perfect fit.

### App Flow

```
[DROP FILE] → [ffprobe] → [DiagnosticReport] → [UI: pass/warn/fail]
                                                       ↓
                                               [one-click FIX] → [ffmpeg encode]
```

### What It Will Look Like

```
┌─────────────────────────────────────────────────────────────┐
│                   /diagnostic  (new page)                   │
│                                                             │
│   ┌─────────────────────────────────────────────────────┐  │
│   │             DROP VIDEO FILES HERE                   │  │
│   └─────────────────────────────────────────────────────┘  │
│                          │                                  │
│                    ffprobe runs                             │
│                          │                                  │
│              DiagnosticRules::run()                         │
│                          │                                  │
│   ┌──────────────────────┴──────────────────────────────┐  │
│   │  filename.mp4                             🟡 3 warn  │  │
│   ├───────────────┬─────────────────────────────────────┤  │
│   │  [ preview ]  │  ✅ container: mp4                  │  │
│   │               │  ✅ video codec: h264               │  │
│   │               │  ✅ subtitles embedded              │  │
│   │               │  🟡 audio: stereo (expected mono)   │  │
│   │               │  🟡 color depth: 10-bit (HDR)       │  │
│   │               │  🟡 frame rate: 23.97 (not 25/30)   │  │
│   │               │  ✅ audio bit depth: 16-bit         │  │
│   │               │  ✅ pixel format: yuv420p           │  │
│   │               │  [ Export JSON ]  [ Fix & Convert ] │  │
│   └───────────────┴─────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Code Path

```
MediaProbeReport (already exists)
        │
        ▼
DiagnosticRules::run()        ← Step 2 — pure Rust, no UI, unit-testable
        │
        ▼
DiagnosticReport { checks: Vec<DiagnosticCheck> }
        │
        ▼
diagnostic_page.rs UI         ← Step 4 — new route, nothing existing touched
```

---

## Open Questions (decide before coding)

```
┌────────────────────────────────┬────────────────────────────────────────────────────────┐
│            Question            │                        Options                        │
├────────────────────────────────┼────────────────────────────────────────────────────────┤
│ Where does output file go?     │ A) same folder as input  B) user picks via dialog      │
├────────────────────────────────┼────────────────────────────────────────────────────────┤
│ ffprobe/ffmpeg bundled or not? │ A) bundle in app (larger binary, zero setup)           │
│                                │ B) show install instructions (lighter, user installs)  │
├────────────────────────────────┼────────────────────────────────────────────────────────┤
│ Large file progress?           │ A) progress bar via ffprobe -progress  B) spinner      │
└────────────────────────────────┴────────────────────────────────────────────────────────┘
```

> Bundling recommendation: bundle ffmpeg/ffprobe. Users of a "no-install debug tool" won't want to install ffmpeg separately. Adds ~60MB to binary but removes all setup friction.

---

## Architecture Decision

**Local-first (recommended).** ffprobe + ffmpeg run on user's machine — no file upload, no server, no privacy issue.  
Tauri desktop app or CLI. Not a web server.

> If server-side: files upload to backend = user privacy concern. Avoid unless explicitly needed.

---

## Target Platforms (define "viable")

"Platform viability" must be concrete. Three profiles:

```
┌─────────────────┬──────────────────────────────────────────────────────┐
│    Profile      │                     Requirements                     │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Web / YouTube   │ mp4, H.264, AAC, 8-bit color, subtitles as sidecar  │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Broadcast / TV  │ mov/mxf, PCM 16-bit audio, mono tracks, timecode    │
├─────────────────┼──────────────────────────────────────────────────────┤
│ Mobile          │ mp4, H.264 Baseline, AAC, max 1080p, no 10-bit      │
└─────────────────┴──────────────────────────────────────────────────────┘
```

---

## What We Already Have

```
┌────────────────────────────────────────┬───────────────────────────────────────────────────────┬────────────┐
│                  Code                  │                       Location                        │   Status   │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ ffprobe runner + parser                │ media_read/service/ffprobe_runner.rs                  │ ✓ done     │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ ffprobe → MediaProbeReport mapper      │ media_read/service/ffprobe_mapper.rs                  │ ✓ done     │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ Subtitle extraction (ffmpeg → SRT)     │ media_read/service/subtitle_extractor.rs              │ ✓ done     │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ Thumbnail generation                   │ media_read/service/thumbnail_generator.rs             │ ✓ done     │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ Drag-drop dropzone UI                  │ components/ui/dropzone.rs                             │ ✓ done     │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ Drop → preview + basic metadata demo   │ components/demos/demo_video_inspector.rs              │ ✓ done     │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ TUI job queue + drag/drop prototype    │ __TUI_FFMRUST/                                        │ ✓ proto    │
├────────────────────────────────────────┼───────────────────────────────────────────────────────┼────────────┤
│ MediaProbeReport struct                │ media_read/data/media_probe_report.rs                 │ ✓ done     │
└────────────────────────────────────────┴───────────────────────────────────────────────────────┴────────────┘
```

ffprobe_mapper already maps: `channel_layout`, `bits_per_sample`, `codec_name`, `audio_count`, `subtitle_count`, `bit_rate`. The raw data for all checks exists.

---

## What's Missing

```
┌───────────────────────────────────┬───────────────────────────────────────────┐
│              Feature              │                    Gap                    │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Audio mono/stereo check           │ channel_layout in stream, not displayed   │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Bit depth check (audio 16-bit)    │ bits_per_sample mapped, no validation     │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Color depth check (video 8-bit)   │ bits_per_raw_sample mapped, no validation │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Platform compat report            │ no profile-based viability logic          │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Format conversion                 │ ffmpeg encode pipeline, 0 code            │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Multi-file VLC-style preview      │ demo has 1 file card, needs grid          │
├───────────────────────────────────┼───────────────────────────────────────────┤
│ Export corrected metadata         │ no write-back                             │
└───────────────────────────────────┴───────────────────────────────────────────┘
```

### 1. Diagnostic / Integrity Report
Derive from existing `MediaProbeReport` — no new infra needed.

> ⚠️ **Bit depth has two meanings in ffprobe — keep them separate:**
> - **Audio** → `streams[].bits_per_sample` = PCM depth (16-bit = pass, 24-bit = ok, 10-bit = fail)
> - **Video** → `streams[].bits_per_raw_sample` = color depth (8-bit = wide compat, 10-bit = HDR/warn for web)

> ⚠️ **Container ≠ codec — check both separately:**
> - Container: `format_name` (mp4, mov, mkv…)
> - Video codec: `streams[].codec_name` where codec_type = "video" (h264, hevc, av1…)
> - Audio codec: `streams[].codec_name` where codec_type = "audio" (aac, mp3, pcm…)
> - A `.mp4` with HEVC inside fails on old Android — container alone does not mean wide compat.

```
┌──────────────────────────────────┬─────────────────────────────────┬──────────────────────────────────┐
│              Check               │           Field used            │               Rule               │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Container format                 │ format_name                     │ mp4/mov = ok, mkv = warn web     │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Video codec                      │ streams[video].codec_name       │ h264 = wide, hevc/av1 = warn     │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio codec                      │ streams[audio].codec_name       │ aac = wide, mp3 = ok, pcm = warn │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Subtitle embedded                │ subtitle_count                  │ > 0 = pass                       │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Subtitle codec valid             │ streams[sub].codec_name         │ mov_text / ass / srt = ok        │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio track present              │ audio_count                     │ warn if 0                        │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio: mono tracks (not stereo)  │ streams[audio].channel_layout   │ "mono" = pass, "stereo" = warn   │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio: separate mono tracks      │ audio_count + channel_layout    │ N × mono tracks = pass           │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio sample rate                │ streams[audio].sample_rate      │ 48000 = broadcast, 44100 = warn  │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio bit depth (PCM)            │ streams[audio].bits_per_sample  │ 16 = pass, 24 = ok, other = warn │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Video color depth                │ streams[video].bits_per_raw_sample │ 8 = compat, 10 = HDR warn   │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Platform viability (per profile) │ container + video codec + audio │ see platform profiles above      │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Pixel format                     │ streams[video].pix_fmt          │ yuv420p = pass, else = warn web  │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ H.264 profile / level            │ streams[video].profile + level  │ Baseline/Main = compat, High warn│
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Interlaced video                 │ streams[video].field_order      │ progressive = pass, else = warn  │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Rotation metadata                │ streams[video].tags.rotate      │ 0 = pass, 90/180/270 = warn      │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Variable frame rate (VFR)        │ r_frame_rate vs avg_frame_rate  │ equal = pass, differ = warn      │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ A/V sync drift                   │ streams[audio/video].start_time │ diff > 100ms = warn              │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ File extension mismatch          │ format_name vs file extension   │ mismatch = warn                  │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Attached pic false positive      │ streams[].disposition           │ skip attached_pic = 1 streams    │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Audio/video duration mismatch    │ streams[a].duration vs [v]      │ diff > 500ms = warn              │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ HDR metadata on 10-bit           │ streams[video].side_data_list   │ HDR10/DV/HLG detected = warn     │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Closed captions in video stream  │ streams[video].side_data_list   │ CEA-608/708 = note (not subtitles│
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Editing codec (not delivery)     │ streams[video].codec_name       │ prores/dnxhd/cineform = warn     │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Default stream flag              │ streams[audio].disposition      │ no default track = warn          │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Forced subtitle flag             │ streams[sub].disposition        │ forced = 1, note per platform    │
├──────────────────────────────────┼─────────────────────────────────┼──────────────────────────────────┤
│ Frame rate                       │ streams[video].r_frame_rate     │ 25/29.97/30 = ok, else = warn    │
└──────────────────────────────────┴─────────────────────────────────┴──────────────────────────────────┘
```

> ⚠️ **HDR conversion rule:** never silently re-encode 10-bit HDR → 8-bit. Must apply tone-mapping  
> (`-vf zscale,tonemap,zscale,format=yuv420p`) or warn user explicitly before proceeding.

**Error states** — alongside pass/warn/fail:
```
DiagnosticStatus::Error(reason)   ← corrupted file, ffprobe crash, unsupported codec
DiagnosticStatus::Pending         ← large file, ffprobe still running
```

**Data pipeline:**

```
MediaProbeReport
    │
    ▼
DiagnosticRules::run(&report, profile)   ← profile = Web | Broadcast | Mobile
    │
    ▼
Vec<DiagnosticCheck { status, label, detail }>
    │
    ├─ pass  → green badge
    ├─ warn  → yellow badge
    └─ fail  → red badge + suggest fix
```

Output: `DiagnosticReport` struct with `Vec<DiagnosticCheck>` (pass/warn/fail + message).

---

### 2. Multi-file Preview (VLC-style)

Current demo: 1 file card.  
Need: drop N files → grid of cards, each with:
- playback preview
- quick diagnostic badge (green/yellow/red)
- subtitle/audio track list

```
┌─────────────────────────────────────────────┐
│  [ DROP FILES HERE ]                        │
├───────────────┬─────────────────────────────┤
│  preview      │  filename.mp4               │
│  [ video ]    │  ✓ subtitles embedded       │
│               │  ✓ audio mono               │
│               │  ✗ video: 10-bit color      │
│               │  ⚠ audio: stereo track      │
│               │  [ Fix & Export ]           │
└───────────────┴─────────────────────────────┘
```

---

### 3. Metadata Export + File Comparison

After inspect: export button → JSON / CSV with all stream data.  
Bonus: export corrected metadata (title, language tags via ffmpeg `-metadata`).

**File comparison** — drop original + converted side by side:
```
┌─────────────────────────┬─────────────────────────┐
│     original.mov        │     converted.mp4        │
├─────────────────────────┼─────────────────────────┤
│ codec:    prores        │ codec:    h264           │
│ pix_fmt:  yuv422p       │ pix_fmt:  yuv420p        │
│ audio:    stereo        │ audio:    mono ×2        │
│ subs:     0             │ subs:     1 (mov_text)   │
│ size:     4.2 GB        │ size:     280 MB         │
└─────────────────────────┴─────────────────────────┘
```
Closes the loop: user said "run through multiple tools to make sure file is ready."

---

### 4. Format Conversion

> ⚠️ Re-encoding = quality loss. User must explicitly choose codec/CRF/preset — no silent "fix".

Priority fixes for debugger:
- `mov → mp4` (container remux, lossless)
- `10-bit color → 8-bit` re-encode (lossy, user confirms)
- embed subtitles (`-c:s mov_text`)
- audio: stereo → dual mono split (`-ac 1 -filter_complex`)
- audio resample to 48kHz 16-bit PCM

---

## Build Order

Strict dependency order — each step unblocks the next.

```
1. Define platform profiles (Web / Broadcast / Mobile)  ← decision, no code
2. DiagnosticReport struct + rules (split audio/video bit depth)  ← pure Rust, testable
3. Wire rules → MediaProbeReport  ← server already runs ffprobe
4. Diagnostic UI (pass/warn/fail badges)  ← user value unlocked

5. Multi-file grid  ← needs #4 badges
6. Subtitle/audio track list per card  ← needs #5

7. JSON/CSV export  ← needs #3 data
8. Format conversion (with explicit user confirm)  ← needs #2 rules to know WHAT to fix
9. Metadata write-back  ← needs #8 ffmpeg pipeline
```

Start at 1 (no code). Everything downstream needs diagnostic data.  
Conversion without knowing what's broken = blind.

---

## Testing Strategy

Diagnostic rules need sample files with known issues — can't test rules without them.

```
tests/fixtures/
  ├── ok_h264_aac_mono_16bit.mp4       ← all pass
  ├── warn_stereo_audio.mp4            ← stereo track
  ├── warn_10bit_color.mp4             ← yuv420p10le
  ├── warn_hevc_in_mp4.mp4             ← container ok, codec warn
  ├── fail_no_audio.mp4                ← audio_count = 0
  ├── fail_extension_mismatch.mp4      ← actually mkv inside
  ├── warn_vfr.mp4                     ← r_frame_rate ≠ avg_frame_rate
  └── warn_rotation.mp4                ← rotate=90 tag
```

Unit test each `DiagnosticRule` against fixtures before wiring to UI.

---

## Node Workflow

Deprioritized. Comes after Phase 3. See `PLAN_NODES_GSTREAMER.md`.
