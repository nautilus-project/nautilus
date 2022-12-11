use crate::core::execute_command;

// TODO: Write this function so that it can write the program's
//      entrypoint & processor as defined in the README
//
pub fn execute_nautilus_build() {
    execute_command("cargo build-bpf");
}