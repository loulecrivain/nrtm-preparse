use crate::{NRTMPreParser, OpType, ParseError, Rule, Verb};
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

{}",
        rpsl
    );

    let pairs = NRTMPreParser::parse(Rule::v2_operation, nrtmv2.as_str())
        .expect("parsing nrtm message failed")
        .next()
        .unwrap();
    let parsed = crate::try_parse_message(pairs).unwrap();
    assert_matches!(parsed.update, OpType::V2(Verb::ADD));
    assert_eq!(parsed.rpsl, rpsl);
    // check that span is correct
    assert_eq!(parsed.span.start_b, 0);
    assert_eq!(parsed.span.end_b, nrtmv2.len());
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

{}",
        rpsl
    );

    let pairs = NRTMPreParser::parse(Rule::v3_operation, nrtmv3.as_str())
        .expect("parsing message failed")
        .next()
        .unwrap();
    let parsed = crate::try_parse_message(pairs).unwrap();
    assert_matches!(parsed.update, OpType::V3(Verb::ADD, 666_666));
    assert_eq!(parsed.rpsl, rpsl);
    // check that span is correct
    assert_eq!(parsed.span.start_b, 0);
    assert_eq!(parsed.span.end_b, nrtmv3.len());
}

#[test]
fn check_spans() {
    let nrtmv3 = "\
ADD 666666

some: property
and: another one
with: double lf ending

ADD 666667

some: property
and: another one
with: double lf ending

";

    let res = crate::try_parse_nrtmv3(nrtmv3).unwrap();
    // check that span is correct
    assert_eq!(res.span.start_b, nrtmv3.find("ADD").unwrap());
    assert_eq!(res.span.end_b, nrtmv3.rfind("ADD").unwrap());
}

#[test]
fn signal_incomplete_nrtmv3() {
    let incomplete_nrtmv3 = "\
ADD 666666

some: property
and: another one
# keine endlich
# kei
";

    let res = crate::try_parse_nrtmv3(incomplete_nrtmv3);

    assert_matches!(res, Err(ParseError::Incomplete));
}

#[test]
fn signal_incomplete_nrtmv2() {
    let incomplete_nrtmv2 = "\
ADD

some: property
and: another one
# keine endlich
# kei
";

    let res = crate::try_parse_nrtmv2(incomplete_nrtmv2);

    assert_matches!(res, Err(ParseError::Incomplete));
}

#[test]
fn signal_no_match() {
    let no_match_nrtmv2 = "\
lsdjkflkjsdlfkjsldkjf;lsd
sdlkfjsdlkjfljsdf
sdlfjlsdkf

";

    let res = crate::try_parse_nrtmv3(no_match_nrtmv2);

    assert_matches!(res, Err(ParseError::NoMatch));
}
