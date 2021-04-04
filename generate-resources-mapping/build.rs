use std::{env, path::Path};

use static_files::resource::generate_resources_mapping;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_filename = Path::new(&out_dir).join("generated.rs");
    generate_resources_mapping("./static", None, generated_filename).unwrap();
}
