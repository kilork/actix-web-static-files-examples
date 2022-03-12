use static_files::NpmBuild;

fn main() -> std::io::Result<()> {
    NpmBuild::new("web")
        .executable("yarn")
        .install()?
        .run("build")?
        .target("web/dist/bundle")
        .change_detection()
        .to_resource_dir()
        .build()
}
