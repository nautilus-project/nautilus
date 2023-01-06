# Work Notes

### Testing
   
Use `cargo test` inside the `/solana` crate to test.   

---

### Revisited Friday (1/6)
* `#[derive(Nautilus)]` successfully implements the proper traits for the struct. However, the methods that come along with it are not complete.
* Left off on creating a new enum based off the user-defined entrypoint enum.   

### Roadmap
* Build entrypoint with defaults
* Allow for override/custom instructions
    * Will have to be able to pick up the args struct and know where it's being used (tricky)
* Finalize all implemented functions/methods for Nautilus structs
* IDL
* SDK