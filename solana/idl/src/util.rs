use super::{idl_metadata::IdlMetadata, Idl};

pub fn load_idl_from_json(idl_path: &str) -> std::io::Result<Idl> {
    let file = std::fs::File::open(idl_path)?;
    let idl: Idl = serde_json::from_reader(file)?;
    Ok(idl)
}

pub fn update_program_id(idl_path: &str, program_id: &str) -> std::io::Result<()> {
    let mut idl: Idl = load_idl_from_json(idl_path)?;
    idl.metadata = IdlMetadata::new(program_id);
    idl.write_to_json(idl_path)?;
    Ok(())
}
