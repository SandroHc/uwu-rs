pub fn uwuify(input: &str) -> String {
    input
        .to_string()
        .replace('n', "ny")
        .replace(['l', 'r'], "w")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_uwuify() {
        assert_eq!(uwuify("nice"), "nyice");
        assert_eq!(uwuify("very elegant solution"), "vewy eweganyt sowutiony");
    }
}
