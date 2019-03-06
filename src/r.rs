use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

use log::info;

use crate::cosmic::download_signature_probabilities;

static MUTATIONAL_PATTERNS_SRC: &str = include_str!("mutational_patterns.R");

pub fn mutational_patterns<P, Q, R, S>(
    vcfs_dir: P,
    sample_sheet: Q,
    cancer_signatures: Option<R>,
    genome_build: &str,
    min_burden: u32,
    min_contribution: u32,
    out_dir: S,
    prefix: &str,
) -> io::Result<ExitStatus>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
    S: AsRef<Path>,
{
    let cancer_signatures = cancer_signatures
        .map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| {
            info!("using default COSMIC signature probabilities");

            let mut dst = env::temp_dir();
            dst.push("signatures.txt");
            download_signature_probabilities(&dst).unwrap();
            dst
        });

    info!("running mutational_patterns.R");
    info!("  genome-build = {}", genome_build);
    info!("  min-burden = {}", min_burden);
    info!("  min-contribution = {}", min_contribution);

    let mut child = Command::new("R")
        .arg("--vanilla")
        .arg("--slave")
        .arg("--args")
        .arg(vcfs_dir.as_ref())
        .arg(sample_sheet.as_ref())
        .arg(cancer_signatures)
        .arg(genome_build)
        .arg(min_burden.to_string())
        .arg(min_contribution.to_string())
        .arg(out_dir.as_ref())
        .arg(prefix)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(MUTATIONAL_PATTERNS_SRC.as_bytes())?;
    }

    {
        let stderr = child.stderr.as_mut().expect("Failed to open stderr");
        let reader = BufReader::new(stderr);

        for line in reader.lines().filter_map(Result::ok) {
            info!("R: {}", line);
        }
    }

    let result = child.wait();

    if let Ok(status) = result {
        if status.success() {
            info!("mutational_patterns.R exited (success)");
        } else {
            info!("mutational_patterns.R exited (fail)");
        }
    }

    result
}
