pub fn parse_cargo_toml(file_path: &str) -> Option<(String, String)> {
    let file_contents = std::fs::read_to_string(file_path).ok()?;
    let toml: toml::Value = file_contents.parse().ok()?;
    let package = toml.get("package")?.as_table()?;
    let name = package.get("name")?.as_str()?.to_string();
    let version = package.get("version")?.as_str()?.to_string();

    Some((name, version))
}
