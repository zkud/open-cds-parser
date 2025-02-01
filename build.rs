extern crate lalrpop;

use std::io::Write;
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR")
        .expect("Failed to get OUT_DIR environment variable");
    let dest_dir_path = Path::new(&out_dir)
        .join("parser/single_module/");
    let dest_path = dest_dir_path.join("cds.lalrpop");
    let src_dir = Path::new("src");

    create_dest_dir(&dest_dir_path);
    create_concatenated_file(&dest_path, &src_dir);
    lalrpop::Configuration::new()
        .process_file(&dest_path).unwrap();
    print_completion_message(&dest_path);
}

fn create_dest_dir(dest_dir_path: &Path) {
    fs::create_dir_all(&dest_dir_path).expect("Failed to create parser/single_module directory");
}

fn create_concatenated_file(dest_path: &Path, src_dir: &Path) {
    let mut concatenated_file =
        fs::File::create(dest_path).expect("Failed to create cds.lalrpop file");

    // Process the header.lalrpop file first
    let header_path = src_dir
        .join("./parser/single_module/header.lalrpop");
    if header_path.exists() {
        let header_contents = read_file_contents(&header_path);
        write_to_concatenated_file(&mut concatenated_file, &header_contents);
    }

    // Recursive search for .lalrpop files
    visit_dirs(src_dir, &mut |path| {
        if is_lalrpop_file(&path) {
            let contents = read_file_contents(&path);
            write_to_concatenated_file(&mut concatenated_file, &contents);
        }
    });
}

fn is_lalrpop_file(path: &Path) -> bool {
    path.extension().unwrap_or_default() == "lalrpop"
        && path.file_name().unwrap() != "header.lalrpop"
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb);
            } else {
                cb(&path);
            }
        }
    }
}

fn read_file_contents(path: &Path) -> String {
    fs::read_to_string(path).expect("Failed to read .lalrpop file")
}

fn write_to_concatenated_file(concatenated_file: &mut fs::File, contents: &str) {
    write!(concatenated_file, "{}\n", contents).expect("Failed to write to cds.lalrpop file");
}

fn print_completion_message(dest_path: &Path) {
    println!("cargo:rerun-if-changed=src/**/*.lalrpop");
    println!("Concatenated .lalrpop files to {:?}", dest_path);
}
