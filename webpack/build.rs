use static_files::NpmBuild;

fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("NODE_OPTIONS", "--openssl-legacy-provider");
    }
    NpmBuild::new("web")
        .install()?
        .run("build")?
        .target("web/dist/bundle")
        .change_detection()
        .to_resource_dir()
        .build()
}
