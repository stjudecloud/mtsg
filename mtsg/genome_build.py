from enum import Enum


class GenomeBuild(Enum):
    GRCH37 = 0
    GRCH38 = 1
    MM9 = 2
    MM10 = 3
    RN6 = 4

    @staticmethod
    def parse(s: str) -> "GenomeBuild":
        if s == "GRCh37":
            return GenomeBuild.GRCH37
        elif s == "GRCh38":
            return GenomeBuild.GRCH38
        elif s == "mm9":
            return GenomeBuild.MM9
        elif s == "mm10":
            return GenomeBuild.MM10
        elif s == "rn6":
            return GenomeBuild.RN6
        else:
            raise ValueError("invalid genome build: '{}'".format(s))

    def __str__(self) -> str:
        if self == GenomeBuild.GRCH37:
            return "GRCh37"
        elif self == GenomeBuild.GRCH38:
            return "GRCh38"
        elif self == GenomeBuild.MM9:
            return "mm9"
        elif self == GenomeBuild.MM10:
            return "mm10"
        elif self == GenomeBuild.RN6:
            return "rn6"
        else:
            raise ValueError("unreachable")
