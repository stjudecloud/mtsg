from pathlib import Path
import argparse

from sigproSS import spss


def run(src_prefix: Path, dst_prefix: Path, genome_build: str = "GRCh38") -> None:
    spss.single_sample(
        str(src_prefix),
        str(dst_prefix),
        ref=genome_build,
        exome=False,
    )


def main() -> None:
    parser = argparse.ArgumentParser(allow_abbrev=False)

    subparsers = parser.add_subparsers(
        title="subcommands", dest="subcommand", required=True
    )

    run_cmd = subparsers.add_parser("run")

    run_cmd.add_argument("--dst-prefix", type=Path)
    run_cmd.add_argument("--genome-build", choices=["GRCh38"], default="GRCh38")
    run_cmd.add_argument("src_prefix", metavar="src-prefix", type=Path)

    args = parser.parse_args()
    cmd = args.subcommand

    if cmd == "run":
        src_prefix: Path = args.src_prefix
        dst_prefix: Path = args.dst_prefix
        genome_build: str = args.genome_build
        run(src_prefix, dst_prefix, genome_build=genome_build)
    else:
        raise ValueError("unreachable")


if __name__ == "__main__":
    main()
