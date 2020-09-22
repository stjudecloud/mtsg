from pathlib import Path

from sigproSS import spss

from mtsg import GenomeBuild


def run(src_prefix: Path, dst_prefix: Path, genome_build: GenomeBuild) -> None:
    spss.single_sample(
        str(src_prefix),
        str(dst_prefix),
        ref=str(genome_build),
        exome=False,
    )
