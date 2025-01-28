pub fn unless_unix<T: Fn() -> ()>(fallback: T) {
    let os = std::env::consts::OS;
    if os != "macos" && os != "linux" {
        fallback();
    }
}