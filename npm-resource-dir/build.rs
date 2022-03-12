use static_files::npm_resource_dir;

fn main() -> std::io::Result<()> {
    npm_resource_dir("./static_packages")?.build()
}
