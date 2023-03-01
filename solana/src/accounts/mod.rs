//
//
// ----------------------------------------------------------------
//
//      Traits used to abstract CRUD operations for PDAs
//
// ----------------------------------------------------------------
//
//
pub mod auth;
pub mod create;
pub mod data;
pub mod delete;
pub mod update;

pub use auth::*;
pub use create::*;
pub use data::*;
pub use delete::*;
pub use update::*;
