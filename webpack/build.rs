use static_files::NpmBuild;

fn main() {
    NpmBuild::new("web")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .target("web/dist/bundle")
        .change_detection()
        .to_resource_dir()
        .build()
        .unwrap();
}
