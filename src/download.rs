use std::path::{Path, PathBuf};

use tokio::process::Command;

use crate::result::{EchoReport, EchoResult};

/// Download audio from a YouTube URL as MP3 using yt-dlp.
/// Returns the path to the downloaded file.
pub async fn download_mp3(url: &str, output_dir: &Path) -> EchoResult<PathBuf> {
    let output_template = output_dir.join("%(title)s.%(ext)s");

    let output = Command::new("yt-dlp")
        .args([
            "-x",
            "--audio-format",
            "mp3",
            "--no-playlist",
            "--no-overwrites",
            "--print",
            "after_move:filepath",
            "-o",
            output_template.to_str().unwrap_or("%(title)s.%(ext)s"),
            url,
        ])
        .output()
        .await
        .map_err(|e| EchoReport::DownloadError(format!("Failed to run yt-dlp: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EchoReport::DownloadError(format!(
            "yt-dlp failed: {}",
            stderr.trim()
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let file_path = stdout.trim().lines().last().unwrap_or("").trim();

    if file_path.is_empty() {
        return Err(EchoReport::DownloadError(
            "yt-dlp did not return a file path".into(),
        ));
    }

    let path = PathBuf::from(file_path);
    if !path.exists() {
        return Err(EchoReport::DownloadError(format!(
            "Downloaded file not found: {}",
            file_path
        )));
    }

    Ok(path)
}
