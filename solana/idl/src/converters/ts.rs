//! Converts a JSON IDL to TypeScript bindings.
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

fn capitalize_first_letter(s: &String) -> String {
    let mut char_iter = s.chars();
    match char_iter.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + char_iter.as_str(),
    }
}
fn decapitalize_first_letter(s: &String) -> String {
    let mut char_iter = s.chars();
    match char_iter.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().collect::<String>() + char_iter.as_str(),
    }
}

/// Trait to enable conversion of IDL components into TypeScript code.
pub trait TypeScriptConverter {
    fn to_typescript_string(&self) -> String;
}

impl TypeScriptConverter for Idl {
    fn to_typescript_string(&self) -> String {
        let ts_body_globs = vec![
            vec![
                String::from("import { PublicKey } from \"@solana/web3.js\""),
                String::from("import BN from \"bn.js\""),
                String::from("\n"),
                self.to_typescript_program_idl(),
                String::from("\n"),
            ],
            // TODO: Configs
            // TODO: Constants
            // TODO: Errors
            self.instructions
                .iter()
                .map(|i| i.to_typescript_string())
                .collect::<Vec<String>>(),
            self.accounts
                .iter()
                .map(|a| a.to_typescript_string())
                .collect::<Vec<String>>(),
            self.types
                .iter()
                .map(|t| t.to_typescript_string())
                .collect::<Vec<String>>(),
        ];
        let ts_body = ts_body_globs.into_iter().flatten().collect::<Vec<String>>();
        let res = ts_body.join("\n");
        res
    }
}

impl Idl {
    pub fn to_typescript_program_idl(&self) -> String {
        let formatted_name = self
            .name
            .split('-')
            .map(|x| capitalize_first_letter(&x.to_string()))
            .collect::<Vec<_>>()
            .concat();

        let tables = self
            .accounts
            .iter()
            .map(|account| format!("    {}: string", decapitalize_first_letter(&account.name)))
            .collect::<Vec<_>>()
            .join(",\n");
        let tables_string = if self.accounts.len() == 0 {
            None
        } else {
            Some(format!("\n  tables: {{\n{}\n  }}\n", tables))
        };

        let strings = vec![tables_string]
            .iter()
            .filter(|&x| x.is_some())
            .map(|x| x.as_deref().unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        format!("export type {}Program = {{{}}}", formatted_name, strings)
    }
}

impl TypeScriptConverter for IdlInstruction {
    fn to_typescript_string(&self) -> String {
        String::from("") // TODO
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
                            "   {}: {};",
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
                                        "   {}: {};",
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
            IdlType::PublicKey => "PublicKey".to_string(),
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

/// The trait to enable writing an IDL to TypeScript.
pub trait TypeScriptIdlWrite {
    /// Writes an IDL to a TypeScript `.ts` file.
    fn write_to_ts(&self, dir_path: &str) -> std::io::Result<()>;
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
