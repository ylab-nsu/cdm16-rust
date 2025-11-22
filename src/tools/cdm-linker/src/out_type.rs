use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq, clap::ValueEnum)]
pub enum OutputType {
    // Cocas-compatible object file
    #[default]
    #[value(name = "object")]
    Object,
    // Logisim image file
    #[value(name = "image")]
    Image,
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("invalid output type")]
pub struct InvalidOutputType;

impl std::str::FromStr for OutputType {
    type Err = InvalidOutputType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "object" => Ok(OutputType::Object),
            "image" => Ok(OutputType::Image),
            _ => Err(InvalidOutputType),
        }
    }
}

impl Display for OutputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            OutputType::Object => write!(f, "object"),
            OutputType::Image => write!(f, "image"),
        }
    }
}
