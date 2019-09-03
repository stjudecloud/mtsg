use once_cell::sync::Lazy;
use regex::Regex;

use super::{Sjid, Type};
use crate::sjid;

static PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        SJ
        (?P<disease>[[:alnum:]]*[[:alpha:]]\d?)
        (?P<number>\d{6})
        _
        (?P<type>[[:upper:]])
        (?P<index>\d+)
        (-(?P<secondary_id>.+))?",
    )
    .unwrap()
});

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
        format!("{}{:06}", sjid::PREFIX, self.number)
    }

    fn sample(&self) -> String {
        format!(
            "{}{}{:06}_{}{}",
            sjid::PREFIX,
            self.disease,
            self.number,
            self.ty.code(),
            self.index
        )
    }
}

pub fn parse(s: &str) -> Result<SampleName, ()> {
    let matches = PATTERN.captures(s).ok_or_else(|| ())?;

    let disease = matches["disease"].to_string();
    let number = parse_number(&matches["number"])?;
    let ty = matches["type"].parse()?;
    let index = matches["index"].parse().map_err(|_| ())?;
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
        assert_eq!(sjid.subject(), "SJ000047");
    }

    #[test]
    fn test_sample() {
        let sjid = build_sjid();
        assert_eq!(sjid.sample(), "SJETV000047_R2");
    }

    #[test]
    fn test_parse() {
        let sjid = parse("SJETV000047_R2-TB-00-0000").unwrap();

        assert_eq!(sjid.disease, "ETV");
        assert_eq!(sjid.number, 47);
        assert_eq!(sjid.ty, Type::Relapse);
        assert_eq!(sjid.index, 2);
        assert_eq!(sjid.secondary_id, Some(String::from("TB-00-0000")));
    }
}
