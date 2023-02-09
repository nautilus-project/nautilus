// #[test]
// fn entrypoint_tests() {
//     let t = trybuild::TestCases::new();
//     t.pass("tests/entrypoint/01-parse.rs");
//     t.pass("tests/entrypoint/02-entrypoint-borsh.rs");
// }

#[test]
fn account_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/accounts/01-parse.rs");
    // t.pass("tests/accounts/02-parse-attrs.rs");
    // t.pass("tests/accounts/03-account-borsh.rs");
}
