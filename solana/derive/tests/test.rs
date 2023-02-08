// #[test]
// fn entrypoint_tests() {
//     let t = trybuild::TestCases::new();
//     // t.pass("tests/entrypoint/01-parse.rs");
//     //t.pass("tests/entrypoint/02-create-builder.rs");
//     //t.pass("tests/entrypoint/03-call-setters.rs");
//     //t.pass("tests/entrypoint/04-call-build.rs");
//     //t.pass("tests/entrypoint/05-method-chaining.rs");
//     //t.pass("tests/entrypoint/06-optional-field.rs");
//     //t.pass("tests/entrypoint/07-repeated-field.rs");
//     //t.compile_fail("tests/entrypoint/08-unrecognized-attribute.rs");
//     //t.pass("tests/entrypoint/09-redefined-prelude-types.rs");
// }

#[test]
fn account_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/accounts/01-parse.rs");
    t.pass("tests/accounts/02-impl-nautilus-account.rs");
    //t.pass("tests/accounts/03-call-setters.rs");
    //t.pass("tests/accounts/04-call-build.rs");
    //t.pass("tests/accounts/05-method-chaining.rs");
    //t.pass("tests/accounts/06-optional-field.rs");
    //t.pass("tests/accounts/07-repeated-field.rs");
    //t.compile_fail("tests/accounts/08-unrecognized-attribute.rs");
    //t.pass("tests/accounts/09-redefined-prelude-types.rs");
}
