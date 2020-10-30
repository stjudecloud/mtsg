from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
import csv
import json
import re

import jinja2

import mtsg

HEADER_DELIMITER = "|"

MAX_SIGNATURE_NAME_COMPONENTS = 2
SIGNATURE_NAME_DELIMITER = "-"
SIGNATURE_NAME_PREFIX = "SBS"


class Disease:
    code: str
    name: str

    def __init__(self, code: str, name: str) -> None:
        self.code = code
        self.name = name


class Sample:
    name: str
    disease: Disease
    contributions: Dict[str, int]

    def __init__(self, name: str, disease: Disease) -> None:
        self.name = name
        self.disease = disease
        self.contributions = {}


def parse_header(s: str) -> Tuple[str, Disease]:
    components = s.split(HEADER_DELIMITER, 2)

    sample_name = components[0]

    if len(components) < 3:
        disease = Disease("UNKNOWN", "Unknown")
    else:
        disease = Disease(components[1], components[2])

    return (sample_name, disease)


def normalize_signature_name(s: str) -> str:
    components = s.split(SIGNATURE_NAME_DELIMITER, MAX_SIGNATURE_NAME_COMPONENTS)

    if len(components) < MAX_SIGNATURE_NAME_COMPONENTS:
        raise ValueError("invalid signature name: {}".format(s))

    position = components[1].lstrip("0")

    return "{}{}".format(SIGNATURE_NAME_PREFIX, position)


def read_signature_activities(src: Path) -> Tuple[List[str], List[Sample]]:
    signatures = []
    samples = []

    with open(src, newline="") as f:
        reader = csv.reader(f, delimiter="\t")

        headers = next(reader, None)

        if not headers:
            raise ValueError("missing headers")

        for header in headers[1:]:
            sample_name, disease = parse_header(header)
            sample = Sample(sample_name, disease)
            samples.append(sample)

        for row in reader:
            signature = row[0]
            signatures.append(signature)

            contributions = row[1:]

            for i, raw_contribution in enumerate(contributions):
                sample = samples[i]
                contribution = int(raw_contribution)
                sample.contributions[signature] = contribution

    return (signatures, samples)


def normalize_samples(
    signatures: List[str], raw_samples: List[Sample]
) -> List[Dict[str, Any]]:
    samples = []

    for sample in raw_samples:
        contributions = []

        for signature in signatures:
            if signature in sample.contributions:
                contributions.append(sample.contributions[signature])
            else:
                contributions.append(0)

        samples.append(
            {
                "name": sample.name,
                "diseaseCode": sample.disease.name,
                "contributions": contributions,
            }
        )

    return samples


def normalize_data(
    reference_signatures: List[str],
    raw_reference_samples: List[Sample],
    query_signatures: List[str],
    raw_query_samples: List[Sample],
) -> Tuple[List[str], List[Dict[str, Any]], List[Dict[str, Any]]]:
    signatures = list(set(reference_signatures + query_signatures))
    signatures.sort()

    reference_samples = normalize_samples(signatures, raw_reference_samples)
    query_samples = normalize_samples(signatures, raw_query_samples)

    signatures = [normalize_signature_name(signature) for signature in signatures]

    return (signatures, reference_samples, query_samples)


def visualize(src: Path, reference_src: Path, dst: Path) -> None:
    reference_signatures, raw_reference_samples = read_signature_activities(
        reference_src
    )
    query_signatures, raw_query_samples = read_signature_activities(src)

    signatures, reference_samples, query_samples = normalize_data(
        reference_signatures, raw_reference_samples, query_signatures, raw_query_samples
    )

    data = {
        "data": {
            "signatures": signatures,
            "reference": reference_samples,
            "query": query_samples,
        }
    }

    generator = "mtsg {}".format(mtsg.__version__)
    payload = json.dumps(data)

    diseases = list(set(sample["diseaseCode"] for sample in reference_samples))
    diseases.sort()

    env = jinja2.Environment(loader=jinja2.PackageLoader("mtsg", "templates"))
    template = env.get_template("signatures.html.j2")

    with open(dst, "w") as f:
        f.write(
            template.render(generator=generator, payload=payload, diseases=diseases)
        )
