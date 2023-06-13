use html_minifier::HTMLMinifierHelper;
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

fn main() {
    minify_templates()
}
fn minify_templates() {
    if cfg!(debug_assertions) {
        println!("cargo:rustc-env=TEMPLATES_ROOT_DIR=./");
        return;
    }

    let repo_dir = Path::new("..").canonicalize().unwrap();
    let src_templates_dir = repo_dir.join("templates");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("Rust OUT_DIR not found"));
    let dst_templates_dir = Path::new(&out_dir).join("templates");
    println!(
        "cargo:rustc-env=TEMPLATES_ROOT_DIR=./{}",
        out_dir.strip_prefix(&repo_dir).unwrap().to_string_lossy()
    );

    // NIT: Might be nice to have a flag/env-var to toggle these logs. Not sure if there's an
    // idiomatic way to achieve this.
    //
    // println!(
    //     r#"cargo:warning="minimizing templates. templates_dir: {}, output_dir: {}"#,
    //     src_templates_dir.to_string_lossy(),
    //     dst_templates_dir.to_string_lossy(),
    // );

    let paths = fs::read_dir(&src_templates_dir)
        .expect("unable to list template directory")
        .map(|entry_res| entry_res.expect("unable to list file in template directory"))
        .filter(|entry| {
            let meta = entry.metadata().expect("unable to list template metadata");
            meta.is_file()
        })
        .map(|entry| entry.path());
    for src_path in paths {
        println!("cargo:rerun-if-changed={}", src_path.to_string_lossy());

        let rel_path = src_path.strip_prefix(&src_templates_dir).unwrap();
        let dst_path = dst_templates_dir.join(&rel_path);
        let dst_path_parent = dst_path
            .parent()
            .expect("parent unexpectedly did not exist in Path");

        // let rel_dst_path = dst_path.strip_prefix(&repo_dir).unwrap();
        // println!(
        //     r#"cargo:warning="minifying template. src:{}, dst_path:{}"#,
        //     rel_path
        //         .file_name()
        //         .expect("template src_path missing file_name")
        //         .to_string_lossy(),
        //     rel_dst_path.to_string_lossy(),
        // );

        fs::create_dir_all(dst_path_parent).expect("failed to create parent dir to template dst");
        let mut input_file = File::open(src_path).unwrap();
        let mut output_file = File::create(dst_path).unwrap();
        let mut buffer = [0u8; 256];
        let mut html_minifier_helper = HTMLMinifierHelper::new();
        loop {
            let c = input_file.read(&mut buffer).unwrap();
            if c == 0 {
                break;
            }
            html_minifier_helper
                .digest(&buffer[..c], &mut output_file)
                .unwrap();
        }
    }
}
