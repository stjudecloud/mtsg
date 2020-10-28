import csv
import sys
from pathlib import Path
from typing import Dict

HEADER_DELIMITER = "|"

sample_info_src = Path(sys.argv[1])
activities_src = Path(sys.argv[2])


def read_sample_info(src: Path) -> Dict[str, str]:
    sample_disease_codes = {}

    with src.open(newline="") as f:
        reader = csv.DictReader(f)

        for row in reader:
            sample_name = row["sample_name"]
            disease_code = row["sj_diseases"]
            sample_disease_codes[sample_name] = disease_code

    return sample_disease_codes


sample_disease_codes = read_sample_info(sample_info_src)

with activities_src.open(newline="") as f:
    reader = csv.reader(f, delimiter="\t")
    headers = next(reader)

    sample_names = headers[1:]
    prepared_headers = [headers[0]]

    for sample_name in sample_names:
        if sample_name in sample_disease_codes:
            disease_code = sample_disease_codes[sample_name]
            header = "{}{}{}".format(sample_name, HEADER_DELIMITER, disease_code)
            prepared_headers.append(header)
        else:
            print("WARN: unknown sample name '{}'".format(sample_name), file=sys.stderr)
            prepared_headers.append(sample_name)

    writer = csv.writer(sys.stdout, delimiter="\t", lineterminator="\n")
    writer.writerow(prepared_headers)

    for row in reader:
        writer.writerow(row)
