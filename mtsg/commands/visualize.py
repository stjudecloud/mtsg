from pathlib import Path
import csv
import json
import re

import jinja2


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
