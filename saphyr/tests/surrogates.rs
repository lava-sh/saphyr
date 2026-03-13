use saphyr::{LoadableYamlNode, Yaml};

#[test]
fn json_surrogate_pair_parses_as_single_unicode_scalar() {
    let doc = Yaml::load_from_str("a: \"\\uD834\\uDD1E\"\n")
        .unwrap()
        .pop()
        .unwrap();

    let value = doc["a"].as_str().unwrap();
    assert_eq!(value, "𝄞");
    assert_eq!(value.chars().count(), 1);
    assert_eq!(value.as_bytes(), &[0xF0, 0x9D, 0x84, 0x9E]);
}

#[test]
fn json_surrogate_pair_matches_yaml_u_escape() {
    let from_json = Yaml::load_from_str("a: \"\\uD834\\uDD1E\"\n")
        .unwrap()
        .pop()
        .unwrap();
    let from_yaml = Yaml::load_from_str("a: \"\\U0001D11E\"\n")
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(from_json, from_yaml);
    assert_eq!(from_json["a"].as_str().unwrap(), "𝄞");
}

#[test]
fn rejects_unpaired_high_surrogate() {
    let err = Yaml::load_from_str("a: \"\\uD834\"\n").unwrap_err();
    let _ = err;
}

#[test]
fn rejects_unpaired_low_surrogate() {
    let err = Yaml::load_from_str("a: \"\\uDD1E\"\n").unwrap_err();
    let _ = err;
}

#[test]
fn rejects_reversed_surrogate_pair() {
    let err = Yaml::load_from_str("a: \"\\uDD1E\\uD834\"\n").unwrap_err();
    let _ = err;
}