use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Context;

use mtsg::sample_sheet;

pub fn generate_sample_sheet<P, Q>(src_prefix: P, dst: Q) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let dst = dst.as_ref();
    let mut writer = File::create(dst)
        .map(BufWriter::new)
        .with_context(|| format!("Could not create file: {}", dst.display()))?;

    sample_sheet::generate(src_prefix, &mut writer)?;

    Ok(())
}
