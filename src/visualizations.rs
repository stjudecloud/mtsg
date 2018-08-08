use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use csv;
use handlebars::Handlebars;
use serde_json;

static CRATE_NAME: &str = env!("CARGO_PKG_NAME");
static CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

static DEFAULT_TEMPLATE: &str = include_str!("default.html.hbs");

lazy_static! {
    static ref HBS: Handlebars = {
        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);
        hbs.register_template_string("default", DEFAULT_TEMPLATE).unwrap();
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

    let result = HBS.render("default", &data).unwrap();

    let mut file = File::create(dst)?;
    file.write_all(result.as_bytes())?;

    Ok(())
}

pub fn read_samples<P>(src: P) -> io::Result<(Vec<String>, Vec<Sample>)>
where
    P: AsRef<Path>
{
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(src)?;

    let headers = reader.headers()
        .map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidInput, format!("{}", e))
        })?
        .iter()
        .skip(1)
        .take(30)
        .map(String::from)
        .collect();

    let samples = reader.records()
        .filter_map(Result::ok)
        .map(|record| {
            let id = record[0].to_string();
            let disease = record[31].to_string();

            let contributions: Vec<f64> = record.iter()
                .skip(1)
                .take(30)
                .map(|v| v.parse())
                .collect::<Result<_, _>>()
                .map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidInput, format!("{}", e))
                })
                .unwrap();

            Sample { id, disease, contributions }
        })
        .collect();

    Ok((headers, samples))
}
