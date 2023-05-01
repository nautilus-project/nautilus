//! Converts a JSON IDL to Python bindings.
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{
    idl_instruction::IdlInstruction,
    idl_type::IdlType,
    idl_type_def::{IdlTypeDef, IdlTypeDefType},
    Idl,
};

/// Trait to enable conversion of IDL components into Python code.
pub trait PythonConverter {
    fn to_python_string(&self) -> String;
}

impl PythonConverter for Idl {
    fn to_python_string(&self) -> String {
        let ts_body_globs = vec![
            // TODO: Imports
            // TODO: Configs
            // TODO: Constants
            // TODO: Errors
            self.instructions
                .iter()
                .map(|i| i.to_python_string())
                .collect::<Vec<String>>(),
            self.accounts
                .iter()
                .map(|a| a.to_python_string())
                .collect::<Vec<String>>(),
            self.types
                .iter()
                .map(|t| t.to_python_string())
                .collect::<Vec<String>>(),
        ];
        let ts_body = ts_body_globs.into_iter().flatten().collect::<Vec<String>>();
        let res = ts_body.join("\n");
        res
    }
}

impl PythonConverter for IdlInstruction {
    fn to_python_string(&self) -> String {
        String::from("") // TODO
    }
}

impl PythonConverter for IdlTypeDef {
    fn to_python_string(&self) -> String {
        match &self.idl_type {
            IdlTypeDefType::Struct { fields } => {
                let fields_str = fields
                    .iter()
                    .map(|field| {
                        format!(
                            "   {}: {}",
                            field.name,
                            field.field_data_type.to_python_string()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                format!("class {}:\n{}\n", self.name, fields_str)
            }
            IdlTypeDefType::Enum { .. } => String::new(), // TODO: Python enums not supported yet
        }
    }
}

impl PythonConverter for IdlType {
    fn to_python_string(&self) -> String {
        match self {
            IdlType::Array(inner_type, size) => {
                format!("[{}; {}]", inner_type.to_python_string(), size)
            }
            IdlType::Bool => "bool".to_string(),
            IdlType::Bytes => "bytes".to_string(),
            IdlType::Defined(name) => name.clone(),
            IdlType::I128 => "int".to_string(),
            IdlType::I16 => "int".to_string(),
            IdlType::I32 => "int".to_string(),
            IdlType::I64 => "int".to_string(),
            IdlType::I8 => "int".to_string(),
            IdlType::Option(inner_type) => format!("Optional[{}]", inner_type.to_python_string()),
            IdlType::Tuple(types) => format!(
                "Tuple[{}]",
                types
                    .iter()
                    .map(|t| t.to_python_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            IdlType::PublicKey => "PublicKey".to_string(),
            IdlType::String => "str".to_string(),
            IdlType::U128 => "int".to_string(),
            IdlType::U16 => "int".to_string(),
            IdlType::U32 => "int".to_string(),
            IdlType::U64 => "int".to_string(),
            IdlType::U8 => "int".to_string(),
            IdlType::Vec(inner_type) => format!("List[{}]", inner_type.to_python_string()),
            IdlType::HashMap(key_type, value_type) => format!(
                "Dict[{}, {}]",
                key_type.to_python_string(),
                value_type.to_python_string()
            ),
            IdlType::BTreeMap(key_type, value_type) => format!(
                "Dict[{}, {}]",
                key_type.to_python_string(),
                value_type.to_python_string()
            ),
            IdlType::HashSet(value_type) => format!("Set[{}]", value_type.to_python_string()),
            IdlType::BTreeSet(value_type) => format!("Set[{}]", value_type.to_python_string()),
        }
    }
}

/// The trait to enable writing an IDL to Python.
pub trait PythonIdlWrite {
    fn write_to_py(&self, dir_path: &str) -> std::io::Result<()>;
}

impl PythonIdlWrite for Idl {
    fn write_to_py(&self, dir_path: &str) -> std::io::Result<()> {
        if dir_path != "." {
            fs::create_dir_all(dir_path)?;
        }
        let py_idl_path = Path::join(Path::new(dir_path), &format!("{}.py", &self.name));
        let mut file = File::create(py_idl_path)?;
        let python_string = self.to_python_string();
        file.write_all(python_string.as_bytes())?;
        Ok(())
    }
}
