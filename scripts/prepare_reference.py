import csv
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Set, Tuple

HEADER_DELIMITER = "|"

sample_info_src = Path(sys.argv[1])
activities_srcs = [Path(src) for src in sys.argv[2:]]


@dataclass
class Disease:
    name: str


@dataclass
class Sample:
    name: str
    contributions: Dict[str, str] = field(default_factory=dict)


def normalize_sample_name(s: str) -> str:
    components = s.rsplit("_", 1)

    if len(components) < 2:
        raise ValueError("invalid sample name '{}'".format(s))

    return components[0]


def read_sample_info(src: Path) -> Dict[str, Disease]:
    sample_name_diseases = {}

    with src.open(newline="") as f:
        reader = csv.DictReader(f)

        for row in reader:
            sample_name = row["SampleID"]
            disease_name = row["new label"]
            sample_name_diseases[sample_name] = Disease(disease_name)

    return sample_name_diseases


def read_activities(src: Path) -> Tuple[Set[str], Dict[str, Sample]]:
    signatures = set()
    samples: Dict[str, Sample] = {}

    with src.open(newline="") as f:
        reader = csv.reader(f, delimiter="\t")
        headers = next(reader)

        sample_names = headers[1:]

        for row in reader:
            signature = row[0]
            contributions = row[1:]

            signatures.add(signature)

            for (sample_name, contribution) in zip(sample_names, contributions):
                if sample_name in samples:
                    sample = samples[sample_name]
                else:
                    sample = Sample(sample_name)
                    samples[sample_name] = sample

                sample.contributions[signature] = contribution

    return (signatures, samples)


sample_name_diseases = read_sample_info(sample_info_src)

signatures = set()
samples = {}

for src in activities_srcs:
    read_signatures, read_samples = read_activities(src)
    signatures.update(read_signatures)
    samples.update(read_samples)

sample_names = list(samples.keys())
sample_names.sort()

activated_signatures = list(signatures)
activated_signatures.sort()

prepared_headers = []
prepared_raw_sample_names = set()

for raw_sample_name in sample_names:
    sample_name = normalize_sample_name(raw_sample_name)

    if sample_name in sample_name_diseases:
        disease = sample_name_diseases[sample_name]

        header = "{}{}{}".format(
            sample_name,
            HEADER_DELIMITER,
            disease.name,
        )

        prepared_headers.append(header)
        prepared_raw_sample_names.add(raw_sample_name)
    else:
        print(
            "WARN: unknown sample name '{}'".format(sample_name),
            file=sys.stderr,
        )

writer = csv.writer(sys.stdout, delimiter="\t", lineterminator="\n")
writer.writerow(["Samples"] + prepared_headers)

for signature in activated_signatures:
    row = [signature]

    for raw_sample_name in sample_names:
        if raw_sample_name in prepared_raw_sample_names:
            sample = samples[raw_sample_name]

            if signature in sample.contributions:
                contribution = sample.contributions[signature]
                row.append(contribution)
            else:
                row.append("0")

    writer.writerow(row)
