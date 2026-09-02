use crate::{PestNRTMParser, Rule};
use pest::Parser;

#[test]
fn rpsl_attr() {
    let mut iter = PestNRTMParser::parse(Rule::rpsl_start_attr, "random: field")
        .expect("rpsl start attr parse failed");
    let rpsl_start_pair = iter.next().unwrap();
    assert_eq!(rpsl_start_pair.as_rule(), Rule::rpsl_start_attr);
}

#[test]
fn rpsl_object() {
    let obj = "\
random: field
with-other: value

";

    let mut iter = PestNRTMParser::parse(Rule::rpsl_object, obj).expect("rpsl obj parse failed");
    let rpsl_object_pair = iter.next().unwrap();
    assert_eq!(rpsl_object_pair.as_rule(), Rule::rpsl_object);
}

#[test]
fn rpsl_object_comments() {
    let obj = "\
random: field
# comment
with-other: value
comment: yes # another
# ending-comment

";

    let mut iter = PestNRTMParser::parse(Rule::rpsl_object, obj).expect("rpsl obj parse failed");
    let rpsl_object_pair = iter.next().unwrap();
    assert_eq!(rpsl_object_pair.as_rule(), Rule::rpsl_object);
}

#[test]
#[should_panic]
fn rpsl_reject_malformed() {
    let obj = "\
object: with property
that-has: something
\\**xxt-with: it fields
# and a comment

";

    let mut iter = PestNRTMParser::parse(Rule::rpsl_object, obj).unwrap();
    let rpsl_object_pair = iter.next().unwrap();
    assert_eq!(rpsl_object_pair.as_rule(), Rule::rpsl_object);
}

#[test]
fn nrtmv2_update() {
    let update2 = "\
ADD

inet6num:       2a0f:6280:c825::/48
netname:        FUTURE-USAGE
descr:          Iridium
country:        KH
admin-c:        DUMY-RIPE
tech-c:         DUMY-RIPE
abuse-c:        ACRO56593-RIPE
status:         ASSIGNED
mnt-by:         HYEHOST-MNT
org:            ORG-IA2236-RIPE
created:        2026-08-19T14:41:34Z
last-modified:  2026-08-19T14:41:34Z
source:         RIPE
geofeed:        https://api.geofeed.space/fc79d4c7-ff20-4ed7-bc0a-1c44b19d377c/geofeed.csv
remarks:        ****************************
remarks:        * THIS OBJECT IS MODIFIED
remarks:        * Please note that all data that is generally regarded as personal
remarks:        * data has been removed from this object.
remarks:        * To view the original object, please query the RIPE Database at:
remarks:        * http://www.ripe.net/whois
remarks:        ****************************

";

    let mut iter =
        PestNRTMParser::parse(Rule::v2_operation, update2).expect("parsing nrtmv2 update failed");
    let nrtmv2_object_pair = iter.next().unwrap();
    assert_eq!(nrtmv2_object_pair.as_rule(), Rule::v2_operation);
}

#[test]
fn nrtmv3_update() {
    let update3 = "\
ADD 65776782

inet6num:       2a0f:6280:c825::/48
netname:        FUTURE-USAGE
descr:          Iridium
country:        KH
admin-c:        DUMY-RIPE
tech-c:         DUMY-RIPE
abuse-c:        ACRO56593-RIPE
status:         ASSIGNED
mnt-by:         HYEHOST-MNT
org:            ORG-IA2236-RIPE
created:        2026-08-19T14:41:34Z
last-modified:  2026-08-19T14:41:34Z
source:         RIPE
geofeed:        https://api.geofeed.space/fc79d4c7-ff20-4ed7-bc0a-1c44b19d377c/geofeed.csv
remarks:        ****************************
remarks:        * THIS OBJECT IS MODIFIED
remarks:        * Please note that all data that is generally regarded as personal
remarks:        * data has been removed from this object.
remarks:        * To view the original object, please query the RIPE Database at:
remarks:        * http://www.ripe.net/whois
remarks:        ****************************

";

    let mut iter =
        PestNRTMParser::parse(Rule::v3_operation, update3).expect("parsing nrtmv3 update failed");
    let nrtmv3_object_pair = iter.next().unwrap();
    assert_eq!(nrtmv3_object_pair.as_rule(), Rule::v3_operation);
}
