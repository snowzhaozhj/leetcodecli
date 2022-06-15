use std::path::{Path, PathBuf};
use std::ops::Range;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};


use serde::{Deserialize, Serialize};
use anyhow::Result;
use serde_json::Deserializer;

use crate::buf_util::{BufReaderWithPos, BufWriterWithPos};


/// log文件中存储的命令
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    pub(crate) fn set(key: String, value: String) -> Self {
        Command::Set { key, value }
    }

    pub(crate) fn remove(key: String) -> Self {
        Command::Remove { key }
    }
}


/// 用于索引Command，存放在内存中
pub(crate) struct CommandPos {
    pub(crate) id: u64,
    pub(crate) pos: u64,
    pub(crate) len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((id, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            id,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}


pub(crate) struct LogFile {
    id: u64,
    reader: Option<BufReaderWithPos<File>>,
    writer: Option<BufWriterWithPos<File>>,
}

impl LogFile {
    fn log_file_name<T: AsRef<Path>>(path: T, id: u64) -> PathBuf {
        path.as_ref().join(format!("{}.log", id))
    }

    pub(crate) fn create(id: u64, path: &Path) -> Result<LogFile> {
        let path = LogFile::log_file_name(path, id);
        let writer = Some(BufWriterWithPos::new(
            OpenOptions::new().create(true).append(true).open(&path)?,
        )?);
        let reader = Some(BufReaderWithPos::new(File::open(&path)?)?);
        Ok(LogFile { id, reader, writer })
    }

    fn load(id: u64, path: &Path) -> Result<LogFile> {
        let path = LogFile::log_file_name(path, id);
        let writer = None;
        let reader = Some(BufReaderWithPos::new(File::open(&path)?)?);
        Ok(LogFile { id, reader, writer })
    }

    pub(crate) fn load_all(path: &Path, log_files: &mut BTreeMap<u64, LogFile>) -> Result<()> {
        let mut ids: Vec<u64> = fs::read_dir(&path)?
            .flat_map(|dir_entry| -> Result<_>{ Ok(dir_entry?.path()) })
            .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
            .filter_map(|path| {
                path.file_stem()
                    .and_then(OsStr::to_str)
                    .map(str::parse::<u64>)
            })
            .flatten()
            .collect();
        ids.sort();
        for id in ids {
            log_files.insert(id, LogFile::load(id, path)?);
        }
        Ok(())
    }

    pub(crate) fn remove(id: u64, path: &Path) -> Result<()> {
        fs::remove_file(LogFile::log_file_name(path, id))?;
        Ok(())
    }

    /// # Return
    /// 旧数据的大小
    pub(crate) fn init_index(&mut self, index: &mut BTreeMap<String, CommandPos>) -> Result<u64> {
        let mut pos = self.reader.as_mut().unwrap().seek(SeekFrom::Start(0))?;
        let mut stream = Deserializer::from_reader(self.reader.as_mut().unwrap()).into_iter::<Command>();
        let mut stale_data: u64 = 0; // 旧数据
        while let Some(cmd) = stream.next() {
            let new_pos = stream.byte_offset() as u64;
            match cmd? {
                Command::Set { key, .. } => {
                    if let Some(old_cmd) = index.insert(key, CommandPos::from((self.id, pos..new_pos))) {
                        stale_data += old_cmd.len;
                    }
                }
                Command::Remove { key } => {
                    if let Some(old_cmd) = index.remove(&key) {
                        stale_data += old_cmd.len;
                    }
                    stale_data += new_pos - pos;
                }
            }
            pos = new_pos;
        }
        Ok(stale_data)
    }

    pub(crate) fn store(&mut self, cmd: Command) -> Result<()> {
        serde_json::to_writer(&mut self.writer.as_mut().unwrap(), &cmd)?;
        Ok(())
    }

    pub(crate) fn flush_write(&mut self) -> Result<()> {
        self.writer.as_mut().unwrap().flush()?;
        Ok(())
    }

    pub(crate) fn write_pos(&self) -> u64 {
        self.writer.as_ref().unwrap().pos
    }

    pub(crate) fn read_cmd(&mut self, cmd_pos: &CommandPos) -> Result<Command> {
        let reader = self.reader.as_mut().unwrap();
        reader.seek(SeekFrom::Start(cmd_pos.pos))?;
        let cmd: Command = serde_json::from_reader(reader.take(cmd_pos.len))?;
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap};
    use std::fs;
    use std::fs::{File};
    use std::path::{Path, PathBuf};
    use serial_test::serial;
    use crate::log_file::LogFile;

    struct RAIIFile {
        path: PathBuf,
    }

    impl RAIIFile {
        fn new<T: AsRef<Path>>(path: T) -> RAIIFile {
            File::create(path.as_ref()).expect(format!("create {} failed", path.as_ref().display()).as_str());
            RAIIFile {
                path: PathBuf::from(path.as_ref()),
            }
        }
    }

    impl Drop for RAIIFile {
        fn drop(&mut self) {
            fs::remove_file(self.path.as_path()).expect(format!("remove {} failed", self.path.display()).as_str());
        }
    }

    #[test]
    #[serial]
    fn test_log_all() {
        let _log1 = RAIIFile::new("1.log");
        let _log2 = RAIIFile::new("2.log");
        let _log4 = RAIIFile::new("4.log");
        let _log12 = RAIIFile::new("12.log");
        let _not_log1 = RAIIFile::new("1.lo");
        let _not_log1 = RAIIFile::new("ab.log");
        let mut log_files = BTreeMap::new();
        LogFile::load_all(".".as_ref(), &mut log_files).expect("load_all failed");
        assert_eq!(log_files.len(), 4);
        assert_eq!(log_files.contains_key(&1), true);
        assert_eq!(log_files.contains_key(&2), true);
        assert_eq!(log_files.contains_key(&4), true);
        assert_eq!(log_files.contains_key(&12), true);
    }
}
