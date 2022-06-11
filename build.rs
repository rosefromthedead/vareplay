fn main() {
    println!("cargo:rerun-if-changed=backend/");
    let libspa = pkg_config::Config::new()
        .probe("libspa-0.2")
        .unwrap();
    let mut build = cc::Build::new();
    build
        .includes(&libspa.include_paths)
        .file("backend/backend.c")
        .file("backend/pw-utils.c");
    build.compile("backend");
    let libva = pkg_config::Config::new()
        .probe("libva")
        .unwrap();
    let _libva_drm = pkg_config::Config::new()
        .probe("libva-drm")
        .unwrap();
}
