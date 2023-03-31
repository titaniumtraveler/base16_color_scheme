use serde::{Deserialize, Serialize};


#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Format {
    Hex(Hex),
    Rgb(Rgb),
    Dec(Dec),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hex {
    Rgb,
    R,
    G,
    B,
    Bgr,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rgb {
    R,
    G,
    B,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dec {
    R,
    G,
    B,
}

}
