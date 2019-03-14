use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Autopsy,
    CellLine,
    Diagnosis,
    Germline,
    Metastatic,
    OtherTumor,
    Relapse,
    Xenograft,
}

impl Type {
    pub fn code(&self) -> &'static str {
        match self {
            Type::Autopsy => "A",
            Type::CellLine => "C",
            Type::Diagnosis => "D",
            Type::Germline => "G",
            Type::Metastatic => "M",
            Type::OtherTumor => "O",
            Type::Relapse => "R",
            Type::Xenograft => "X",
        }
    }
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Type::Autopsy),
            "C" => Ok(Type::CellLine),
            "D" => Ok(Type::Diagnosis),
            "G" => Ok(Type::Germline),
            "M" => Ok(Type::Metastatic),
            "O" => Ok(Type::OtherTumor),
            "R" => Ok(Type::Relapse),
            "X" => Ok(Type::Xenograft),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("A".parse(), Ok(Type::Autopsy));
        assert_eq!("C".parse(), Ok(Type::CellLine));
        assert_eq!("D".parse(), Ok(Type::Diagnosis));
        assert_eq!("G".parse(), Ok(Type::Germline));
        assert_eq!("M".parse(), Ok(Type::Metastatic));
        assert_eq!("O".parse(), Ok(Type::OtherTumor));
        assert_eq!("R".parse(), Ok(Type::Relapse));
        assert_eq!("X".parse(), Ok(Type::Xenograft));

        assert_eq!("S".parse::<Type>(), Err(()));
        assert_eq!("J".parse::<Type>(), Err(()));
    }
}
