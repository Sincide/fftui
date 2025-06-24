use std::fs;
use ffmpeg_tui::scan_files;

#[test]
fn finds_mkv_files() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("a.mkv"), b"test").unwrap();
    fs::write(tmp.path().join("b.txt"), b"no").unwrap();
    let files = scan_files(tmp.path());
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].file_name().unwrap(), "a.mkv");
}
