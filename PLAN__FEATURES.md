# Features

## Done ✓

| Feature | Domain | Description |
|---|---|---|
| Media inspect | `media_read` | Upload → ffprobe → codec, resolution, duration, bitrate |
| Compression / transcode | `media_write` | FFmpeg jobs server-side |
| Dropzone UI | `components/ui` | Drag-drop, image/video preview, file type icons |
| Dropzone grid view | `components/ui` | Card grid with hover overlay |
| Auth | `auth` | Login, roles, protected routes |
| Bug reports | `bugreports` | Basic CRUD |

---

## media_read — To do

| Status | Feature | Difficulty | Priority | Description |
|---|---|---|---|---|
| ✅ | **Thumbnail generation** | 🟡 Medium | ✅ Done | 3 frames at 10%/50%/90% → base64 in response, shown in UI |
| ✅ | **Subtitle extract** | 🟡 Medium | ✅ Done | Extract all streams → SRT, browser download button per track |
| ⬜ | **Waveform** | 🟢 Low | 🟡 Medium | Audio file → waveform image (ffmpeg `showwavespic`) |
| ⬜ | **Scene detection** | 🟡 Medium | 🟡 Medium | Detect cuts / scene changes (ffmpeg `scdet` filter) |

---

## media_write — To do

| Status | Feature | Difficulty | Priority | Description |
|---|---|---|---|---|
| ⬜ | **Job progress** | 🟡 Medium | 🔴 High | Real-time FFmpeg progress (%, ETA) via SSE or polling |
| ⬜ | **Download result** | 🟢 Low | 🔴 High | Download processed file after job completes |
| ⬜ | **Job history** | 🟡 Medium | 🔴 High | List past jobs + status (pending / running / done / failed) |
| ⬜ | **Format conversion** | 🟡 Medium | 🔴 High | mp4→webm, mov→mp4, mkv→mp4 |
| ⬜ | **Audio extract** | 🟢 Low | 🟡 Medium | Strip video → mp3 / wav / aac |
| ⬜ | **Clip trim** | 🟡 Medium | 🟡 Medium | Cut start/end timestamps (`-ss` `-to`) |
| ⬜ | **Resize** | 🟡 Medium | 🟡 Medium | Scale to 720p / 1080p / custom |
| ⬜ | **Bitrate control** | 🟡 Medium | 🟡 Medium | Set target bitrate / CRF |
| ⬜ | **Batch jobs** | 🟠 High | 🟡 Medium | Queue multiple files, process in parallel |
| ⬜ | **Watermark** | 🟡 Medium | 🟢 Low | Overlay image/text on video |
| ⬜ | **Audio Broadcast Convert** | 🟠 High | 🟡 Medium | converts audio tracks to Broadcast delivery specs (stereo to mono)(atmos, 5.1 convert) |
| ⬜ | **Keyframe detection/analysis** | 🟠 High | 🟢 Low | detects keyframe data and returns analytical data of each keyframe |
| ⬜ | **HDR processing** | 🔴 Very High | 🟢 Low | detects HDR content, enables level changing or format conversion (PQ to HDR10), nits control |
| ⬜ | **Audio Spectrum analyzer** | 🟡 Medium | 🟢 Low | audio track spectrum analyzer for audio visualization |
| ⬜ | **Audio sync control (slipping, move forward/back)** | 🟠 High | 🟢 Low | slip/move audio tracks forward or backward in time |
| ⬜ | **Timecode** | 🟠 High | 🟡 Medium | reads timecode and has ability to change the timecode value |
| ⬜ | **Subtitles** | 🔴 Very High | 🟡 Medium | reads/writes subtitles into CEA-608/CEA-708, timing adjustments and editing |

---

## diagnostic — To do

| Status | Feature | Difficulty | Priority | Description |
|---|---|---|---|---|
| ✅ | **Diagnostic page** | 🟡 Medium | ✅ Done | Dedicated `/diagnostic` page with upload, progress polling, profiles, pass/warn/fail report |
| ✅ | **Diagnostic rules core** | 🟠 High | ✅ Done | Container, codec, pixel format, color depth, frame rate, VFR, A/V sync, audio checks |
| ✅ | **HDR / side data checks** | 🟠 High | ✅ Done | HDR10 / Dolby Vision / 360 / Stereo3D detection from `side_data_list` |
| ✅ | **Closed captions check** | 🟢 Low | ✅ Done | Detects embedded CEA-608/708 caption flags from ffprobe stream data |
| ✅ | **Forced subtitles check** | 🟢 Low | ✅ Done | Detects subtitle streams flagged as `forced` |
| ✅ | **Subtitle codec validation** | 🟢 Low | ✅ Done | Validates subtitle stream codecs like `mov_text`, `srt`, `webvtt`, warns on sensitive codecs |
| ✅ | **Metadata tag checks** | 🟡 Medium | ✅ Done | Checks audio/subtitle language tags, creation time, and timecode presence by profile |
| 🟡 | **Raw inspector polish** | 🟡 Medium | 🔴 High | Improve readability of raw stream / tags / chapters / side data in diagnostic UI |
| ⬜ | **NLE metadata comparison** | 🟠 High | 🔴 High | Compare DaVinci / Premiere / FCPX exports to spot metadata differences |
| ⬜ | **Waveform** | 🟢 Low | 🟡 Medium | Audio file → waveform image to support visual inspection in diagnostic flow |
| ⬜ | **Scene detection** | 🟡 Medium | 🟡 Medium | Detect cuts / scene changes to support deeper content inspection |

---

## UI / UX — To do

| Status | Feature | Difficulty | Priority | Description |
|---|---|---|---|---|
| ✅ | **Dropzone click-to-browse** | 🟢 Low | ✅ Done | Click area → native file picker |
| ✅ | **Dropzone append files** | 🟢 Low | ✅ Done | Drop adds to list, doesn't replace |
| ✅ | **Dropzone validation** | 🟢 Low | ✅ Done | `max_files`, `max_size_mb`, `accept` mime types |
| ✅ | **Dropzone list/grid toggle** | 🟢 Low | ✅ Done | Switch between row list and card grid |
| ⬜ | **Job progress bar UI** | 🟢 Low | 🔴 High | Visual progress tied to `media_write` jobs |
| ⬜ | **Video player** | 🟡 Medium | 🟡 Medium | In-app playback of uploaded / processed files |
| ✅ | **Dark mode** | 🟢 Low | ✅ Done | Toggle in navbar top-right, localStorage persistence |
| ✅ | **Side menu resize** | 🟡 Medium | ✅ Done | enables the side bar to be resized |

---

## Long term (Phase 2-3 — see PLAN_NODES_GSTREAMER.md)

| Feature | Difficulty | Priority | Description |
|---|---|---|---|
| **Node graph UI** | 🔴 Very High | 🟢 Low | Visual procedural pipeline (like Touch Designer) |
| **GStreamer backend** | 🔴 Very High | 🟢 Low | Replace FFmpeg jobs with GStreamer pipelines |
| **Live streaming** | 🔴 Very High | 🟢 Low | WebRTC input/output (like VDO.Ninja) |
| **Camera input** | 🔴 Very High | 🟢 Low | Capture from webcam / NDI / capture card |
| **Monitoring dashboard** | 🟠 High | 🟢 Low | CPU/GPU/bandwidth metrics per job (like Datadog) |
