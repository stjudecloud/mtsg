from pathlib import Path
from typing import List
import csv
import json
import re

import jinja2

from mtsg import SampleName
import mtsg


class Sample:
    id: str
    contributions: List[int]

    def __init__(self, id: str) -> None:
        self.id = id
        self.contributions = []

    def disease(self) -> str:
        try:
            sample_name = SampleName.parse(self.id)
            return sample_name.disease
        except ValueError:
            return ""


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
            sample = Sample(sample_name)
            samples.append(sample)

        for row in reader:
            header = row[0]
            headers.append(header)

            contributions = row[1:]

            for i, raw_contribution in enumerate(contributions):
                sample = samples[i]
                contribution = int(raw_contribution)
                sample.contributions.append(contribution)

    prepared_samples = []

    for sample in samples:
        prepared_samples.append(
            {
                "id": sample.id,
                "disease": sample.disease(),
                "contributions": sample.contributions,
            }
        )

    data = {"data": {"headers": headers, "samples": prepared_samples}}

    generator = "mtsg {}".format(mtsg.__version__)
    payload = json.dumps(data)

    diseases = list(set(sample.disease() for sample in samples))
    diseases.sort()

    env = jinja2.Environment(loader=jinja2.PackageLoader("mtsg", "templates"))
    template = env.get_template("signatures.html.j2")

    with open(dst, "w") as f:
        f.write(
            template.render(generator=generator, payload=payload, diseases=diseases)
        )
