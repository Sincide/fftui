use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    CpuH264,
    GpuH264,
}

impl Preset {
    pub const ALL: [Preset; 2] = [Preset::CpuH264, Preset::GpuH264];

    pub fn name(self) -> &'static str {
        match self {
            Preset::CpuH264 => "H.264 (CPU - libx264)",
            Preset::GpuH264 => "H.264 (GPU - VAAPI)",
        }
    }
}

pub struct FileItem {
    pub path: PathBuf,
    pub selected: bool,
    pub progress: Option<String>,
}

pub enum FocusArea {
    Files,
    Presets,
}

pub struct AppState {
    pub files: Vec<FileItem>,
    pub preset: Preset,
    pub focus: FocusArea,
    pub cursor: usize,
}

impl AppState {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self {
            files: files
                .into_iter()
                .map(|p| FileItem {
                    path: p,
                    selected: true,
                    progress: None,
                })
                .collect(),
            preset: Preset::CpuH264,
            focus: FocusArea::Files,
            cursor: 0,
        }
    }
}
