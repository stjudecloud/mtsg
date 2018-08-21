use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use glob::glob;
use regex::Regex;

static DEFAULT_TAG: &str = "unknown";

pub fn generate<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file = File::create(dst)?;
    let mut writer = BufWriter::new(file);

    let mut pattern = src.as_ref().to_path_buf();
    pattern.push("*.vcf");

    let pattern = pattern.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid path: {:?}", pattern),
        )
    })?;

    let pathnames = glob(pattern)
        .map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidInput, format!("{}", e))
        })?
        .filter_map(Result::ok);

    for pathname in pathnames {
        let sample_id = pathname.file_stem()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid path: {:?}", pattern),
                )
            })?;

        let disease = parse_disease(sample_id).unwrap_or(DEFAULT_TAG);

        writeln!(&mut writer, "{}\t{}", sample_id, disease)?;
    }

    Ok(())
}

/// Extracts the disease name from a sample ID.
///
/// The regex pattern used is crude. It incorrectly captures single letter
/// symbols with the case number (e.g., SJE2001_D => E2) and misses symbols with
/// trailing numbers (e.g., SJAMLM7005_D (AMLM7)).
fn parse_disease(name: &str) -> Option<&str> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"SJ(\w\d*?\w+?)\d+").unwrap();
    }

    PATTERN.captures(name).and_then(|matches| {
        matches.get(1).map(|m| m.as_str())
    })
}

#[cfg(test)]
mod tests {
    use super::parse_disease;

    #[test]
    fn test_parse_disease() {
        assert_eq!(parse_disease("SJACT001_D"), Some("ACT"));
        assert_eq!(parse_disease("SJAMLM7005_D"), Some("AMLM"));
        assert_eq!(parse_disease("SJBALL020013_D1"), Some("BALL"));
        assert_eq!(parse_disease("SJE2A001_D"), Some("E2A"));
        assert_eq!(parse_disease("SJRB001130_M1"), Some("RB"));
        assert_eq!(parse_disease("XXABC001_D"), None);
    }
}
