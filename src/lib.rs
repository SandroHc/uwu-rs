use thiserror::Error;

#[derive(Error, Debug)]
pub enum UwuError {
    #[error(transparent)]
    Unknown(#[from] Box<dyn std::error::Error + Send>),
}

pub fn uwuify(input: &str) -> Result<String, UwuError> {
    Ok(input
        .to_string()
        .replace('n', "ny")
        .replace(['l', 'r'], "w"))
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn assert_uwuify() {
        assert_eq!(uwuify("nice").unwrap(), "nyice");
        assert_eq!(
            uwuify("very elegant solution").unwrap(),
            "vewy eweganyt sowutiony"
        );
    }
}
