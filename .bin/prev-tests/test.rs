
mod entrypoint;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_tests() {
        entrypoint::impl_test();
    }
}