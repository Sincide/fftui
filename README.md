# ffmpeg-tui

`ffmpeg-tui` is a terminal user interface for running batch video conversions using FFmpeg.

This prototype scans the working directory for `.mkv` files and lets you select files and presets before starting the conversion process.

Run with:

```bash
cargo run
```

Conversion is currently stubbed with `echo` commands. Logs are written to the `logs/` directory.

