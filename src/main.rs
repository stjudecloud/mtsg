#[macro_use]
extern crate clap;
extern crate mutspec;
extern crate log;
extern crate env_logger;

use std::io;
use std::process;

use clap::{App, AppSettings, Arg, SubCommand};
use log::LevelFilter;

use mutspec::cosmic::download_signature_probabilities;
use mutspec::r::mutational_patterns;
use mutspec::sample_sheet;
use mutspec::vcf::split_file;
use mutspec::visualizations::create_visualization;

fn exit_with_io_error(error: io::Error) -> ! {
    eprintln!("{}", error);
    process::exit(1);
}

fn main() {
    let download_signatures_cmd = SubCommand::with_name("download-signatures")
        .about("Downloads and preprocesses known mutational signatures (COSMIC)")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("file")
            .help("Output pathname")
            .required(true));

    let generate_sample_sheet_cmd = SubCommand::with_name("generate-sample-sheet")
        .about("Generates a sample sheet from a directory of VCFs")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("file")
            .help("Output filename")
            .required(true))
        .arg(Arg::with_name("input-directory")
            .help("Input directory of single-sample VCFs")
            .required(true)
            .index(1));

    let run_cmd = SubCommand::with_name("run")
        .about("Finds the linear combination of mutation signatures that reconstructs the mutation matrix")
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
            .help("Input directory of single-sample VCFs")
            .required(true)
            .index(1))
        .arg(Arg::with_name("sample-sheet")
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
            .help("Input multi-sample VCF. Accepts both uncompressed and gzipped inputs.")
            .required(true)
            .index(1));

    let visualize_cmd = SubCommand::with_name("visualize")
        .about("Creates an interactive visualization for the given cancer signatures")
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("file")
            .help("Output pathname")
            .required(true))
        .arg(Arg::with_name("input")
            .help("Fitted mutation matrix to cancer signatures")
            .required(true)
            .index(1));

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Use verbose logging"))
        .subcommand(download_signatures_cmd)
        .subcommand(generate_sample_sheet_cmd)
        .subcommand(run_cmd)
        .subcommand(split_vcf_cmd)
        .subcommand(visualize_cmd)
        .get_matches();

    if matches.is_present("verbose") {
        env_logger::Builder::from_default_env()
            .filter(Some(crate_name!()), LevelFilter::Info)
            .init()
    }

    if let Some(matches) = matches.subcommand_matches("download-signatures") {
        let dst = matches.value_of("output").unwrap();
        download_signature_probabilities(dst).unwrap_or_else(|e| exit_with_io_error(e));
    } else if let Some(matches) = matches.subcommand_matches("generate-sample-sheet") {
        let src = matches.value_of("input-directory").unwrap();
        let dst = matches.value_of("output").unwrap();
        sample_sheet::generate(src, dst).unwrap_or_else(|e| exit_with_io_error(e));
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let vcfs_dir = matches.value_of("vcfs-dir").unwrap();
        let sample_sheet = matches.value_of("sample-sheet").unwrap();
        let cancer_signatures = matches.value_of("cancer-signatures");
        let genome_build = matches.value_of("genome-build").unwrap();
        let min_burden = value_t!(matches, "min-burden", u32).unwrap_or_else(|e| e.exit());
        let min_contribution = value_t!(matches, "min-contribution", u32).unwrap_or_else(|e| e.exit());
        let out_dir = matches.value_of("output-directory").unwrap();

        let result = mutational_patterns(
            vcfs_dir,
            sample_sheet,
            cancer_signatures,
            genome_build,
            min_burden,
            min_contribution,
            out_dir,
        );

        match result {
            Ok(status) => {
                if !status.success() {
                    let code = status.code().unwrap_or(1);
                    process::exit(code);
                }
            },
            Err(e) => exit_with_io_error(e),
        }
    } else if let Some(matches) = matches.subcommand_matches("split-vcf") {
        let src = matches.value_of("input").unwrap();
        let dst = matches.value_of("output-directory").unwrap();
        split_file(src, dst).unwrap_or_else(|e| exit_with_io_error(e));
    } else if let Some(matches) = matches.subcommand_matches("visualize") {
        let src = matches.value_of("input").unwrap();
        let dst = matches.value_of("output").unwrap();
        create_visualization(src, dst).unwrap_or_else(|e| exit_with_io_error(e));
    }
}
