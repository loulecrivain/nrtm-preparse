#[cfg(test)]
mod tests;

use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "./grammar.pest"]
pub(crate) struct NRTMPreParser;
