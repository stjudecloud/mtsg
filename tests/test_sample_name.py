from mtsg import SampleName, sample_name


def test_sample_name_for_sjid_v1() -> None:
    sample_name = SampleName.parse("SJETV047_S")
    assert sample_name.disease == "ETV"
    assert sample_name.number == 47
    assert sample_name.ty == "S"
    assert sample_name.index == 2
    assert sample_name.secondary_id == None

    sample_name = SampleName.parse("SJETV047_S-TB-00-0000")
    assert sample_name.disease == "ETV"
    assert sample_name.number == 47
    assert sample_name.ty == "S"
    assert sample_name.index == 2
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


def test_parse_index_from_type() -> None:
    assert sample_name.parse_index_from_type("G") == 1
    assert sample_name.parse_index_from_type("D") == 1
    assert sample_name.parse_index_from_type("X") == 1
    assert sample_name.parse_index_from_type("A") == 1
    assert sample_name.parse_index_from_type("M") == 1
    assert sample_name.parse_index_from_type("O") == 1
    assert sample_name.parse_index_from_type("R") == 1
    assert sample_name.parse_index_from_type("C") == 1

    assert sample_name.parse_index_from_type("Y") == 2
    assert sample_name.parse_index_from_type("S") == 2
    assert sample_name.parse_index_from_type("E") == 2
    assert sample_name.parse_index_from_type("B") == 2
    assert sample_name.parse_index_from_type("H") == 2

    assert sample_name.parse_index_from_type("Z") == 3
    assert sample_name.parse_index_from_type("F") == 3
    assert sample_name.parse_index_from_type("T") == 3
    assert sample_name.parse_index_from_type("I") == 3

    assert sample_name.parse_index_from_type("Q") == None
