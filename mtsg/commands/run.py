from pathlib import Path

from sigproSS import spss


def run(src_prefix: Path, dst_prefix: Path, genome_build: str = "GRCh38") -> None:
    spss.single_sample(
        str(src_prefix),
        str(dst_prefix),
        ref=genome_build,
        exome=False,
    )
