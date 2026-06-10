# Plan — Node Pipeline UI + GStreamer

## What Yukihiro wants

Node-based procedural workflow for video processing (like Touch Designer / GStreamer UI).
Web + Desktop from the same codebase (Dioxus). Live streaming via WebRTC (like VDO.Ninja).

```
[FileSource] ──→ [Decoder] ──→ [Scaler] ──→ [Encoder] ──→ [FileSink]
     ●───────────────●─────────────●────────────●───────────────●

Connecting nodes visually = defining the processing pipeline.
```

---

## Current state

**In place:**
- Dioxus fullstack (web + desktop, same codebase) ✓
- FFmpeg server-side jobs ✓ (`feat: add real media write ffmpeg jobs`)
- Dropzone UI with image/video preview ✓

**Missing:**
- Node graph UI
- GStreamer integration
- Live streaming / WebRTC

---

## FFmpeg vs GStreamer

| | FFmpeg | GStreamer |
|---|---|---|
| Current use | Transcode, thumbnails | — |
| Setup | Simple (CLI wrap) | Heavy (system libs) |
| "Transcode a file" | 1 command | ~30 lines pipeline |
| Node UI | ✗ not native | ✓ architecture = node graph |
| WebRTC / Live | ✗ | ✓ native |
| Rust bindings | `ffmpeg-next` | `gstreamer = "0.23"` (official) |

**Key insight:** GStreamer's architecture IS the node graph.
Each `Element` = 1 node. Each `Pad` = 1 connection.

```
Dioxus Node UI          GStreamer Pipeline
──────────────          ─────────────────
[FileSource]      →     filesrc
[H264 Decode]     →     avdec_h264
[Scale 720p]      →     videoscale
[x264 Encode]     →     x264enc
[MP4 Sink]        →     mp4mux + filesink
```

---

## Roadmap

### Phase 1 — Now (FFmpeg, already in place)
- Server-side transcode with FFmpeg ✓
- Upload + preview in Dioxus ✓
- Metadata extraction (duration, codec, resolution, bitrate)

### Phase 2 — Node UI (Dioxus, no GStreamer yet)
- Visual node graph in Dioxus
- Drag & connect nodes
- Hardcoded preset pipelines (FFmpeg under the hood)

```
┌──────────┐    ┌──────────┐    ┌──────────┐
│   File   │───▶│ Transcode│───▶│  Output  │
│  Source  │    │  H264    │    │   Sink   │
└──────────┘    └──────────┘    └──────────┘
```

### Phase 3 — GStreamer
- Replace FFmpeg jobs with GStreamer pipelines
- Node UI maps directly to GStreamer elements
- Live streaming / WebRTC

---

## Rule

> FFmpeg = server jobs now.  
> GStreamer = when the node UI exists.  
> No migration until the UI is there.
