use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Url(pub Box<str>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HtmlPage(pub Box<str>);

impl TryFrom<Box<str>> for Url {
    type Error = ();

    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        // TODO add validation
        Ok(Url(value))
    }
}

impl From<Url> for Box<str> {
    fn from(value: Url) -> Self {
        value.0
    }
}

impl From<HtmlPage> for Box<str> {
    fn from(value: HtmlPage) -> Self {
        value.0
    }
}

impl From<Box<str>> for HtmlPage {
    fn from(value: Box<str>) -> Self {
        HtmlPage(value)
    }
}
