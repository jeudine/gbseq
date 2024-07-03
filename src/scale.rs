use std::fmt;
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum Scale {
    #[default]
    NaturalMinor,
    HarmonicMinor,
    PhrygianMode,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NaturalMinor => "Nat Min",
                Self::HarmonicMinor => "Har Min",
                Self::PhrygianMode => "Phr Mod",
            }
        )
    }
}
