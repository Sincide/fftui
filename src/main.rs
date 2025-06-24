use std::path::PathBuf;

use walkdir::WalkDir;

mod ui;
mod ffmpeg;
mod state;

use state::AppState;

fn scan_files() -> Vec<PathBuf> {
    WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|s| s == "mkv").unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let files = scan_files();
    let mut state = AppState::new(files);
    ui::run_app(&mut state)?;
    ffmpeg::convert_selected(&mut state.files, state.preset).await;
    Ok(())
}
