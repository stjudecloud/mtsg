use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

use log::warn;

static SUBSTITUTIONS: &[&str] = &["C>A", "C>G", "C>T", "T>A", "T>C", "T>G"];

#[rustfmt::skip]
static C_TRIPLETS: &[&str] = &[
    "ACA", "ACC", "ACG", "ACT",
    "CCA", "CCC", "CCG", "CCT",
    "GCA", "GCC", "GCG", "GCT",
    "TCA", "TCC", "TCG", "TCT",
];

#[rustfmt::skip]
static T_TRIPLETS: &[&str] = &[
    "ATA", "ATC", "ATG", "ATT",
    "CTA", "CTC", "CTG", "CTT",
    "GTA", "GTC", "GTG", "GTT",
    "TTA", "TTC", "TTG", "TTT",
];

const TOTAL_TRIPLETS: usize = 96;

// The first two columns "Substitution Type" and "Trinucleotide" are not used.
const N_SKIPPABLE_HEADERS: usize = 2;
const N_SIGNATURES: usize = 30;

// COSMIC mutational signature probabilities
static SP_URL: &str =
    "https://cancer.sanger.ac.uk/cancergenome/assets/signatures_probabilities.txt";

pub fn download_signature_probabilities<W>(writer: &mut W) -> io::Result<()>
where
    W: Write,
{
    let body = download().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let (headers, rows) = extract_table(body.as_bytes())?;
    write_table(writer, &headers, &rows).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn download() -> reqwest::Result<String> {
    reqwest::blocking::get(SP_URL).and_then(|r| r.text())
}

/// Extracts 30 known mutational signature probabilities and their 96 somatic
/// mutation types.
///
/// Returns a list of headers and raw row data (type + probabilities).
///
/// # Errors
///
/// Returns an I/O error if parsing the header fails or fewer than 96 mutation
/// types are found.
fn extract_table<R>(reader: R) -> io::Result<(Vec<String>, Vec<Vec<String>>)>
where
    R: Read,
{
    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);

    // The `take` adapter is used instead of reading to the end of line because
    // there's empty column data trailing each row.
    let headers: Vec<String> = csv
        .headers()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .iter()
        .skip(N_SKIPPABLE_HEADERS)
        .take(N_SIGNATURES + 1)
        .map(String::from)
        .collect();

    if headers.len() < N_SIGNATURES + 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "expected {} columns, got {}",
                N_SIGNATURES + 1,
                headers.len()
            ),
        ));
    }

    let mut mapped_rows = HashMap::new();

    for record in csv.records().filter_map(Result::ok) {
        let row: Vec<String> = record
            .iter()
            .skip(N_SKIPPABLE_HEADERS)
            .take(N_SIGNATURES + 1)
            .map(String::from)
            .collect();

        mapped_rows.insert(row[0].clone(), row);
    }

    let ordered_rows: Vec<Vec<String>> = somatic_mutation_types()
        .filter_map(|ty| {
            let row = mapped_rows.remove(&ty);

            if row.is_none() {
                warn!("missing row for '{}'", ty);
            }

            row
        })
        .collect();

    if ordered_rows.len() < TOTAL_TRIPLETS {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "expected {} triplets, got {}",
                TOTAL_TRIPLETS,
                ordered_rows.len()
            ),
        ))
    } else {
        Ok((headers, ordered_rows))
    }
}

fn write_table<W>(writer: &mut W, headers: &[String], rows: &[Vec<String>]) -> csv::Result<()>
where
    W: Write,
{
    let mut csv = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(writer);

    csv.write_record(headers)?;

    for row in rows {
        csv.write_record(row)?;
    }

    Ok(())
}

/// Builds an iterator that returns mutation types in the same order used by
/// MutationalPatterns.
///
/// This order can be seen in both [`signatures_probabilities.txt`] (column "Somatic Mutation
/// Type") and [`MutationalPatterns.R`] (`TRIPLETS_96`).
///
/// [`signatures_probabilities.txt`]: https://cancer.sanger.ac.uk/cancergenome/assets/signatures_probabilities.txt
/// [`MutationalPatterns.R`]: https://github.com/UMCUGenetics/MutationalPatterns/blob/283570e84132416764ebbc739bf44f6edc982791/R/MutationalPatterns.R
fn somatic_mutation_types() -> impl Iterator<Item = String> {
    C_TRIPLETS
        .iter()
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
    use std::fs;

    use super::*;

    #[test]
    fn test_extract_table() {
        let data = fs::read_to_string("test/fixtures/probabilities.txt").unwrap();
        let (headers, rows) = extract_table(data.as_bytes()).unwrap();
        assert_eq!(headers.len(), 31);
        assert_eq!(rows.len(), TOTAL_TRIPLETS);
    }

    #[test]
    #[should_panic(expected = "expected 31 columns, got 0")]
    fn test_extract_table_with_an_empty_reader() {
        extract_table("".as_bytes()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected 31 columns, got 3")]
    fn test_extract_table_with_fewer_signature_columns() {
        let data =
            fs::read_to_string("test/fixtures/probabilities.missing-signatures.txt").unwrap();
        extract_table(data.as_bytes()).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected 96 triplets, got 2")]
    fn test_extract_table_with_missing_mutation_types() {
        let data =
            fs::read_to_string("test/fixtures/probabilities.missing-mutation-types.txt").unwrap();
        extract_table(data.as_bytes()).unwrap();
    }

    #[test]
    fn test_write_table() {
        let headers: Vec<String> = vec![String::from("Signature 1"), String::from("Signature 2")];

        let rows: Vec<Vec<String>> = vec![
            vec![String::from("0.95"), String::from("0.05")],
            vec![String::from("0.33"), String::from("0.67")],
        ];

        let mut writer = Vec::new();

        write_table(&mut writer, &headers, &rows).unwrap();

        let actual = String::from_utf8(writer).unwrap();

        let expected = "\
Signature 1\tSignature 2
0.95\t0.05
0.33\t0.67
";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_somatic_mutation_types() {
        let types: Vec<String> = somatic_mutation_types().collect();

        assert_eq!(types.len(), TOTAL_TRIPLETS);

        assert_eq!(&types[0], "A[C>A]A");
        assert_eq!(&types[1], "A[C>A]C");

        assert_eq!(&types[15], "T[C>A]T");
        assert_eq!(&types[16], "A[C>G]A");

        assert_eq!(&types[94], "T[T>G]G");
        assert_eq!(&types[95], "T[T>G]T");
    }
}
