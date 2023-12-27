use crate::extensions::Extensions;
use crate::metadata::Metadata;

pub struct Response<T> {
    metadata: Metadata,
    body: T,
    extensions: Extensions,
}
