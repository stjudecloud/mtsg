#[macro_use]
extern crate clap;

use std::io;
use std::path::Path;
use std::process::Command;

use clap::{App, Arg};

fn mutational_patterns<P, Q, R, S>(
    vcfs_dir: P,
    sample_sheet: Q,
    cancer_signatures: R,
    reference_genome: &str,
    min_burden: u32,
    min_contribution: u32,
    out_dir: S,
) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
    S: AsRef<Path>,
{
    let child = Command::new("Rscript")
        .arg("/app/src/r/mutational_patterns.R")
        .arg(vcfs_dir.as_ref())
        .arg(sample_sheet.as_ref())
        .arg(cancer_signatures.as_ref())
        .arg(reference_genome)
        .arg(min_burden.to_string())
        .arg(min_contribution.to_string())
        .arg(out_dir.as_ref())
        .spawn()?;

    let output = child.wait_with_output()?;

    println!("{}", String::from_utf8(output.stdout).unwrap());

    Ok(())
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("cancer-signatures")
            .long("cancer-signatures")
            .value_name("file")
            .help("Cancer genome signature probabilities"))
        .arg(Arg::with_name("genome-build")
            .long("genome-build")
            .value_name("string")
            .help("Name of the reference genome in which the inputs are aligned")
            .default_value("GRCh38")
            .possible_values(&["GRCh37", "GRCh38"]))
        .arg(Arg::with_name("min-burden")
            .long("min-burden")
            .value_name("integer")
            .help("Threshold to exclude mutations with low burden")
            .default_value("9"))
        .arg(Arg::with_name("min-contribution")
            .long("min-contribution")
            .value_name("integer")
            .help("Threshold to exclude signatures with low contribution")
            .default_value("9"))
        .arg(Arg::with_name("output-directory")
            .short("o")
            .long("output-directory")
            .value_name("directory")
            .help("Results directory")
            .required(true))
        .arg(Arg::with_name("vcfs-dir")
            .long("vcfs-dir")
            .help("Input directory of single-sample VCFs")
            .required(true)
            .index(1))
        .arg(Arg::with_name("sample-sheet")
            .long("sample-sheet")
            .help("Sample sheet of sample names mapped to tissue of origin")
            .required(true)
            .index(2))
        .get_matches();

    let vcfs_dir = matches.value_of("vcfs-dir").unwrap();
    let sample_sheet = matches.value_of("sample-sheet").unwrap();
    let cancer_signatures = matches.value_of("cancer-signatures").unwrap();
    let genome_build = matches.value_of("genome-build").unwrap();
    let min_burden = value_t!(matches, "min-burden", u32).unwrap_or_else(|e| e.exit());
    let min_contribution = value_t!(matches, "min-contribution", u32).unwrap_or_else(|e| e.exit());
    let out_dir = matches.value_of("output-directory").unwrap();

    mutational_patterns(
        vcfs_dir,
        sample_sheet,
        cancer_signatures,
        genome_build,
        min_burden,
        min_contribution,
        out_dir,
    ).unwrap();
}
