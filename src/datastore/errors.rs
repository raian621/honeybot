#[derive(Debug, PartialEq)]
pub enum Error {
    DatabaseEntryNotFound,
    DatabaseUnexpectedErr,
    CacheEntryNotFound,
}
