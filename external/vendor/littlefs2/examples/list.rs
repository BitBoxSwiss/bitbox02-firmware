use std::{
    env,
    fs::File,
    io::{Read as _, Seek as _, SeekFrom},
};

use littlefs2::{
    consts::{U1, U512},
    driver::Storage,
    fs::{Allocation, FileType, Filesystem},
    io::{Error, Result},
    object_safe::DynFilesystem,
    path::{Path, PathBuf},
};

const BLOCK_COUNT: usize = 128;
const BLOCK_SIZE: usize = 512;

fn main() {
    let path = env::args().nth(1).expect("missing argument");
    let file = File::open(&path).expect("failed to open file");
    let metadata = file.metadata().expect("failed to query metadata");

    let expected_len = BLOCK_COUNT * BLOCK_SIZE;
    let actual_len = usize::try_from(metadata.len()).unwrap();

    assert_eq!(actual_len % BLOCK_COUNT, 0);

    let mut s = FileStorage {
        file,
        len: actual_len,
    };
    let mut alloc = Allocation::new();
    let fs = Filesystem::mount(&mut alloc, &mut s).expect("failed to mount filesystem");

    let available_blocks = fs.available_blocks().unwrap();
    println!("expected_len:     {expected_len}");
    println!("actual_len:       {actual_len}");
    println!("available_blocks: {available_blocks}");
    println!();

    let path = PathBuf::new();
    list(&fs, &path);
}

fn list(fs: &dyn DynFilesystem, path: &Path) {
    fs.read_dir_and_then(path, &mut |iter| {
        for entry in iter {
            let entry = entry.unwrap();
            match entry.file_type() {
                FileType::File => println!("F {}", entry.path()),
                FileType::Dir => match entry.file_name().as_str() {
                    "." => (),
                    ".." => (),
                    _ => {
                        println!("D {}", entry.path());
                        list(fs, entry.path());
                    }
                },
            }
        }
        Ok(())
    })
    .unwrap()
}

struct FileStorage {
    file: File,
    len: usize,
}

impl Storage for FileStorage {
    type CACHE_SIZE = U512;
    type LOOKAHEAD_SIZE = U1;

    const READ_SIZE: usize = 16;
    const WRITE_SIZE: usize = 512;
    const BLOCK_SIZE: usize = BLOCK_SIZE;
    const BLOCK_COUNT: usize = BLOCK_COUNT;

    fn read(&mut self, off: usize, buf: &mut [u8]) -> Result<usize> {
        assert!(off + buf.len() <= BLOCK_SIZE * BLOCK_COUNT);
        if off >= self.len {
            // blocks that are not in the file are assumed to be empty
            buf.iter_mut().for_each(|byte| *byte = 0);
            Ok(buf.len())
        } else {
            self.file
                .seek(SeekFrom::Start(off.try_into().unwrap()))
                .map_err(|_| Error::IO)?;
            self.file.read(buf).map_err(|_| Error::IO)
        }
    }

    fn write(&mut self, _off: usize, _data: &[u8]) -> Result<usize> {
        unimplemented!("read-only filesystem");
    }

    fn erase(&mut self, _off: usize, _len: usize) -> Result<usize> {
        unimplemented!("read-only filesystem");
    }
}
