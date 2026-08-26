use crate::{NRTMPreParser, OpType, Rule, Verb};
use pest::Parser;
use std::assert_matches;

#[test]
fn parse_v2_add_msg() {
    let rpsl = "\
some: property
and: another one
with: double lf ending

";
    let nrtmv2 = format!(
        "\
ADD

{}
",
        rpsl
    );

    let pairs = NRTMPreParser::parse(Rule::v2_operation, nrtmv2.as_str())
        .expect("parsing nrtm message failed")
        .next()
        .unwrap();
    let parsed = crate::try_parse_message(pairs).unwrap();
    assert_matches!(parsed.update, OpType::V2(Verb::ADD));
    assert_eq!(parsed.rpsl, rpsl);
}

#[test]
fn parse_v3_add_msg() {
    let rpsl = "\
some: property
and: another one
with: double lf ending

";
    let nrtmv3 = format!(
        "\
ADD 666666

{}
",
        rpsl
    );

    let pairs = NRTMPreParser::parse(Rule::v3_operation, nrtmv3.as_str())
        .expect("parsing message failed")
        .next()
        .unwrap();
    let parsed = crate::try_parse_message(pairs).unwrap();
    assert_matches!(parsed.update, OpType::V3(Verb::ADD, 666_666));
    assert_eq!(parsed.rpsl, rpsl);
}
