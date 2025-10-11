use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;

fn main() {
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = std::env::var("OUT_DIR").expect("Build scripts should have a target OUT_DIR var");

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();

    paths_to_copy.push("res/");
    copy_items(&paths_to_copy, out_dir, &copy_options).expect("should be able to copy");
}
