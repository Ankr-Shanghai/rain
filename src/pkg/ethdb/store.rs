#![allow(dead_code)]

use super::database::DBI;
use redb::{Database, Error, ReadOnlyTable, ReadableTable, TableDefinition};

struct DB {
    handler: Database,
}

impl DB {
    pub fn new(path: String) -> Self {
        let db = Database::create(path).unwrap();
        DB { handler: db }
    }
}

impl DBI for DB {
    fn get(&self, table: String, key: &[u8]) -> Result<Vec<u8>, Error> {
        let tbl: TableDefinition<'_, &[u8], &[u8]> = TableDefinition::new(table.as_str());
        let dbr = self.handler.begin_read()?;
        let tbr: ReadOnlyTable<'_, &[u8], &[u8]> = dbr.open_table(tbl)?;
        let val = tbr.get(key)?.unwrap().value().to_vec();
        Ok(val)
    }

    fn put(&mut self, table: String, key: &[u8], value: &[u8]) -> Result<bool, Error> {
        let tbl: TableDefinition<'_, &[u8], &[u8]> = TableDefinition::new(table.as_str());
        let dbw = self.handler.begin_write()?;
        {
            let mut tbw = dbw.open_table(tbl)?;
            tbw.insert(key, value)?;
        }
        dbw.commit()?;

        Ok(true)
    }
}

#[cfg(test)]
mod db_tests {
    #[test]
    fn dbtest() {
        use super::*;
        let mut db = DB::new("testdb".to_string());
        let key = "key".as_bytes();

        let value = "value---hello".as_bytes();
        {
            db.put("test".to_string(), key, value).unwrap();
        }
        let val = db.get("test".to_string(), key).unwrap();

        assert_eq!(val, value);
    }
}
