use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    idl_instruction::IdlInstruction,
    idl_type::IdlType,
    idl_type_def::{IdlTypeDef, IdlTypeDefType, IdlTypeEnumFields},
    Idl,
};

pub trait TypeScriptIdlWrite {
    fn write_to_ts(&self, dir_path: &str) -> std::io::Result<()>;
}

pub trait TypeScriptConverter {
    fn to_typescript_string(&self) -> String;
}

impl TypeScriptIdlWrite for Idl {
    fn write_to_ts(&self, dir_path: &str) -> std::io::Result<()> {
        if dir_path != "." {
            fs::create_dir_all(dir_path)?;
        }

        let ts_idl_path = Path::join(Path::new(dir_path), &format!("{}.ts", &self.name));

        let mut file = File::create(ts_idl_path)?;
        let typescript_string = self.to_typescript_string();
        file.write_all(typescript_string.as_bytes())?;

        Ok(())
    }
}

impl TypeScriptConverter for Idl {
    fn to_typescript_string(&self) -> String {
        // TODO: Lay down schema and add instructions/configs:
        let mut all_types = self.accounts.clone();
        all_types.extend(self.types.clone());
        let all_types_strings: Vec<String> =
            all_types.iter().map(|t| t.to_typescript_string()).collect();
        let res = all_types_strings.join("\n");
        res
    }
}

impl TypeScriptConverter for IdlInstruction {
    fn to_typescript_string(&self) -> String {
        todo!()
    }
}

impl TypeScriptConverter for IdlTypeDef {
    fn to_typescript_string(&self) -> String {
        match &self.idl_type {
            IdlTypeDefType::Struct { fields } => {
                let fields_str = fields
                    .iter()
                    .map(|field| {
                        format!(
                            "{}: {};",
                            field.name,
                            field.field_data_type.to_typescript_string()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                format!("type {} = {{\n{}\n}};", self.name, fields_str)
            }
            IdlTypeDefType::Enum { variants } => {
                let variants_str = variants
                    .iter()
                    .map(|variant| {
                        let fields_str = match &variant.fields {
                            Some(IdlTypeEnumFields::Named(fields)) => fields
                                .iter()
                                .map(|field| {
                                    format!(
                                        "{}: {};",
                                        field.name,
                                        field.field_data_type.to_typescript_string()
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join(", "),
                            None => String::new(),
                        };

                        format!("{} = {{ {} }}", variant.name, fields_str)
                    })
                    .collect::<Vec<String>>()
                    .join(" | ");

                format!("type {} = {};", self.name, variants_str)
            }
        }
    }
}

impl TypeScriptConverter for IdlType {
    fn to_typescript_string(&self) -> String {
        match self {
            IdlType::Array(idl_type, size) => {
                format!("[{}; {}]", idl_type.to_typescript_string(), size)
            }
            IdlType::Bool => "boolean".to_string(),
            IdlType::Bytes => "Uint8Array".to_string(),
            IdlType::Defined(name) => name.clone(),
            IdlType::I128 | IdlType::I16 | IdlType::I32 | IdlType::I64 | IdlType::I8 => {
                "number".to_string()
            }
            IdlType::Option(idl_type) => format!("{} | null", idl_type.to_typescript_string()),
            IdlType::Tuple(idl_types) => format!(
                "[{}]",
                idl_types
                    .iter()
                    .map(|idl_type| idl_type.to_typescript_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            IdlType::PublicKey => "Uint8Array".to_string(),
            IdlType::String => "string".to_string(),
            IdlType::U128 | IdlType::U16 | IdlType::U32 | IdlType::U64 | IdlType::U8 => {
                "number".to_string()
            }
            IdlType::Vec(idl_type) => format!("{}[]", idl_type.to_typescript_string()),
            IdlType::HashMap(key_type, value_type) => format!(
                "Map<{}, {}>",
                key_type.to_typescript_string(),
                value_type.to_typescript_string()
            ),
            IdlType::BTreeMap(key_type, value_type) => format!(
                "Map<{}, {}>",
                key_type.to_typescript_string(),
                value_type.to_typescript_string()
            ),
            IdlType::HashSet(idl_type) => format!("Set<{}>", idl_type.to_typescript_string()),
            IdlType::BTreeSet(idl_type) => format!("Set<{}>", idl_type.to_typescript_string()),
        }
    }
}
