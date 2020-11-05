from dataclasses import dataclass
from pathlib import Path
from typing import List
import csv
import sys


@dataclass
class Sample:
    name: str
    contributions: List[str]


src = Path(sys.argv[1])

signatures = []
samples = []

with src.open(newline="") as f:
    reader = csv.reader(f, delimiter="\t")

    headers = next(reader)
    signatures = [header for header in headers if header.startswith("Signature")]

    for row in reader:
        sample_name = row[0]
        contributions = row[1 : len(signatures) + 1]

        assert len(contributions) == len(signatures)

        sample = Sample(sample_name, contributions)
        samples.append(sample)

writer = csv.writer(sys.stdout, delimiter="\t", lineterminator="\n")

sample_names = [sample.name for sample in samples]
writer.writerow(["Samples"] + sample_names)

for i, signature in enumerate(signatures):
    row = [signature]

    for sample in samples:
        row.append(sample.contributions[i])

    writer.writerow(row)
