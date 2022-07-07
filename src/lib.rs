mod client;
mod response;

#[cfg(test)]
mod tests {
    use crate::response;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
