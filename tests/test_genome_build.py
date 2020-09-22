import pytest

from mtsg import GenomeBuild


def test_parse() -> None:
    assert GenomeBuild.parse("GRCh37") == GenomeBuild.GRCH37
    assert GenomeBuild.parse("GRCh38") == GenomeBuild.GRCH38
    assert GenomeBuild.parse("mm9") == GenomeBuild.MM9
    assert GenomeBuild.parse("mm10") == GenomeBuild.MM10
    assert GenomeBuild.parse("rn6") == GenomeBuild.RN6

    with pytest.raises(ValueError):
        assert GenomeBuild.parse("")

    with pytest.raises(ValueError):
        assert GenomeBuild.parse("hg38")

    with pytest.raises(ValueError):
        assert GenomeBuild.parse("grch37")


def test___str__() -> None:
    assert str(GenomeBuild.GRCH37) == "GRCh37"
    assert str(GenomeBuild.GRCH38) == "GRCh38"
    assert str(GenomeBuild.MM9) == "mm9"
    assert str(GenomeBuild.MM10) == "mm10"
    assert str(GenomeBuild.RN6) == "rn6"
