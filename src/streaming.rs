use crate::{NRTMMessage, ParseError, try_parse_nrtmv2, try_parse_nrtmv3};
use tokio_util::bytes::BytesMut;
use tokio_util::codec::Decoder;

const MIN_BUFFER_LEN: usize = 8192;

#[derive(Clone)]
pub struct NRTMDec {
    parser: fn(&str) -> Result<NRTMMessage, ParseError>,
}

impl NRTMDec {
    fn new_v2() -> Self {
        NRTMDec {
            parser: try_parse_nrtmv2,
        }
    }

    fn new_v3() -> Self {
        NRTMDec {
            parser: try_parse_nrtmv3,
        }
    }
}

impl Decoder for NRTMDec {
    type Item = NRTMMessage;
    type Error = ParseError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        src.reserve(MIN_BUFFER_LEN);

        // from_utf8 is no-copy
        let str = String::from_utf8(src.to_vec()).map_err(ParseError::NonUTF8Input)?;

        match (self.parser)(str.as_str()) {
            Ok(message) => {
                let _message_bytes = src.split_to(message.span.end_b);
                // implicit drop for message_str and message_bytes
                Ok(Some(message))
            }
            // per tokio-util codec documentation,
            // If the bytes look valid, but a frame isn’t fully available yet,
            // then Ok(None) is returned.
            Err(ParseError::Incomplete) => Ok(None),
            Err(ParseError::LeadingGarbage(span)) => {
                // split garbage, flush and return err
                let _garbage_bytes = src.split_to(span.end_b);
                Err(ParseError::LeadingGarbage(span))
            }
            // malformed input, split at malformed, flush and return err
            // the rest will be eaten as leading garbage
            Err(ParseError::Parser(err)) => {
                let _malformed_bytes = match err.location {
                    // boop it! kick it! drop it!
                    pest::error::InputLocation::Pos(u) => src.split_to(u),
                    pest::error::InputLocation::Span((_, end)) => src.split_to(end),
                };
                Err(ParseError::Parser(err))
            }
            // malformed utf-8, flush incriminated section
            Err(ParseError::NonUTF8Input(u8)) => {
                let _utf8err_bytes = &mut src.split_to(u8.utf8_error().valid_up_to());
                Err(ParseError::NonUTF8Input(u8))
            }
            // malformed serial
            Err(ParseError::MalformedSerial(span, e)) => {
                let _garbage_bytes = src.split_to(span.end_b); // ibid
                Err(ParseError::MalformedSerial(span, e))
            }
            Err(ParseError::NoMatch) => Err(ParseError::NoMatch),
            Err(ParseError::IoError(ioe)) => {
                src.clear(); // clear buffer
                Err(ParseError::IoError(ioe))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::streaming::NRTMDec;
    use crate::{OpType, Verb};
    use std::assert_matches;
    use tokio::fs::File;
    use tokio_stream::StreamExt;
    use tokio_util::codec::FramedRead;

    #[tokio::test]
    async fn v3_message_stream_ok() {
        let nrtmv3_sample = File::open("./src/tests/nrtmv3_ripe_sample.txt")
            .await
            .unwrap();
        let decoder = NRTMDec::new_v3();
        let mut reader = FramedRead::new(nrtmv3_sample, decoder);

        let res = reader.next().await.unwrap().unwrap();
        assert_matches!(res.update, OpType::V3(Verb::ADD, 65776764));
        let res = reader.next().await.unwrap().unwrap();
        assert_matches!(res.update, OpType::V3(Verb::ADD, 65776765));
    }
}
