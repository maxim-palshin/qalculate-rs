fn main() {
    println!("cargo:rerun-if-changed=wrapper/qalculator.cpp");
    println!("cargo:rerun-if-changed=wrapper/qalculator.h");

    if let Err(e) = pkg_config::probe_library("libqalculate") {
        eprintln!("Failed to find libqalculate: {}", e);
        eprintln!("Please install libqalculate development package:");
        eprintln!("  Ubuntu/Debian: sudo apt install libqalculate-dev");
        eprintln!("  Fedora: sudo dnf install libqalculate-devel");
        eprintln!("  Arch: sudo pacman -S libqalculate");
        std::process::exit(1);
    }

    cc::Build::new()
        .cpp(true)
        .file("wrapper/qalculator.cpp")
        .flag("-std=c++23")
        .compile("qalculator");

    println!("cargo:rustc-link-lib=qalculate");
    println!("cargo:rustc-link-lib=stdc++");
}
