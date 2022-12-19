use convert_case::{Case, Casing};
use regex::Regex;
use std::io::{
    BufReader,
    prelude::*,
};

const LIB_RS: &'static str = "./src/lib.rs";
const NAUTILUS_MOD_RS: &'static str = "./src/nautilus_mod.rs";
const NAUTILUS_MOD_STRING: &'static str = "\nmod nautilus_mod;";

fn find_program_name() -> std::io::Result<String> {
    let cargo_toml = std::fs::read_to_string("./Cargo.toml")?;
    let name_re = Regex::new("name\\s+=\\s+\".*\"").expect("Failed to parse regex");
    let quote_re = Regex::new("\".*\"").expect("Failed to parse regex");
    let found_name = name_re.find(&cargo_toml).map(|m| m.as_str()).unwrap_or("");
    Ok(
        quote_re.find(found_name)
            .map(|m| m.as_str())
            .unwrap_or("")
            .replace("\"", "")
            .replace("-", "_")
            .to_case(Case::Pascal)
    )
}

fn find_crud_annotations() -> std::io::Result<Vec<String>> {
    let struct_re = Regex::new("derive(Nautilus.*pub struct .*\\s+{").expect("Failed to parse regex");
    for file in fin_all_rust_files() {
        let found_name = struct_re.find(&file).map(|m| m.as_str()).unwrap_or("");
    };
    Ok(vec![
        "CreatePerson".to_string(),
        "DeletePerson".to_string(),
        "UpdatePerson".to_string(),
    ])
}

fn create_entrypoint() -> std::io::Result<String> {
    let program_name = find_program_name()?;
    let mut file_contents = "
use borsh::{{ BorshDeserialize, BorshSerialize }};
use solana_program::{{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey, 
}};

entrypoint!(process_instruction);
    ".to_owned();

    let mut enum_contents = format!("
#[derive(BorshDeserialize, BorshSerialize)]
pub enum {}Instruction {{
    ", program_name).to_owned();

    let mut processor_contents = format!("
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {{

    let instruction = {}Instruction::try_from_slice(&instruction_data)?;
    
    match instruction {{
    ", program_name).to_owned();

    find_crud_annotations()?.iter().for_each(|c| {
        enum_contents += &format!("
    {},", &c);
        processor_contents += &format!("
        {}Instruction::{} => {{
            msg!(\"{}Instruction: {}\");
            return Ok(())
        }},
        ", program_name, c, program_name, c);
    });

    enum_contents += "
}
    ";
    processor_contents += "
    }
    ";
    file_contents += &enum_contents;
    file_contents += &processor_contents;
    file_contents += "
}
    ";
    Ok(file_contents)
}

pub fn build_program_entrypoint() -> std::io::Result<()> {
    let mut file = std::fs::File::create(NAUTILUS_MOD_RS)?;
    file.write_all(create_entrypoint()?.as_bytes())?;
    let mut lib_rs = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LIB_RS)
        .unwrap();
    writeln!(lib_rs, "{}", NAUTILUS_MOD_STRING)?;
    Ok(())
}

pub fn strip_program_entrypoint() -> std::io::Result<()> {
    let lib_rs = std::fs::OpenOptions::new()
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

pub fn try_read_program_so() -> Option<String> {
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