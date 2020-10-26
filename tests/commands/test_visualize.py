import pytest

from mtsg.commands.visualize import normalize_signature_name


def test_normalize_signature_name() -> None:
    assert normalize_signature_name("Signature Subs-01") == "SBS1"
    assert normalize_signature_name("Signature Subs-07a") == "SBS7a"
    assert normalize_signature_name("Signature Subs-13") == "SBS13"

    with pytest.raises(ValueError):
        normalize_signature_name("")

    with pytest.raises(ValueError):
        normalize_signature_name("Signature Subs")
