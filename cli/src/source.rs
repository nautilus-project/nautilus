use std::fs::{
    File,
    OpenOptions,
};
use std::io::BufReader;
use std::io::prelude::*;

const LIB_RS: &'static str = "./src/lib.rs";
const NAUTILUS_MOD_RS: &'static str = "./src/nautilus_mod.rs";
const NAUTILUS_MOD_STRING: &'static str = "\nmod nautilus_mod;";

fn find_crud_annotations() -> Vec<String> {
    vec![]
}

pub fn build_program_entrypoint() -> std::io::Result<()> {
    let instructions = find_crud_annotations();
    let mut file = File::create(NAUTILUS_MOD_RS)?;
    file.write_all(b"Hello, world!")?;
    let mut lib_rs = OpenOptions::new()
        .write(true)
        .append(true)
        .open(LIB_RS)
        .unwrap();
    writeln!(lib_rs, "{}", NAUTILUS_MOD_STRING)?;
    Ok(())
}

pub fn strip_program_entrypoint() -> std::io::Result<()> {
    let lib_rs = OpenOptions::new()
        .read(true)
        .write(true)
        .open(LIB_RS)?;
    let mut lines = BufReader::new(lib_rs).lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>().join("\n");

    NAUTILUS_MOD_STRING.chars().for_each(|_| { lines.pop(); });
    std::fs::write(LIB_RS, lines).expect("Can't write");
    std::fs::remove_file(NAUTILUS_MOD_RS)?;
    Ok(())
}

pub fn try_read_program_name() -> Option<String> {
    for element in std::path::Path::new(r"./target/deploy/").read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "so" {
                return Some(path.display().to_string())
            }
        }
    }
    None
}