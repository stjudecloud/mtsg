use std::{io, path::Path};

use log::warn;
use mtsg::vcf;

pub fn split_vcf<P, Q>(
    srcs: &[P],
    dst_prefix: Q,
    disable_column: Option<usize>,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    for src in srcs {
        match vcf::split_file(&src, &dst_prefix, disable_column) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                warn!(
                    "{}: invalid VCF (unexpected EOF), skipping",
                    src.as_ref().display()
                );
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
