import pytest

from mtsg.commands.visualize import (
    normalize_signature_name,
    parse_header,
    Disease,
    ParseHeaderError,
)


def test_parse_header() -> None:
    actual = parse_header("SJSMPL000001_D1|SMPL")
    expected = ("SJSMPL000001_D1", Disease("SMPL"))
    assert actual == expected

    actual = parse_header("SJSMPL000001_D1")
    expected = ("SJSMPL000001_D1", Disease("Unknown"))
    assert actual == expected

    with pytest.raises(ParseHeaderError):
        parse_header("")


def test_normalize_signature_name() -> None:
    assert normalize_signature_name("Signature Subs-01") == "SBS1"
    assert normalize_signature_name("Signature Subs-07a") == "SBS7a"
    assert normalize_signature_name("Signature Subs-13") == "SBS13"

    with pytest.raises(ValueError):
        normalize_signature_name("")

    with pytest.raises(ValueError):
        normalize_signature_name("Signature Subs")
