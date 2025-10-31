use time::{Duration, PrimitiveDateTime};

#[derive(Debug, Clone, Copy)]
pub enum ExMTtlValue {
    Duration(Duration),
    AtTime(PrimitiveDateTime),
    NoExp,
}
