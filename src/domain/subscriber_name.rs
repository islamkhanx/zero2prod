use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = name.chars().any(|ch| forbidden_chars.contains(&ch));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not valid subscriber name.", name))
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ë".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn names_containing_an_invalid_char_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Islamkhan Serikbayev".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
