use lazy_static::lazy_static;
use regex::Regex;

use super::{Sjid, Type};
use crate::sjid;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(
        r"(?x)
        SJ
        (?P<disease>[[:alnum:]]*[[:alpha:]]\d?)
        (?P<number>\d{3})
        _
        (?P<type>[[:upper:]])
        (-(?P<secondary_id>.+))?"
    )
    .unwrap();
}

#[derive(Debug)]
pub struct SampleName {
    disease: String,
    number: u32,
    ty: Type,
    index: usize,
    secondary_id: Option<String>,
}

impl Sjid for SampleName {
    fn disease(&self) -> &str {
        &self.disease
    }

    fn number(&self) -> u32 {
        self.number
    }

    fn ty(&self) -> Type {
        self.ty
    }

    fn index(&self) -> usize {
        self.index
    }

    fn secondary_id(&self) -> Option<&str> {
        self.secondary_id.as_ref().map(String::as_str)
    }

    fn subject(&self) -> String {
        format!("{}{}{:03}", sjid::PREFIX, self.disease, self.number)
    }

    fn sample(&self) -> String {
        let ty = map_type(self.ty, self.index);
        format!("{}_{}", self.subject(), ty)
    }
}

pub fn parse(s: &str) -> Result<SampleName, ()> {
    let matches = PATTERN.captures(s).ok_or_else(|| ())?;

    let disease = matches["disease"].to_string();
    let number = parse_number(&matches["number"])?;
    let ty = parse_type(&matches["type"])?;
    let index = parse_index(&matches["type"])?;
    let secondary_id = matches.name("secondary_id").map(|s| s.as_str().to_string());

    Ok(SampleName {
        disease,
        number,
        ty,
        index,
        secondary_id,
    })
}

pub fn parse_number(s: &str) -> Result<u32, ()> {
    s.parse().map_err(|_| ())
}

pub fn parse_type(code: &str) -> Result<Type, ()> {
    match code {
        "Y" | "Z" => Ok(Type::Xenograft),
        "S" | "T" => Ok(Type::Relapse),
        "E" | "F" => Ok(Type::Diagnosis),
        "B" => Ok(Type::Autopsy),
        "H" | "I" => Ok(Type::Germline),
        _ => code.parse(),
    }
}

pub fn parse_index(code: &str) -> Result<usize, ()> {
    match code {
        "G" | "D" | "X" | "A" | "M" | "O" | "R" | "C" => Ok(1),
        "Y" | "S" | "E" | "B" | "H" => Ok(2),
        "Z" | "F" | "T" | "I" => Ok(3),
        _ => Err(()),
    }
}

/// Maps a v2 type and index to a v1 type.
pub fn map_type(ty: Type, index: usize) -> &'static str {
    match (ty, index) {
        (Type::Autopsy, 1) => ty.code(),
        (Type::Autopsy, 2) => "B",

        (Type::CellLine, 1) => ty.code(),

        (Type::Diagnosis, 1) => ty.code(),
        (Type::Diagnosis, 2) => "E",
        (Type::Diagnosis, 3) => "F",

        (Type::Germline, 1) => ty.code(),
        (Type::Germline, 2) => "H",
        (Type::Germline, 3) => "I",

        (Type::Metastatic, 1) => ty.code(),

        (Type::Relapse, 1) => ty.code(),
        (Type::Relapse, 2) => "S",
        (Type::Relapse, 3) => "T",

        (Type::Xenograft, 1) => ty.code(),
        (Type::Xenograft, 2) => "Y",
        (Type::Xenograft, 3) => "Z",

        (_, _) => panic!("invalid type '{:?}' and index '{}'", ty, index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_sjid() -> SampleName {
        SampleName {
            disease: String::from("ETV"),
            number: 47,
            ty: Type::Relapse,
            index: 2,
            secondary_id: Some(String::from("TB-00-0000")),
        }
    }

    #[test]
    fn test_disease() {
        let sjid = build_sjid();
        assert_eq!(sjid.disease(), "ETV");
    }

    #[test]
    fn test_number() {
        let sjid = build_sjid();
        assert_eq!(sjid.number(), 47);
    }

    #[test]
    fn test_ty() {
        let sjid = build_sjid();
        assert_eq!(sjid.ty(), Type::Relapse);
    }

    #[test]
    fn test_index() {
        let sjid = build_sjid();
        assert_eq!(sjid.index(), 2);
    }

    #[test]
    fn test_secondary_id() {
        let sjid = build_sjid();
        assert_eq!(sjid.secondary_id(), Some("TB-00-0000"));
    }

    #[test]
    fn test_subject() {
        let sjid = build_sjid();
        assert_eq!(sjid.subject(), "SJETV047");
    }

    #[test]
    fn test_sample() {
        let sjid = build_sjid();
        assert_eq!(sjid.sample(), "SJETV047_S");
    }

    #[test]
    fn test_parse() {
        let sjid = parse("SJETV047_S-TB-00-0000").unwrap();

        assert_eq!(sjid.disease, "ETV");
        assert_eq!(sjid.number, 47);
        assert_eq!(sjid.ty, Type::Relapse);
        assert_eq!(sjid.index, 2);
        assert_eq!(sjid.secondary_id, Some(String::from("TB-00-0000")));
    }

    #[test]
    fn test_parse_index() {
        assert_eq!(parse_index("G"), Ok(1));
        assert_eq!(parse_index("D"), Ok(1));
        assert_eq!(parse_index("X"), Ok(1));
        assert_eq!(parse_index("A"), Ok(1));
        assert_eq!(parse_index("M"), Ok(1));
        assert_eq!(parse_index("O"), Ok(1));
        assert_eq!(parse_index("R"), Ok(1));
        assert_eq!(parse_index("C"), Ok(1));

        assert_eq!(parse_index("Y"), Ok(2));
        assert_eq!(parse_index("S"), Ok(2));
        assert_eq!(parse_index("E"), Ok(2));
        assert_eq!(parse_index("B"), Ok(2));
        assert_eq!(parse_index("H"), Ok(2));

        assert_eq!(parse_index("Z"), Ok(3));
        assert_eq!(parse_index("F"), Ok(3));
        assert_eq!(parse_index("T"), Ok(3));
        assert_eq!(parse_index("I"), Ok(3));
    }

    #[test]
    fn test_map_type() {
        assert_eq!(map_type(Type::Autopsy, 1), "A");
        assert_eq!(map_type(Type::Autopsy, 2), "B");

        assert_eq!(map_type(Type::CellLine, 1), "C");

        assert_eq!(map_type(Type::Diagnosis, 1), "D");
        assert_eq!(map_type(Type::Diagnosis, 2), "E");
        assert_eq!(map_type(Type::Diagnosis, 3), "F");

        assert_eq!(map_type(Type::Germline, 1), "G");
        assert_eq!(map_type(Type::Germline, 2), "H");
        assert_eq!(map_type(Type::Germline, 3), "I");

        assert_eq!(map_type(Type::Metastatic, 1), "M");

        assert_eq!(map_type(Type::Relapse, 1), "R");
        assert_eq!(map_type(Type::Relapse, 2), "S");
        assert_eq!(map_type(Type::Relapse, 3), "T");

        assert_eq!(map_type(Type::Xenograft, 1), "X");
        assert_eq!(map_type(Type::Xenograft, 2), "Y");
        assert_eq!(map_type(Type::Xenograft, 3), "Z");
    }
}
