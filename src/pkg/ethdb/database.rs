use redb::Error;
pub trait DBI {
    fn get(&self, table: String, key: &[u8]) -> Result<Vec<u8>, Error>;
    fn put(&mut self, table: String, key: &[u8], value: &[u8]) -> Result<bool, Error>;
}
