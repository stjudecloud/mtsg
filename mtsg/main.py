from pathlib import Path
import argparse
import csv
import json
import re

from sigproSS import spss
import jinja2

import mtsg


def run(src_prefix: Path, dst_prefix: Path, genome_build: str = "GRCh38") -> None:
    spss.single_sample(
        str(src_prefix),
        str(dst_prefix),
        ref=genome_build,
        exome=False,
    )


def visualize(src: Path, dst: Path) -> None:
    headers = []
    samples = []

    with open(src, newline="") as f:
        reader = csv.reader(f, delimiter="\t")

        csv_headers = next(reader, None)

        if not csv_headers:
            raise ValueError("missing headers")

        sample_names = csv_headers[1:]

        for sample_name in sample_names:
            matches = re.match("SJ([A-Z]+).+", sample_name)
            disease = matches.group(1)

            sample = {
                "id": sample_name,
                "disease": disease,
                "contributions": [],
            }

            samples.append(sample)

        for row in reader:
            header = row[0]
            headers.append(header)

            contributions = row[1:]

            for i, raw_contribution in enumerate(contributions):
                sample = samples[i]
                contribution = int(raw_contribution)
                sample["contributions"].append(contribution)

    data = {
        "data": {
            "headers": headers,
            "samples": samples,
        }
    }

    generator = "mtsg {}".format(mtsg.__version__)
    payload = json.dumps(data)
    diseases = list(set(sample["disease"] for sample in samples))

    env = jinja2.Environment(loader=jinja2.PackageLoader("mtsg", "templates"))
    template = env.get_template("signatures.html.j2")

    with open(dst, "w") as f:
        f.write(
            template.render(generator=generator, payload=payload, diseases=diseases)
        )


def main() -> None:
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument(
        "--version", action="version", version="%(prog)s {}".format(mtsg.__version__)
    )

    subparsers = parser.add_subparsers(
        title="subcommands", dest="subcommand", required=True
    )

    run_cmd = subparsers.add_parser("run")
    run_cmd.add_argument("--dst-prefix", type=Path)
    run_cmd.add_argument("--genome-build", choices=["GRCh38"], default="GRCh38")
    run_cmd.add_argument("src_prefix", metavar="src-prefix", type=Path)

    visualize_cmd = subparsers.add_parser("visualize")
    visualize_cmd.add_argument("--output", type=Path)
    visualize_cmd.add_argument("src", type=Path)

    args = parser.parse_args()
    cmd = args.subcommand

    if cmd == "run":
        src_prefix: Path = args.src_prefix
        dst_prefix: Path = args.dst_prefix
        genome_build: str = args.genome_build
        run(src_prefix, dst_prefix, genome_build=genome_build)
    elif cmd == "visualize":
        src: Path = args.src
        dst: Path = args.output
        visualize(src, dst)
    else:
        raise ValueError("unreachable")


if __name__ == "__main__":
    main()
