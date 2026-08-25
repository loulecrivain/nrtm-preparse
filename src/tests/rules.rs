use crate::{NRTMPreParser, Rule};
use pest::Parser;

#[test]
fn rpsl_attr() {
    NRTMPreParser::parse(Rule::rpsl_start_attr, "random: field").expect("rpsl start attr parse failed");
}

#[test]
fn rpsl_object() {
    let obj = "\
random: field
with-other: value
";

    NRTMPreParser::parse(Rule::rpsl_object, obj).expect("rpsl obj parse failed");
}

#[test]
fn rpsl_object_comments() {
    let obj = "\
random: field
# comment
with-other: value
comment: yes # another
";

    NRTMPreParser::parse(Rule::rpsl_object, obj).expect("rpsl obj parse failed");
}
