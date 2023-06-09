use crate::prelude::*;
use std::collections::VecDeque;
use std::fs::read_dir;

#[derive(Debug)]
pub struct DirEntry {
    pub path: String,
    pub file_name: String,
    pub file_type: String,
    pub is_dir: bool,
}

#[derive(Debug, Default)]
pub struct DirEntries {
    pub dir_entries: Option<VecDeque<DirEntry>>,
}

impl DirEntries {
    fn new() -> Self {
        Self {
            dir_entries: Some(VecDeque::new()),
        }
    }
    pub fn default() -> Self {
        Self::new()
    }
    pub fn print_dir_entries(&self) -> Result<()> {
        let dir_entries_str = String::try_from(W(self))?;
        debug!("{}", dir_entries_str);
        Ok(())
    }
}

impl TryFrom<W<&DirEntries>> for String {
    type Error = Error;
    fn try_from(dir_entries: W<&DirEntries>) -> Result<Self> {
        let mut dir_entries_str = String::new();
        for dir_entry in &dir_entries.0.dir_entries {
            for entry in dir_entry {
                dir_entries_str.push_str(&f!(
                    "\nPath: {}\nFile Name: {}\nFile Type: {}\nIs Dir: {}\n",
                    entry.path,
                    entry.file_name,
                    entry.file_type,
                    entry.is_dir
                ));
            }
        }
        Ok(dir_entries_str)
    }
}

impl TryFrom<W<&DirEntry>> for String {
    type Error = Error;
    fn try_from(dir_entry: W<&DirEntry>) -> Result<Self> {
        let dir_entry_str = f!(
            "\nPath: {}\nFile Name: {}\nFile Type: {}\nIs Dir: {}\n",
            dir_entry.0.path,
            dir_entry.0.file_name,
            dir_entry.0.file_type,
            dir_entry.0.is_dir
        );
        Ok(dir_entry_str)
    }
}

impl DirEntry {
    fn new(path: String, file_name: String, file_type: String, is_dir: bool) -> Result<Self> {
        Ok(Self {
            path,
            file_name,
            file_type,
            is_dir,
        })
    }
    pub fn default() -> Result<Self> {
        Self::new("".to_string(), "".to_string(), "".to_string(), false)
    }
    pub fn get_dirs(dir: &std::path::Path, dir_entries: &mut DirEntries) -> Result<Self> {
        if dir.is_dir() {
            let entries = read_dir(dir)?
                .map(|res| res.map(|e| e.path()))
                .collect::<std::io::Result<Vec<_>>>()?;

            for entry in entries {
                debug!("{:?}", entry);
                let file_name = entry.file_name().unwrap().to_str().ok_or_else(|| {
                    Error::IO(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        f!("Invalid Path {entry:?}"),
                    ))
                })?;
                let file_type = match entry.extension() {
                    Some(ext) => ext.to_str().unwrap().to_string(),
                    None => "".to_string(),
                };
                let is_dir = entry.is_dir();
                let entry: String = W(&entry).try_into()?;
                let dir_entry = Self::new(entry, file_name.to_string(), file_type, is_dir);
                let temp_default = Self::default().expect("Failed to get DirEntry");
                let temp_entry = dir_entry.unwrap_or(temp_default);
                temp_entry.print_dir_entry()?;
                dir_entries
                    .dir_entries
                    .as_mut()
                    .unwrap()
                    .push_back(temp_entry);
            }
        }
        Ok(Self::default().expect("Failed to get DirEntry"))
    }
    pub fn print_dir_entry(&self) -> Result<()> {
        let dir_entry: String = W(self).try_into()?;
        debug!("{}", dir_entry);
        Ok(())
    }
}
