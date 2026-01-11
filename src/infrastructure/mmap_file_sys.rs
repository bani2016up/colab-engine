use memmap2::{Mmap, MmapMut};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use crate::domain::traits::dyn_file::{DynemicFileRead, DynemicFileWrite};

pub struct MmapFileSystemSource {
    path: PathBuf,
    mmap: Option<Mmap>,
}

impl MmapFileSystemSource {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            path,
            mmap: Some(mmap),
        })
    }
}

impl DynemicFileRead for MmapFileSystemSource {
    fn get_slice(&self, start: usize, end: usize) -> String {
        if let Some(mmap) = &self.mmap {
            let slice = &mmap[start..end];
            String::from_utf8_lossy(slice).to_string()
        } else {
            String::new()
        }
    }

    fn get_content(&self) -> String {
        if let Some(mmap) = &self.mmap {
            String::from_utf8_lossy(&mmap[..]).to_string()
        } else {
            String::new()
        }
    }
}

impl DynemicFileWrite for MmapFileSystemSource {
    fn set_slice(&mut self, start: usize, end: usize, content: String) {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .expect("Failed to open file");

        let mut mmap = unsafe { MmapMut::map_mut(&file).expect("Failed to map file") };
        let bytes = content.as_bytes();
        mmap[start..start + bytes.len()].copy_from_slice(bytes);
        mmap.flush().expect("Failed to flush");
    }

    fn set_content(&mut self, content: String) {
        std::fs::write(&self.path, content).expect("Failed to write file");
        let file = File::open(&self.path).expect("Failed to open file");
        self.mmap = Some(unsafe { Mmap::map(&file).expect("Failed to map file") });
    }
}
