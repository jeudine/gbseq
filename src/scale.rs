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

impl Scale {
    pub fn next(self) -> Self {
        match self {
            Self::NaturalMinor => Self::HarmonicMinor,
            Self::HarmonicMinor => Self::PhrygianMode,
            Self::PhrygianMode => Self::NaturalMinor,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::NaturalMinor => Self::PhrygianMode,
            Self::HarmonicMinor => Self::NaturalMinor,
            Self::PhrygianMode => Self::HarmonicMinor,
        }
    }
}
