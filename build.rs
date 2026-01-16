// build.rs
fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winres::WindowsResource::new();
        // ここにアイコンファイル（.ico）のパスを指定
        res.set_icon("/icon.ico");
        res.compile().unwrap();
    }
}
