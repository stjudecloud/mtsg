pub mod r#type;
pub mod v1;
pub mod v2;

pub use r#type::Type;

pub static PREFIX: &str = "SJ";

pub trait Sjid {
    fn disease(&self) -> &str;
    fn number(&self) -> u32;
    fn ty(&self) -> Type;
    fn index(&self) -> usize;
    fn secondary_id(&self) -> Option<&str>;
    fn subject(&self) -> String;
    fn sample(&self) -> String;
}

pub fn parse(s: &str) -> Result<Box<dyn Sjid>, ()> {
    v1::parse(s)
        .map(|sjid| Box::new(sjid) as Box<dyn Sjid>)
        .or_else(|_| v2::parse(s).map(|sjid| Box::new(sjid) as Box<dyn Sjid>))
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse() {
        assert!(parse("SJACT001_D").is_ok());
        assert!(parse("SJBALL020469_D1").is_ok());
        assert!(parse("").is_err());
        assert!(parse("HAP1_CHK2_56-3").is_err());
    }
}
