use std::{fs::File, io::Write, path::Path, process::Stdio};

use tokio::{io::AsyncReadExt, process::Command};

use crate::state::{FileItem, Preset};

pub async fn convert_file(item: &mut FileItem, preset: Preset) -> anyhow::Result<()> {
    let input = item.path.clone();
    let output = input
        .with_file_name(format!("{}[H264].mkv", input.file_stem().unwrap().to_string_lossy()));

    std::fs::create_dir_all("logs")?;
    let log_path = Path::new("logs").join(
        input
            .file_name()
            .map(|n| n.to_string_lossy().into_owned() + ".log")
            .unwrap_or_else(|| "conversion.log".to_string()),
    );
    let mut log_file = File::create(log_path)?;

    // Stub command. Replace with actual ffmpeg invocation.
    let mode = match preset {
        Preset::CpuH264 => "cpu",
        Preset::GpuH264 => "gpu",
    };
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!(
        "echo Converting {} to {} with {}; sleep 1; echo Done",
        input.display(),
        output.display(),
        mode
    ));

    let mut child = cmd.stdout(Stdio::piped()).spawn()?;

    let mut stdout = child.stdout.take().unwrap();
    let mut buf = Vec::new();
    stdout.read_to_end(&mut buf).await?;
    log_file.write_all(&buf)?;
    item.progress = Some("done".into());

    let status = child.wait().await?;
    if !status.success() {
        anyhow::bail!("command failed");
    }
    Ok(())
}

pub async fn convert_selected(files: &mut [FileItem], preset: Preset) {
    for item in files.iter_mut().filter(|f| f.selected) {
        let _ = convert_file(item, preset).await;
    }
}
