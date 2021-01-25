pub mod http;
pub mod s3;

use std::io::Result;
use bytes::Bytes;
use futures::Stream;

type BodyStream = Box<dyn Stream<Item = Result<Bytes>> + Send + Sync + Unpin>;