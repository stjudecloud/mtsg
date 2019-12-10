use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{self, json};

static CRATE_NAME: &str = env!("CARGO_PKG_NAME");
static CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

static TAG_COLUMN_NAME: &str = "tissue";

static SIGNATURES_TEMPLATE: &str = include_str!("signatures.html.hbs");

static HBS: Lazy<Handlebars> = Lazy::new(|| {
    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.register_template_string("signatures", SIGNATURES_TEMPLATE)
        .unwrap();
    hbs
});

#[derive(Serialize)]
pub struct Sample {
    id: String,
    disease: String,
    contributions: Vec<f64>,
}

pub fn create_visualization<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file = File::open(&src)?;
    let pathname = format!("{}", src.as_ref().display());
    let (headers, samples) = read_table(file, &pathname)?;

    let mut file = File::create(dst)?;
    write_html(&mut file, &headers, &samples)
}

pub fn read_table<R>(reader: R, pathname: &str) -> io::Result<(Vec<String>, Vec<Sample>)>
where
    R: Read,
{
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}:1: {}", pathname, e)))?
        .iter()
        .map(String::from)
        .collect();

    let name = headers.last().map(String::as_str).unwrap_or("");

    if name != TAG_COLUMN_NAME {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "{}:1: expected last column to be '{}', got '{}'",
                pathname, TAG_COLUMN_NAME, name
            ),
        ));
    }

    let n_headers = headers.len();

    let headers: Vec<String> = headers
        .into_iter()
        .skip(1)
        .take(n_headers - 2)
        .map(|h| h.replace(".", " "))
        .collect();

    let samples = reader
        .records()
        .filter_map(Result::ok)
        .enumerate()
        .map(|(i, record)| {
            let line_no = i + 2;

            let id = record[0].to_string();

            let disease = record.get(n_headers - 1).map(String::from).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}:{}: missing {}", pathname, line_no, TAG_COLUMN_NAME),
                )
            })?;

            let contributions: Vec<f64> = record
                .iter()
                .skip(1)
                .take(n_headers - 2)
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{}:{}: {}", pathname, line_no, e),
                    )
                })?;

            Ok(Sample {
                id,
                disease,
                contributions,
            })
        })
        .collect::<Result<Vec<Sample>, io::Error>>()?;

    Ok((headers, samples))
}

fn write_html<W>(writer: &mut W, headers: &[String], samples: &[Sample]) -> io::Result<()>
where
    W: Write,
{
    let diseases: BTreeSet<String> = samples.iter().map(|s| s.disease.clone()).collect();

    let payload = json!({
        "data": {
            "headers": headers,
            "samples": samples,
        },
    });

    let data = json!({
        "diseases": diseases,
        "generator": format!("{} {}", CRATE_NAME, CRATE_VERSION),
        "payload": serde_json::to_string(&payload).unwrap(),
    });

    let result = HBS.render("signatures", &data).unwrap();

    writer.write_all(result.as_bytes())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_read_table() {
        let data = fs::read_to_string("test/fixtures/signatures.txt").unwrap();
        let (headers, samples) = read_table(data.as_bytes(), "<test>").unwrap();

        assert_eq!(headers.len(), 30);
        assert_eq!(headers[0], "Signature 1");
        assert_eq!(headers[29], "Signature 30");

        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].id, "SJACT001_D");
        assert_eq!(samples[0].disease, "ACT");
        assert_eq!(samples[0].contributions.len(), 30);
        assert_eq!(samples[0].contributions[0], 1.71758029457482);
        assert_eq!(samples[0].contributions[29], 0.0);
    }

    #[test]
    #[should_panic(expected = r#"expected last column to be \'tissue\', got \'\'"#)]
    fn test_read_table_with_empty_reader() {
        read_table("".as_bytes(), "<test>").unwrap();
    }

    #[test]
    #[should_panic(expected = r#"expected last column to be \'tissue\', got \'Signature.1\'"#)]
    fn test_read_table_with_no_tissue_column() {
        let data = "\
\tSignature.1
SJACT001_D\t0
SJAMLM7005_D\t0
";

        read_table(data.as_bytes(), "<test>").unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid float literal")]
    fn test_read_table_with_bad_contributions() {
        let data = "\
\tSignature.1\ttissue
SJACT001_D\t0\tACT
SJAMLM7005_D\tNA\tAMLM7
";

        read_table(data.as_bytes(), "<test>").unwrap();
    }

    #[test]
    fn test_write_html() {
        let headers = vec![
            String::from("Signature 1"),
            String::from("Signature 2"),
            String::from("Signature 3"),
        ];

        let samples = vec![Sample {
            id: String::from("SJACT001_D"),
            disease: String::from("ACT"),
            contributions: vec![11.7157, 131.0337, 295.0582],
        }];

        let mut buf = Vec::new();
        write_html(&mut buf, &headers, &samples).unwrap();

        assert!(buf.starts_with(b"<!DOCTYPE html>"));
    }
}
