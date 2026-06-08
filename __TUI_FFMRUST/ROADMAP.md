# ffmrust — Roadmap & Architecture

## Vision

TUI-driven FFmpeg pipeline in Rust. Client can monitor encode jobs live (queue, progress, stats), cancel/retry, and drop files directly into terminal.

---

## Phases

```
Phase 1 (current)        Phase 2              Phase 3
─────────────────────    ──────────────────   ──────────────────
TUI demo (simulated) →   FFmpeg binding    →  Own abstraction
  - queue UI              - ffmpeg-next        - JobRunner trait
  - drag/drop             - real encode        - async pipeline
  - progress sim          - real progress      - error handling
  - celebration           - real metadata      - cancel/retry
```

---

## Phase 2 — FFmpeg binding

Use [`ffmpeg-next`](https://crates.io/crates/ffmpeg-next) as the real encode backend.

**Tradeoffs:**
- unsafe-heavy (wraps libffmpeg C bindings)
- API not idiomatic Rust
- hard to unit-test directly

→ wrap it behind an abstraction layer immediately (see Phase 3).

---

## Phase 3 — Own abstraction

```rust
trait Encoder {
    async fn encode(&self, job: Job) -> Result<EncoderOutput>;
    fn progress(&self) -> impl Stream<Item = f32>;
}
```

Benefits:
- swap backend later (ffmpeg-next → subprocess → custom)
- testable with fake impl (what the demo already is)
- clean cancel/retry surface

---

## Async architecture

`tokio` + `mpsc` channels for progress updates into TUI:

```
tokio::spawn(encode_job)
        │
        └─ mpsc::Sender<Progress>
                │
                └─ App.update() ← TUI tick loop
                        │
                        └─ redraw frame
```

Cancel via `tokio_util::CancellationToken`.

---

## Implementation order

1. `JobRunner` trait (async) — fake impl already exists as demo
2. Plug `ffmpeg-next` as real `JobRunner` impl
3. Progress via `Stream<Item = Progress>`
4. Cancel + retry logic
5. Real file drop → parse metadata (duration, codec, resolution) on drop

---

## Notes

- TUI demo = free test harness for the real pipeline
- `App` + tick loop already async-friendly — just feed real progress via channel
- Keep fake impl for dev/demo mode (`--demo` flag later)
