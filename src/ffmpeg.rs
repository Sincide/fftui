use std::{fs::File, io::Write, path::Path, process::Stdio};

use tokio::{io::{AsyncBufReadExt, BufReader}, process::Command};

use crate::state::{FileItem, Preset};

pub async fn convert_file(item: &mut FileItem, preset: Preset) -> anyhow::Result<()> {
    let input = item.path.clone();
    let output = input.with_file_name(format!("{}[H264].mkv", input.file_stem().unwrap().to_string_lossy()));

    std::fs::create_dir_all("logs")?;
    let log_path = Path::new("logs").join(
        input
            .file_name()
            .map(|n| n.to_string_lossy().into_owned() + ".log")
            .unwrap_or_else(|| "conversion.log".to_string()),
    );
    let mut log_file = File::create(log_path)?;

    let mut cmd = Command::new("ffmpeg");
    match preset {
        Preset::CpuH264 => {
            cmd.args(["-y", "-i"]).arg(&input).args(["-c:v", "libx264", output.to_str().unwrap()]);
        }
        Preset::GpuH264 => {
            cmd.args(["-y", "-hwaccel", "vaapi", "-i"]).arg(&input).args(["-c:v", "h264_vaapi", output.to_str().unwrap()]);
        }
    }
    cmd.arg("-progress").arg("pipe:1").stdout(Stdio::piped()).stderr(Stdio::null());

    let mut child = cmd.spawn()?;
    let mut reader = BufReader::new(child.stdout.take().unwrap()).lines();
    while let Some(line) = reader.next_line().await? {
        log_file.write_all(line.as_bytes())?;
        log_file.write_all(b"\n")?;
        item.progress = Some(line.clone());
    }
    let status = child.wait().await?;
    if status.success() {
        item.progress = Some("done".into());
    } else {
        item.progress = Some("failed".into());
    }
    Ok(())
}

pub async fn convert_selected(files: &mut [FileItem], preset: Preset) {
    for item in files.iter_mut().filter(|f| f.selected) {
        let _ = convert_file(item, preset).await;
    }
}
