use std::path::PathBuf;
use std::collections::BTreeMap;
use std::fs;

pub type Result<T> = anyhow::Result<T>;

use crate::log_file::{Command, CommandPos, LogFile};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

pub struct DataBase {
    path: PathBuf,
    current_id: u64,
    log_files: BTreeMap<u64, LogFile>,
    index: BTreeMap<String, CommandPos>,
    stale_data: u64,
}

impl DataBase {
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let mut log_files = BTreeMap::new();
        let mut index = BTreeMap::new();
        let mut stale_data = 0;
        LogFile::load_all(path.as_path(), &mut log_files)?;
        let current_id = log_files.keys().last().unwrap_or(&0) + 1;
        for (_, log_file) in &mut log_files {
            stale_data += log_file.init_index(&mut index)?;
        }
        log_files.insert(current_id, LogFile::create(current_id, &path)?);

        Ok(DataBase {
            path,
            current_id,
            log_files,
            index,
            stale_data,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key.clone(), value);
        let writer = self.log_files.iter_mut().next_back().unwrap().1;
        let pos = writer.write_pos();
        writer.store(cmd)?;
        writer.flush_write()?;
        let new_pos = writer.write_pos();
        // 更新索引
        if let Some(old_cmd) = self.index.insert(key, CommandPos::from((self.current_id, pos..new_pos))) {
            self.stale_data += old_cmd.len;
        }
        if self.stale_data > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(key) {
            let reader = self.log_files.get_mut(&cmd_pos.id)
                .expect("Cannot find reader");
            if let Command::Set { value, .. } = reader.read_cmd(cmd_pos)? {
                return Ok(Some(value));
            } else {
                return Err(anyhow::Error::msg("Unexpected command"));
            }
        }
        Ok(None)
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        if self.index.contains_key(key) {
            let cmd = Command::remove(key.to_string());
            let writer = self.log_files.iter_mut().next_back().unwrap().1;
            writer.store(cmd)?;
            writer.flush_write()?;
            let old_cmd = self.index.remove(key).expect("Key not found");
            self.stale_data += old_cmd.len;
            Ok(())
        } else {
            Err(anyhow::Error::msg("Key not found"))
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.index.contains_key(key)
    }

    fn compact(&mut self) -> Result<()> {
        let compaction_id = self.current_id + 1;
        let mut compaction_log_file = LogFile::create(compaction_id, &self.path)?;
        self.current_id += 2;
        let log_file = LogFile::create(self.current_id, &self.path)?;

        let mut new_pos = 0;
        for cmd_pos in &mut self.index.values_mut() {
            let reader = self.log_files.get_mut(&cmd_pos.id).expect("reader not found");
            let cmd = reader.read_cmd(&cmd_pos)?;
            compaction_log_file.store(cmd)?;
            *cmd_pos = CommandPos::from((compaction_id, new_pos..new_pos + cmd_pos.len));
            new_pos += cmd_pos.len;
        }

        for &id in self.log_files.keys() {
            LogFile::remove(id, &self.path)?;
        }
        self.log_files.clear();
        self.log_files.insert(compaction_id, compaction_log_file);
        self.log_files.insert(self.current_id, log_file);

        self.stale_data = 0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::database::DataBase;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_open() {
        DataBase::open("testdb").expect("open failed");
        fs::remove_dir_all("testdb").expect("remove dir failed");
    }

    #[test]
    #[serial]
    fn test_base() {
        let mut db = DataBase::open("testdb").expect("open failed");
        db.set("Hello".to_string(), "World".to_string()).expect("set failed");
        assert_eq!(db.contains("Hello"), true);
        assert_eq!(db.get("Hello").expect("get failed").unwrap(), "World".to_string());
        db.set("A".to_string(), "World".to_string()).expect("set failed");
        assert_eq!(db.contains("A"), true);
        assert_eq!(db.get("A").expect("get failed").unwrap(), "World".to_string());
        assert_eq!(db.contains("Hello"), true);
        assert_eq!(db.get("Hello").expect("get failed").unwrap(), "World".to_string());
        db.set("Hello".to_string(), "NewValue".to_string()).expect("set failed");
        assert_eq!(db.get("Hello").expect("get failed").unwrap(), "NewValue".to_string());
        db.remove("A").expect("remove failed");
        assert_eq!(db.contains("A"), false);
        fs::remove_dir_all("testdb").expect("remove dir failed");
    }
}
