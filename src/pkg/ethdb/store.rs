#![allow(dead_code)]

use crate::constant;
use redb::{Database, Error, ReadOnlyTable, ReadableTable, TableDefinition};

pub struct DB {
    path: String,
    handler: Database,
}

impl DB {
    pub fn new(path: String) -> Self {
        let db = Database::create(path.clone()).unwrap();
        DB {
            handler: db,
            path: path,
        }
    }
}

impl std::default::Default for DB {
    fn default() -> Self {
        let path = "testdb".to_string();
        let db = Database::create(path.clone()).unwrap();
        DB {
            handler: db,
            path: path,
        }
    }
}

impl std::fmt::Debug for DB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DB")
            .field("path", &self.path)
            .field("handler", &self.handler)
            .finish()
    }
}

impl DB {
    pub fn get(&self, table: String, key: &[u8]) -> Result<Vec<u8>, Error> {
        let tbl: TableDefinition<'_, &[u8], &[u8]> = TableDefinition::new(table.as_str());
        let dbr = self.handler.begin_read()?;
        let tbr: ReadOnlyTable<'_, &[u8], &[u8]> = dbr.open_table(tbl)?;
        let val = tbr.get(key)?.unwrap().value().to_vec();
        Ok(val)
    }

    pub fn put(&mut self, table: String, key: &[u8], value: &[u8]) -> Result<bool, Error> {
        let tbl: TableDefinition<'_, &[u8], &[u8]> = TableDefinition::new(table.as_str());
        let dbw = self.handler.begin_write()?;
        {
            let mut tbw = dbw.open_table(tbl)?;
            tbw.insert(key, value)?;
        }
        dbw.commit()?;

        Ok(true)
    }

    pub fn get_block_number(&self) -> Result<u64, Error> {
        let tbl: TableDefinition<'_, &[u8], &[u8]> = TableDefinition::new(constant::GLOBAL_TABLE);
        let dbr = self.handler.begin_read()?;
        let tbr: ReadOnlyTable<'_, &[u8], &[u8]> = dbr.open_table(tbl)?;
        let val = tbr
            .get(constant::LATEST_BLOCK.as_bytes())?
            .unwrap()
            .value()
            .to_vec();
        let val = String::from_utf8(val).unwrap().parse::<u64>().unwrap();
        Ok(val)
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
