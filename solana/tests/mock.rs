struct Mock<'a> {
    field: &'a String,
}

fn mock_sublayer(mock1: Mock, mock2: Mock) {
    println!("{}", mock1.field);
    println!("{}", mock2.field);
}

fn mock_top() {
    let mock1 = Mock {
        field: &String::from("Joe"),
    };
    let mock2 = Mock {
        field: &String::from("Joe"),
    };
    mock_sublayer(mock1, mock2);
}

#[test]
fn mock() {
    mock_top();
}
