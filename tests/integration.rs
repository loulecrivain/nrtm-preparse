use nrtm_parser::{NRTMParser, NRTMV3Parser, OpType, Verb};
use std::assert_matches;

#[test]
fn parse_single_message_example() {
    let message = "\
ADD 65776764

inetnum:        213.178.11.0 - 213.178.11.255
netname:        PSVNEO-CUSTOMER-NET-BACKUP
country:        DE
admin-c:        DUMY-RIPE
tech-c:         DUMY-RIPE
status:         ASSIGNED PA
created:        2026-08-19T14:41:10Z
last-modified:  2026-08-19T14:41:10Z
source:         RIPE
mnt-by:         IMSC-MNT
remarks:        ****************************
remarks:        * THIS OBJECT IS MODIFIED
remarks:        * Please note that all data that is generally regarded as personal
remarks:        * data has been removed from this object.
remarks:        * To view the original object, please query the RIPE Database at:
remarks:        * http://www.ripe.net/whois
remarks:        ****************************

";

    let res = NRTMV3Parser::try_parse(message);
    let res = res.expect("parsing should have succeeded");
    assert_matches!(res.update, OpType::V3(Verb::ADD, _));
}
