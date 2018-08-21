use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use csv;
use handlebars::Handlebars;
use serde_json;

static CRATE_NAME: &str = env!("CARGO_PKG_NAME");
static CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

static SIGNATURES_TEMPLATE: &str = include_str!("signatures.html.hbs");

lazy_static! {
    static ref HBS: Handlebars = {
        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);
        hbs.register_template_string("signatures", SIGNATURES_TEMPLATE).unwrap();
        hbs
    };
}

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
    let (headers, samples) = read_samples(src)?;

    let diseases: BTreeSet<String> = samples.iter()
        .map(|s| s.disease.clone())
        .collect();

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

    let mut file = File::create(dst)?;
    file.write_all(result.as_bytes())?;

    Ok(())
}

pub fn read_samples<P>(src: P) -> io::Result<(Vec<String>, Vec<Sample>)>
where
    P: AsRef<Path>
{
    let filename = match src.as_ref().file_name().and_then(OsStr::to_str) {
        Some(filename) => filename.to_string(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                String::from("invalid src"),
            ))
        },
    };

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(src)?;

    let headers = reader.headers()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{}:1: {}", filename, e),
            )
        })?
        .iter()
        .skip(1)
        .take(30)
        .map(String::from)
        .collect();

    let samples = reader.records()
        .filter_map(Result::ok)
        .enumerate()
        .map(|(i, record)| {
            let line_no = i + 2;

            let id = record[0].to_string();
            let disease = record.get(31)
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{}:{}: missing tissue", filename, line_no),
                    )
                })?;

            let contributions: Vec<f64> = record.iter()
                .skip(1)
                .take(30)
                .map(|v| v.parse())
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{}:{}: {}", filename, line_no, e),
                    )
                })?;

            Ok(Sample { id, disease, contributions })
        })
        .collect::<Result<Vec<Sample>, io::Error>>()?;

    Ok((headers, samples))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_samples() {
        let (headers, samples) = read_samples("test/fixtures/signatures.txt").unwrap();

        assert_eq!(headers.len(), 30);
        assert_eq!(headers[0], "Signature.1");

        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].id, "SJACT001_D");
        assert_eq!(samples[0].disease, "ACT");
        assert_eq!(samples[0].contributions.len(), 30);
        assert_eq!(samples[0].contributions[0], 1.71758029457482);
    }

    #[test]
    fn test_read_samples_with_invalid_src() {
        let result = read_samples("/");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_samples_with_bad_contributions() {
        let result = read_samples("test/fixtures/signatures.bad-contribution.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_samples_with_no_tissue_column() {
        let result = read_samples("test/fixtures/signatures.no-tissue-column.txt");
        assert!(result.is_err());
    }
}
