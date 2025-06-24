use std::path::PathBuf;

use crate::scanner::scan_files;

mod ui;
mod ffmpeg;
mod state;
mod config;
mod scanner;

use state::AppState;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg_path = PathBuf::from("config.toml");
    let cfg = Config::load(&cfg_path);
    let preset = cfg.preset().unwrap_or(state::Preset::CpuH264);

    let files = scan_files(PathBuf::from(".").as_path());
    let mut state = AppState::new(files, preset);

    if ui::run_selection(&mut state)? {
        ui::run_conversion(&mut state).await?;
    }

    let mut cfg = cfg;
    cfg.set_preset(state.preset);
    cfg.save(&cfg_path);
    Ok(())
}
