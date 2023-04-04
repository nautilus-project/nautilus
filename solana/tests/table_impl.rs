use nautilus::*;

trait CreatePerson<'a> {}

impl<'a> CreatePerson<'a> for Wallet<'a> {}
