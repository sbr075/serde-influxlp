pub(super) mod datatypes;
pub(super) mod io;
pub(super) mod slice;
pub(super) mod traits;

pub(crate) use io::IoReader;
pub(crate) use slice::SliceReader;
pub(crate) use traits::Reader;
