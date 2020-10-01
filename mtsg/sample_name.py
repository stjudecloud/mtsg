from typing import Optional
import re

SJID_V1_PATTERN = re.compile(
    r"(?x)SJ(?P<disease>[a-zA-Z0-9]*[a-zA-Z]\d?)(?P<number>\d{3})_(?P<type>[A-Z])(-(?P<secondary_id>.+))?"
)

SJID_V2_PATTERN = re.compile(
    r"(?x)SJ(?P<disease>[a-zA-Z0-9]*[a-zA-Z]\d?)(?P<number>\d{6})_(?P<type>[A-Z])(?P<index>\d+)(-(?P<secondary_id>.+))?"
)


class SampleName:
    disease: str
    number: int
    ty: str
    index: int
    secondary_id: Optional[str]

    @staticmethod
    def parse(s: str) -> "SampleName":
        if sample_name := parse_sjid_v2(s):
            return sample_name
        elif sample_name := parse_sjid_v1(s):
            return sample_name
        else:
            raise ValueError("invalid sample name: {}".format(s))

    def __init__(
        self,
        disease: str,
        number: int,
        ty: str,
        index: int,
        secondary_id: Optional[str] = None,
    ) -> None:
        self.disease = disease
        self.number = number
        self.ty = ty
        self.index = index
        self.secondary_id = secondary_id


def parse_sjid_v1(s: str) -> Optional[SampleName]:
    matches = re.match(SJID_V1_PATTERN, s)

    if not matches:
        return None

    disease = matches.group("disease")
    number = int(matches.group("number"))
    ty = matches.group("type")
    index = 0
    secondary_id = matches.group("secondary_id")

    return SampleName(disease, number, ty, index, secondary_id)


def parse_sjid_v2(s: str) -> Optional[SampleName]:
    matches = re.match(SJID_V2_PATTERN, s)

    if not matches:
        return None

    disease = matches.group("disease")
    number = int(matches.group("number"))
    ty = matches.group("type")
    index = int(matches.group("index"))
    secondary_id = matches.group("secondary_id")

    return SampleName(disease, number, ty, index, secondary_id)
