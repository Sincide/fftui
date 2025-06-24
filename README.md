# ffmpeg-tui

`ffmpeg-tui` is a terminal user interface for running batch video conversions using FFmpeg.

The tool scans the working directory for `.mkv` files and lets you select which ones to convert.  Conversion progress is displayed live inside the terminal and logs are written to the `logs/` directory.  The last used preset is stored in `config.toml`.

Run with:

```bash
cargo run
```

FFmpeg must be available in your `PATH`. Logs for each input file are written to the `logs/` directory.

