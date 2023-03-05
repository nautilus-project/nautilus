use nautilus::nautilus;

#[test]
fn can_parse() {
    #[nautilus]
    fn hello_world(message: String) {
        println!("{}", message);
    }

    fn my_instruction(message: String) {
        hello_world(message, "Yo".to_string());
    }

    my_instruction("Joe".to_string());
}
