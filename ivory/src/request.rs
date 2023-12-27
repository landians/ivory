use crate::extensions::Extensions;
use crate::metadata::Metadata;

pub struct Request<T> {
    metadata: Metadata,
    body: T,
    extensions: Extensions,
}
