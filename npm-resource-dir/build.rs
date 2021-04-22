use static_files::{npm_resource_dir, resource_dir};
use std::{env, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut rd = resource_dir("./static");
    rd.with_generated_filename(Path::new(&out_dir).join("generated_index.rs"))
        .with_generated_fn("index");
    rd.build().unwrap();

    npm_resource_dir("./static_packages")
        .unwrap()
        .build()
        .unwrap();
}
