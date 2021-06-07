pub fn display_path(path: &str) -> String {
    let path = path.replace("//", "/");
    let path = path.replace("\\\\", "/");
    path.replace("\\", "/")
}
