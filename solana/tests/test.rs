
mod impl_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tests() {
        impl_test::impl_test();
    }
}