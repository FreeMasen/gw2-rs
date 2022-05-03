use uuid::Uuid;

pub mod codec;
pub mod endpoints;


trait UpperHyphenated {
    fn as_upper_hyphenated(&self) -> String;
}

impl UpperHyphenated for Uuid {
    fn as_upper_hyphenated(&self) -> String {
        format!("{}", self.as_hyphenated()).to_ascii_uppercase()
    }
}
