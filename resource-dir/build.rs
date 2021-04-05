use static_files::resource::resource_dir;

fn main() {
    resource_dir("./static").build().unwrap();
}
