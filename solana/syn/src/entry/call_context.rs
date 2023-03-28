use syn::Ident;

use crate::object::NautilusObject;

#[derive(Debug)]
pub enum CallContext {
    Nautilus(NautilusObject),
    Arg(Ident),
}
