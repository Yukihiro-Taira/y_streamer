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

| Status | Feature | Description |
|---|---|---|
| ✅ | **Thumbnail generation** | 3 frames at 10%/50%/90% → base64 in response, shown in UI |
| ✅ | **Subtitle extract** | Extract all streams → SRT, browser download button per track |
| ⬜ | **Waveform** | Audio file → waveform image (ffmpeg `showwavespic`) |
| ⬜ | **Scene detection** | Detect cuts / scene changes (ffmpeg `scdet` filter) |

---

## media_write — To do

| Status | Feature | Description |
|---|---|---|
| ⬜ | **Job progress** | Real-time FFmpeg progress (%, ETA) via SSE or polling |
| ⬜ | **Download result** | Download processed file after job completes |
| ⬜ | **Job history** | List past jobs + status (pending / running / done / failed) |
| ⬜ | **Format conversion** | mp4→webm, mov→mp4, mkv→mp4 |
| ⬜ | **Audio extract** | Strip video → mp3 / wav / aac |
| ⬜ | **Clip trim** | Cut start/end timestamps (`-ss` `-to`) |
| ⬜ | **Resize** | Scale to 720p / 1080p / custom |
| ⬜ | **Bitrate control** | Set target bitrate / CRF |
| ⬜ | **Batch jobs** | Queue multiple files, process in parallel |
| ⬜ | **Watermark** | Overlay image/text on video |
| ⬜ | **Audio Broadcast Convert** | converts audio tracks to Broadcast delivery specs (stereo to mono)(atmos, 5.1 convert) |
| ⬜ | **Keyframe detection/analysis** | detects keyframe data and returns analytical data of each keyframe |
| ⬜ | **HDR processing** | detects HDR content, enables level changing or format conversion (PQ to HDR10), nits control |
| ⬜ | **Audio Spectrum analyzer** | audio track spectrum analyzer for audio visualization |
| ⬜ | **Audio sync control (slipping, move forward/back)** | slip/move audio tracks forward or backward in time |
| ⬜ | **Timecode** | reads timecode and has ability to change the timecode value |
| ⬜ | **Subtitles** | reads/writes subtitles into CEA-608/CEA-708, timing adjustments and editing |

---

## UI / UX — To do

| Status | Feature | Description |
|---|---|---|
| ⬜ | **Dropzone click-to-browse** | Click area → native file picker |
| ⬜ | **Dropzone append files** | Drop adds to list, doesn't replace |
| ⬜ | **Dropzone validation** | `max_files`, `max_size_mb`, `accept` mime types |
| ⬜ | **Dropzone list/grid toggle** | Switch between row list and card grid |
| ⬜ | **Job progress bar UI** | Visual progress tied to `media_write` jobs |
| ⬜ | **Video player** | In-app playback of uploaded / processed files |
| ✅ | **Dark mode** | Toggle in navbar top-right, localStorage persistence |
| ⬜ | **Side menu resize** | enables the side bar to be resized |

---

## Long term (Phase 2-3 — see PLAN_NODES_GSTREAMER.md)

| Feature | Description |
|---|---|
| **Node graph UI** | Visual procedural pipeline (like Touch Designer) |
| **GStreamer backend** | Replace FFmpeg jobs with GStreamer pipelines |
| **Live streaming** | WebRTC input/output (like VDO.Ninja) |
| **Camera input** | Capture from webcam / NDI / capture card |
| **Monitoring dashboard** | CPU/GPU/bandwidth metrics per job (like Datadog) |
