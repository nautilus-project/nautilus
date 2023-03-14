#[test]
fn entry() {
    use nautilus::*;

    #[nautilus]
    mod my_mod {
        fn print_something(from: String, to: String) {
            println!("Something");
        }
        fn print_something_again(from: String, to: String) {
            println!("Something again");
        }
    }
}
