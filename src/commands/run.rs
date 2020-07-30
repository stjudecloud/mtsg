use std::{path::Path, process};

use mtsg::r;

#[allow(clippy::too_many_arguments)]
pub fn run<P, Q, R, S>(
    vcfs_dir: P,
    sample_sheet: Q,
    cancer_signatures: Option<R>,
    genome_build: &str,
    min_burden: u32,
    min_contribution: u32,
    out_dir: S,
    prefix: &str,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    R: AsRef<Path>,
    S: AsRef<Path>,
{
    let result = r::mutational_patterns(
        vcfs_dir,
        sample_sheet,
        cancer_signatures,
        genome_build,
        min_burden,
        min_contribution,
        out_dir,
        prefix,
    );

    match result {
        Ok(status) => {
            if !status.success() {
                let code = status.code().unwrap_or(1);
                process::exit(code);
            }
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
