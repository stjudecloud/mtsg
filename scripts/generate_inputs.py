from pathlib import Path
import re
import sys

import pandas

SHEET_NAME = "SBS_GRCh37"
SIGNATURES_FILENAME = "genomeSignatures.txt"
SIGNATURE_NAMES_FILENAME = "signaturesSet.txt"

SIGNATURE_NAME_PATTERN = re.compile(r"SBS(\d+)(.)?")
NORMALIZED_SIGNATURE_NAME_PREFIX = "Signature Subs-"


def normalize_name(s: str) -> str:
    matches = SIGNATURE_NAME_PATTERN.match(s)

    if not matches:
        return s

    index = int(matches[1])

    if matches[2]:
        suffix = matches[2]
    else:
        suffix = ""

    return "{}{:02d}{}".format(NORMALIZED_SIGNATURE_NAME_PREFIX, index, suffix)


src = Path(sys.argv[1])
dst_prefix = Path(sys.argv[2])

signatures_dst = dst_prefix / SIGNATURES_FILENAME
signature_names_dst = dst_prefix / SIGNATURE_NAMES_FILENAME

df = pandas.read_excel(io=src, sheet_name=SHEET_NAME)
df.columns = [normalize_name(name) for name in df.columns]

signatures = df.iloc[:, 2:]
signatures.to_csv(signatures_dst, sep="\t", header=False, index=False)

names = df.columns[2:]

with open(signature_names_dst, "w") as f:
    for i, name in enumerate(names):
        f.write(name)

        if i < len(names) - 1:
            f.write("\n")
