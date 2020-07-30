use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Context;
use mtsg::cosmic;

pub fn download_signatures<P>(dst: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dst = dst.as_ref();
    let mut writer = File::create(dst)
        .map(BufWriter::new)
        .with_context(|| format!("Could not open file: {}", dst.display()))?;

    cosmic::download_signature_probabilities(&mut writer)?;

    Ok(())
}
