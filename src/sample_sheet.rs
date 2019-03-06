use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use glob::glob;
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;

static DEFAULT_TAG: &str = "unknown";

struct NameTagPair {
    name: String,
    tag: String,
}

pub fn generate<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut pattern = src.as_ref().to_path_buf();
    pattern.push("*.vcf");

    let pattern = format!("{}", pattern.display());

    let pathnames =
        list_directory(&pattern).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let pairs = build_pairs(&pathnames);

    let file = File::create(dst)?;
    let mut writer = BufWriter::new(file);

    write_table(&mut writer, &pairs)
}

fn write_table<W>(writer: &mut W, pairs: &[NameTagPair]) -> io::Result<()>
where
    W: Write,
{
    for pair in pairs {
        writeln!(writer, "{}\t{}", pair.name, pair.tag)?;
    }

    Ok(())
}

/// Returns a list of the basenames given a search pattern.
///
/// This silently discards invalid and missing filenames.
fn list_directory(pattern: &str) -> Result<Vec<String>, String> {
    let basenames = glob(pattern)
        .map_err(|e| format!("{}", e))?
        .filter_map(Result::ok)
        .filter_map(|path| basename(&path))
        .collect();

    Ok(basenames)
}

fn basename<P>(path: P) -> Option<String>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .file_stem()
        .and_then(|n| n.to_str())
        .map(String::from)
}

fn build_pairs(names: &[String]) -> Vec<NameTagPair> {
    names
        .iter()
        .map(|name| {
            let tag = parse_disease(name).unwrap_or_else(|| {
                warn!("could not extract disease from sample name '{}'", name);
                DEFAULT_TAG
            });

            NameTagPair {
                name: name.clone(),
                tag: tag.to_string(),
            }
        })
        .collect()
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

    PATTERN
        .captures(name)
        .and_then(|matches| matches.get(1).map(|m| m.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_table() {
        let pairs = vec![
            NameTagPair {
                name: String::from("SJACT001_D"),
                tag: String::from("ACT"),
            },
            NameTagPair {
                name: String::from("SJBALL020013_D1"),
                tag: String::from("BALL"),
            },
        ];

        let mut data = Vec::new();

        write_table(&mut data, &pairs).unwrap();

        let actual = String::from_utf8(data).unwrap();

        let expected = "\
SJACT001_D\tACT
SJBALL020013_D1\tBALL
";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_basename() {
        assert_eq!(basename("sample.vcf"), Some(String::from("sample")));
        assert_eq!(basename("sample.vcf.gz"), Some(String::from("sample.vcf")));
        assert_eq!(basename("sample"), Some(String::from("sample")));
        assert_eq!(basename(".vcf"), Some(String::from(".vcf")));
        assert_eq!(basename(""), None);
    }

    #[test]
    fn test_build_pairs() {
        let names = vec![String::from("SJACT001_D"), String::from("SJBALL020013_D1")];
        let pairs = build_pairs(&names);

        assert_eq!(pairs.len(), 2);

        assert_eq!(pairs[0].name, "SJACT001_D");
        assert_eq!(pairs[0].tag, "ACT");

        assert_eq!(pairs[1].name, "SJBALL020013_D1");
        assert_eq!(pairs[1].tag, "BALL");
    }

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
