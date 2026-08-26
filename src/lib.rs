#[cfg(test)]
mod tests;

use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "./grammar.pest"]
pub(crate) struct NRTMPreParser;

#[derive(Debug)]
pub enum Verb {
    ADD,
    DEL,
}

#[derive(Debug)]
pub enum OpType {
    V2(Verb),
    V3(Verb, u64),
}

#[derive(Debug)]
pub struct NRTMMessage<'a> {
    pub update: OpType,
    pub rpsl: &'a str,
}

#[derive(Debug)]
pub enum ParseError {
    NoMatch,
    Parser(Error<Rule>),
    MalformedSerial(std::num::ParseIntError),
}

pub fn try_parse_message(pair: Pair<Rule>) -> Result<NRTMMessage, ParseError> {
    fn try_parse_serial_from(iter: &mut Pairs<Rule>) -> Result<u64, ParseError> {
        iter.next()
            .ok_or(ParseError::NoMatch)?
            .as_str()
            .parse()
            .map_err(ParseError::MalformedSerial)
    }

    match pair.as_rule() {
        Rule::v2_operation => {
            return try_parse_message(pair.into_inner().next().ok_or(ParseError::NoMatch)?);
        }
        Rule::v3_operation => {
            return try_parse_message(pair.into_inner().next().ok_or(ParseError::NoMatch)?);
        }
        Rule::v3_add_operation => {
            let mut inner_rules = pair.into_inner();
            let serial: u64 = try_parse_serial_from(&mut inner_rules)?;
            let rpsl = inner_rules.next().ok_or(ParseError::NoMatch)?.as_str();

            return Ok(NRTMMessage {
                update: OpType::V3(Verb::ADD, serial),
                rpsl,
            });
        }
        Rule::v3_del_operation => {
            let mut inner_rules = pair.into_inner();
            let serial: u64 = try_parse_serial_from(&mut inner_rules)?;
            let rpsl = inner_rules.next().ok_or(ParseError::NoMatch)?.as_str();

            return Ok(NRTMMessage {
                update: OpType::V3(Verb::DEL, serial),
                rpsl,
            });
        }
        Rule::v2_add_operation => {
            let mut inner_rules = pair.into_inner();
            let rpsl = inner_rules.next().ok_or(ParseError::NoMatch)?.as_str();

            return Ok(NRTMMessage {
                update: OpType::V2(Verb::ADD),
                rpsl,
            });
        }
        Rule::v2_del_operation => {
            let mut inner_rules = pair.into_inner();
            let rpsl = inner_rules.next().ok_or(ParseError::NoMatch)?.as_str();

            return Ok(NRTMMessage {
                update: OpType::V2(Verb::DEL),
                rpsl,
            });
        }
        _ => {}
    };

    Err(ParseError::NoMatch)
}
