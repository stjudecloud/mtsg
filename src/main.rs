#[macro_use]
extern crate clap;
extern crate mutspec;

use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

use clap::{App, Arg, SubCommand};

use mutspec::cosmic::download_signature_probabilities;
use mutspec::vcf::split_file;

fn mutational_patterns<P, Q, R, S>(
    vcfs_dir: P,
    sample_sheet: Q,
    cancer_signatures: Option<R>,
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
    let cancer_signatures = cancer_signatures
        .map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| {
            let mut dst = env::temp_dir();
            dst.push("signatures.txt");
            download_signature_probabilities(&dst).unwrap();
            dst
        });

    let child = Command::new("Rscript")
        .arg("/app/src/r/mutational_patterns.R")
        .arg(vcfs_dir.as_ref())
        .arg(sample_sheet.as_ref())
        .arg(cancer_signatures)
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
    let download_signatures_cmd = SubCommand::with_name("download-signatures")
        .about("Downloads and preprocesses a COSMIC mutational signatures table")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("file")
            .help("Output pathname")
            .required(true));

    let run_cmd = SubCommand::with_name("run")
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
            .index(2));

    let split_vcf_cmd = SubCommand::with_name("split-vcf")
        .about("Splits a multi-sample VCF to multiple single-sample VCFs")
        .arg(Arg::with_name("output-directory")
            .short("o")
            .long("output-directory")
            .value_name("directory")
            .help("Results directory")
            .required(true))
        .arg(Arg::with_name("input")
            .value_name("file")
            .help("Input multi-sample VCF")
            .required(true)
            .index(1));

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .subcommand(download_signatures_cmd)
        .subcommand(run_cmd)
        .subcommand(split_vcf_cmd)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("download-signatures") {
        let dst = matches.value_of("output").unwrap();
        download_signature_probabilities(dst).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let vcfs_dir = matches.value_of("vcfs-dir").unwrap();
        let sample_sheet = matches.value_of("sample-sheet").unwrap();
        let cancer_signatures = matches.value_of("cancer-signatures");
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
    } else if let Some(matches) = matches.subcommand_matches("split-vcf") {
        let src = matches.value_of("input").unwrap();
        let dst = matches.value_of("output-directory").unwrap();
        split_file(src, dst).unwrap();
    }
}
