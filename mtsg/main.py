from pathlib import Path
import argparse

from sigproSS import spss


def main() -> None:
    parser = argparse.ArgumentParser(allow_abbrev=False)

    parser.add_argument("--dst-prefix", type=Path)
    parser.add_argument("--genome-build", choices=["GRCh38"], default="GRCh38")
    parser.add_argument("src_prefix", metavar="src-prefix", type=Path)

    args = parser.parse_args()

    src_prefix: Path = args.src_prefix
    dst_prefix: Path = args.dst_prefix
    genome_build: str = args.genome_build

    spss.single_sample(
        str(src_prefix),
        str(dst_prefix),
        ref=genome_build,
        exome=False,
    )


if __name__ == "__main__":
    main()
