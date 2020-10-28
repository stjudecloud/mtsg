from pathlib import Path
import argparse

from mtsg import GenomeBuild
from mtsg.commands import run, visualize
import mtsg


def main() -> None:
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument(
        "--version", action="version", version="%(prog)s {}".format(mtsg.__version__)
    )

    subparsers = parser.add_subparsers(
        title="subcommands", dest="subcommand", required=True
    )

    run_cmd = subparsers.add_parser("run")
    run_cmd.add_argument("--dst-prefix", type=Path, required=True)

    run_cmd.add_argument(
        "--genome-build",
        choices=GenomeBuild.__members__.values(),
        default=GenomeBuild.GRCH38,
        type=GenomeBuild.parse,
    )

    run_cmd.add_argument("src_prefix", metavar="src-prefix", type=Path)

    visualize_cmd = subparsers.add_parser("visualize")
    visualize_cmd.add_argument("--reference", type=Path, required=True)
    visualize_cmd.add_argument("--output", type=Path, required=True)
    visualize_cmd.add_argument("src", type=Path)

    args = parser.parse_args()
    cmd = args.subcommand

    if cmd == "run":
        src_prefix: Path = args.src_prefix
        dst_prefix: Path = args.dst_prefix
        genome_build: GenomeBuild = args.genome_build
        run(src_prefix, dst_prefix, genome_build=genome_build)
    elif cmd == "visualize":
        src: Path = args.src
        reference_src: Path = args.reference
        dst: Path = args.output
        visualize(src, reference_src, dst)
    else:
        raise ValueError("unreachable")


if __name__ == "__main__":
    main()
