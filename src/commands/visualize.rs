use std::{fs::File, path::Path};

use anyhow::Context;

use mtsg::visualizations::{read_table, write_html};

pub fn visualize<P, Q>(src: P, dst: Q) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let file =
        File::open(src).with_context(|| format!("Could not open file: {}", src.display()))?;

    let pathname = src.display().to_string();
    let (headers, samples) = read_table(file, &pathname)?;

    let dst = dst.as_ref();
    let mut file =
        File::create(dst).with_context(|| format!("Could not create file: {}", dst.display()))?;
    write_html(&mut file, &headers, &samples)?;

    Ok(())
}
