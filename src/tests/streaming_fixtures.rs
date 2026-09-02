use crate::streaming::NRTMDec;
use crate::{NRTMMessage, ParseError};
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;

pub fn v3_reader_from(
    bytes_slice: &[u8],
) -> impl StreamExt<Item = Result<NRTMMessage, ParseError>> {
    FramedRead::new(bytes_slice, NRTMDec::new_v3())
}
