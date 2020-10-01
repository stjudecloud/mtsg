from mtsg import SampleName


def test_sample_name_for_sjid_v1() -> None:
    sample_name = SampleName.parse("SJETV047_S")
    assert sample_name.disease == "ETV"
    assert sample_name.number == 47
    assert sample_name.ty == "S"
    assert sample_name.index == 0
    assert sample_name.secondary_id == None

    sample_name = SampleName.parse("SJETV047_S-TB-00-0000")
    assert sample_name.disease == "ETV"
    assert sample_name.number == 47
    assert sample_name.ty == "S"
    assert sample_name.index == 0
    assert sample_name.secondary_id == "TB-00-0000"


def test_sample_name_for_sjid_v2() -> None:
    sample_name = SampleName.parse("SJETV000047_R2")
    assert sample_name.disease == "ETV"
    assert sample_name.number == 47
    assert sample_name.ty == "R"
    assert sample_name.index == 2
    assert sample_name.secondary_id == None

    sample_name = SampleName.parse("SJETV000047_R2-TB-00-0000")
    assert sample_name.disease == "ETV"
    assert sample_name.number == 47
    assert sample_name.ty == "R"
    assert sample_name.index == 2
    assert sample_name.secondary_id == "TB-00-0000"
