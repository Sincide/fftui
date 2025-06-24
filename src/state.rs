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

    pub fn key(self) -> &'static str {
        match self {
            Preset::CpuH264 => "cpu",
            Preset::GpuH264 => "gpu",
        }
    }

    pub fn from_key(key: &str) -> Option<Preset> {
        match key {
            "cpu" => Some(Preset::CpuH264),
            "gpu" => Some(Preset::GpuH264),
            _ => None,
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
    pub fn new(files: Vec<PathBuf>, preset: Preset) -> Self {
        Self {
            files: files
                .into_iter()
                .map(|p| FileItem {
                    path: p,
                    selected: true,
                    progress: None,
                })
                .collect(),
            preset,
            focus: FocusArea::Files,
            cursor: 0,
        }
    }

    pub fn all_done(&self) -> bool {
        self.files
            .iter()
            .filter(|f| f.selected)
            .all(|f| matches!(f.progress.as_deref(), Some("done") | Some("failed")))
    }
}
