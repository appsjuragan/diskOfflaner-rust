fn main() {
    // Only compile resource on Windows
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/g1.ico");
        res.set("ProductName", "DiskOfflaner");
        res.set(
            "FileDescription",
            "Safe and simple disk management for Windows and Linux",
        );
        res.set("CompanyName", "Apps Juragan");
        res.set("LegalCopyright", "Copyright (c) 2024");

        if let Err(e) = res.compile() {
            eprintln!("Failed to compile resources: {e}");
            std::process::exit(1);
        }
    }
}
