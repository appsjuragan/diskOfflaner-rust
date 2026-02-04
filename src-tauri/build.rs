fn main() {
    tauri_build::build();

    // Only compile resource on Windows if needed (Tauri usually handles this via bundle config)
    /*
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icons/icon.ico");
        res.set("ProductName", "DiskOfflaner");
        // ...
        // res.compile().unwrap();
    }
    */
}
