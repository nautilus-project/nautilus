pub fn update_program_id(
    idl_path: &str,
    program_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(idl_path)?;
    let mut idl: crate::Idl = serde_json::from_reader(file)?;
    idl.metadata = crate::IdlMetadata::new(program_id);
    idl.write();
    Ok(())
}
