import csv
import sys
from pathlib import Path
from typing import Dict

HEADER_DELIMITER = "|"

sample_info_src = Path(sys.argv[1])
activities_srcs = [Path(src) for src in sys.argv[2:]]


class Disease:
    name: str
    code: str

    def __init__(self, name: str, code: str) -> None:
        self.name = name
        self.code = code


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
            sample_name = row["sample_name"]
            disease_name = row["sj_long_disease_name"]
            disease_code = row["sj_diseases"]
            sample_name_diseases[sample_name] = Disease(disease_name, disease_code)

    return sample_name_diseases


sample_name_diseases = read_sample_info(sample_info_src)

prepared_headers = ["Samples"]
readers = []

for activities_src in activities_srcs:
    f = activities_src.open(newline="")
    reader = csv.reader(f, delimiter="\t")
    headers = next(reader)

    raw_sample_names = headers[1:]

    for raw_sample_name in raw_sample_names:
        sample_name = normalize_sample_name(raw_sample_name)

        if sample_name in sample_name_diseases:
            disease = sample_name_diseases[sample_name]
            header = "{}{}{}".format(sample_name, HEADER_DELIMITER, disease.name)
            prepared_headers.append(header)
        else:
            print(
                "WARN: unknown sample name '{}'".format(sample_name),
                file=sys.stderr,
            )
            prepared_headers.append(sample_name)

    readers.append(reader)

writer = csv.writer(sys.stdout, delimiter="\t", lineterminator="\n")
writer.writerow(prepared_headers)

for rdrs in zip(*readers):
    row_out = []

    for i, row_in in enumerate(rdrs):
        if i == 0:
            row_out.extend(row_in)
        else:
            row_out.extend(row_in[1:])

    writer.writerow(row_out)
