use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Cursor, Read, Write};
use std::iter;
use std::path::Path;

use csv;
use reqwest;

static SUBSTITUTIONS: &[&str] = &["C>A", "C>G", "C>T", "T>A", "T>C", "T>G"];

static C_TRIPLETS: &[&str] = &[
    "ACA", "ACC", "ACG", "ACT",
    "CCA", "CCC", "CCG", "CCT",
    "GCA", "GCC", "GCG", "GCT",
    "TCA", "TCC", "TCG", "TCT",
];

static T_TRIPLETS: &[&str] = &[
    "ATA", "ATC", "ATG", "ATT",
    "CTA", "CTC", "CTG", "CTT",
    "GTA", "GTC", "GTG", "GTT",
    "TTA", "TTC", "TTG", "TTT",
];

// COSMIC mutational signature probabilities
static SP_URL: &str = "https://cancer.sanger.ac.uk/cancergenome/assets/signatures_probabilities.txt";

pub fn download_signature_probabilities<P>(dst: P) -> io::Result<()> where P: AsRef<Path> {
    let body = download().map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("{}", e))
    })?;

    let reader = Cursor::new(body);
    let (headers, rows) = process(reader)?;

    let file = File::create(dst)?;
    let mut writer = BufWriter::new(file);

    for row in iter::once(&headers).chain(rows.iter()) {
        for (i, cell) in row.iter().enumerate() {
            write!(&mut writer, "{}", cell)?;

            if i < row.len() - 1 {
                write!(&mut writer, "\t")?;
            }
        }

        writeln!(&mut writer)?;
    }

    Ok(())
}

fn download() -> reqwest::Result<String> {
    let mut response = reqwest::get(SP_URL)?;
    response.text()
}

fn process<R>(reader: R) -> io::Result<(Vec<String>, Vec<Vec<String>>)>
where
    R: Read,
{
    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);

    let headers: Vec<String> = csv.headers()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{}", e)))?
        .iter()
        .skip(2)
        .take(31)
        .map(String::from)
        .collect();

    let mut mapped_rows = HashMap::new();

    for record in csv.records().filter_map(Result::ok) {
        let row: Vec<String> = record.iter()
            .skip(2)
            .take(31)
            .map(String::from)
            .collect();

        mapped_rows.insert(row[0].clone(), row);
    }

    let ordered_rows: Vec<Vec<String>> = somatic_mutation_types()
        .map(|ty| mapped_rows[&ty].clone())
        .collect();

    Ok((headers, ordered_rows))
}

/// Builds an iterator that returns mutation types in the same order used by
/// MutationalPatterns.
fn somatic_mutation_types() -> impl Iterator<Item = String> {
    C_TRIPLETS.iter()
        .chain(C_TRIPLETS.iter())
        .chain(C_TRIPLETS.iter())
        .chain(T_TRIPLETS.iter())
        .chain(T_TRIPLETS.iter())
        .chain(T_TRIPLETS.iter())
        .enumerate()
        .map(|(i, &triplet)| {
            let j = i / C_TRIPLETS.len();
            format!("{}[{}]{}", &triplet[..1], SUBSTITUTIONS[j], &triplet[2..])
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_somatic_mutation_types() {
        let types: Vec<String> = somatic_mutation_types().collect();

        assert_eq!(types.len(), 96);

        assert_eq!(&types[0], "A[C>A]A");
        assert_eq!(&types[1], "A[C>A]C");

        assert_eq!(&types[15], "T[C>A]T");
        assert_eq!(&types[16], "A[C>G]A");

        assert_eq!(&types[94], "T[T>G]G");
        assert_eq!(&types[95], "T[T>G]T");
    }
}

