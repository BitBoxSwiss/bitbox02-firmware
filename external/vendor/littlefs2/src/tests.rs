use core::convert::TryInto;
use generic_array::typenum::consts;

use crate::{
    fs::{Allocation, Attribute, File, Filesystem},
    io::{Error, OpenSeekFrom, Read, Result, SeekFrom},
    path, BACKEND_VERSION, DISK_VERSION,
};

ram_storage!(
    name = OtherRamStorage,
    backend = OtherRam,
    erase_value = 0xff,
    read_size = 1,
    write_size = 32,
    cache_size_ty = consts::U32,
    block_size = 256,
    block_count = 512,
    lookahead_size_ty = consts::U1,
    filename_max_plus_one_ty = consts::U256,
    path_max_plus_one_ty = consts::U256,
);

ram_storage!(
    name = RamStorage,
    backend = Ram,
    erase_value = 0xff,
    read_size = 20 * 5,
    write_size = 20 * 7,
    cache_size_ty = consts::U700,
    block_size = 20 * 35,
    block_count = 32,
    lookahead_size_ty = consts::U16,
    filename_max_plus_one_ty = consts::U256,
    path_max_plus_one_ty = consts::U256,
);

#[cfg(feature = "unstable-littlefs-patched")]
ram_storage!(
    name = LargerRamStorage,
    backend = LargerRam,
    erase_value = 0xff,
    read_size = 20 * 5,
    write_size = 20 * 7,
    cache_size_ty = consts::U700,
    block_size = 20 * 35,
    block_count = 64,
    lookahead_size_ty = consts::U16,
    filename_max_plus_one_ty = consts::U256,
    path_max_plus_one_ty = consts::U256,
);

#[test]
fn version() {
    assert_eq!((BACKEND_VERSION.major(), BACKEND_VERSION.minor()), (2, 9));
    assert_eq!((DISK_VERSION.major(), DISK_VERSION.minor()), (2, 0));
}

#[test]
fn format() {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);
    let mut alloc = Filesystem::allocate();

    // should fail: FS is not formatted
    assert_eq!(
        Filesystem::mount(&mut alloc, &mut storage)
            .map(drop)
            .unwrap_err(),
        Error::CORRUPTION
    );
    // should succeed
    assert!(Filesystem::format(&mut storage).is_ok());
    // should succeed now that storage is formatted
    let _fs = Filesystem::mount(&mut alloc, &mut storage).unwrap();
    // check there are no segfaults
}

// #[macro_use]
// macro_rules! setup_fs {
//     () => {
//         let mut backend = OtherRam::default();
//         let mut storage = OtherRamStorage::new(&mut backend);
//         let mut alloc_fs = Filesystem::allocate();
//         Filesystem::format(&mut alloc_fs, &mut storage).unwrap();
//         let mut fs = Filesystem::mount(&mut alloc_fs, &mut storage).unwrap();
//     }
// }

#[test]
fn borrow_fs_allocation() {
    let mut backend = OtherRam::default();

    let mut storage = OtherRamStorage::new(&mut backend);
    let mut alloc_fs = Filesystem::allocate();
    Filesystem::format(&mut storage).unwrap();
    let _fs = Filesystem::mount(&mut alloc_fs, &mut storage).unwrap();
    // previous `_fs` is fine as it's masked, due to NLL
    let fs = Filesystem::mount(&mut alloc_fs, &mut storage).unwrap();

    fs.create_file_and_then(b"data.bin\0".try_into().unwrap(), |_| Ok(()))
        .unwrap();
    fs.create_file_and_then(b"data.bin\0".try_into().unwrap(), |_| Ok(()))
        .unwrap();
}

#[test]
fn borrow_fs_allocation2() {
    let mut backend = OtherRam::default();

    let mut storage = OtherRamStorage::new(&mut backend);
    let mut alloc_fs = Filesystem::allocate();
    Filesystem::format(&mut storage).unwrap();
    let _fs = Filesystem::mount(&mut alloc_fs, &mut storage).unwrap();
    // previous `_fs` is fine as it's masked, due to NLL

    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.create_file_and_then(b"data.bin\0".try_into().unwrap(), |_| Ok(()))
            .unwrap();
        fs.create_file_and_then(b"data.bin\0".try_into().unwrap(), |_| Ok(()))
            .unwrap();
        // where is boats when you need him lol
        Ok(())
    })
    .unwrap();
}

#[test]
fn borrow_fs_allocation3() {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |_| Ok(())).unwrap();

    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.create_file_and_then(b"data.bin\0".try_into().unwrap(), |_| Ok(()))
            .unwrap();
        fs.create_file_and_then(b"data.bin\0".try_into().unwrap(), |_| Ok(()))
            .unwrap();
        // where is boats when you need him lol
        Ok(())
    })
    .unwrap();
}

#[test]
fn test_fs_with() -> Result<()> {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        assert_eq!(fs.total_blocks(), 512);
        assert_eq!(fs.total_space(), 256 * 512);
        // superblock
        assert_eq!(fs.available_blocks()?, 512 - 2);
        assert_eq!(fs.available_space()?, 130_560);

        fs.create_dir(b"/tmp\0".try_into().unwrap())?;
        assert_eq!(fs.available_blocks()?, 512 - 4);
        fs.create_dir(b"/mnt\0".try_into().unwrap())?;
        assert_eq!(fs.available_blocks()?, 512 - 6);
        fs.rename(
            b"tmp\0".try_into().unwrap(),
            b"mnt/tmp\0".try_into().unwrap(),
        )?;
        assert_eq!(fs.available_blocks()?, 512 - 6);
        fs.remove(b"/mnt/tmp\0".try_into().unwrap())?;
        assert_eq!(fs.available_blocks()?, 512 - 4);
        fs.remove(b"/mnt\0".try_into().unwrap())?;
        assert_eq!(fs.available_blocks()?, 512 - 2);

        fs.create_file_and_then(b"/test_with.txt\0".try_into().unwrap(), |file| {
            assert!(file.write(&[0u8, 1, 2])? == 3);
            Ok(())
        })
        .unwrap();

        let mut buf = [0u8; 3];
        fs.open_file_and_then(b"/test_with.txt\0".try_into().unwrap(), |file| {
            assert_eq!(fs.available_blocks()?, 510);
            assert!(file.read_exact(&mut buf).is_ok());
            assert_eq!(&buf, &[0, 1, 2]);
            Ok(())
        })
        .unwrap();

        // surprise surprise, inline files!
        assert_eq!(fs.available_blocks()?, 512 - 2);

        Ok(())
    })
}

#[test]
fn test_create() {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        assert_eq!(fs.total_blocks(), 512);
        assert_eq!(fs.total_space(), 256 * 512);
        // superblock
        assert_eq!(fs.available_blocks().unwrap(), 512 - 2);
        assert_eq!(fs.available_space().unwrap(), 130_560);

        assert!(!fs.exists(path!("/test_open.txt")));
        assert_eq!(
            File::open_and_then(fs, b"/test_open.txt\0".try_into().unwrap(), |_| { Ok(()) })
                .map(drop)
                .unwrap_err(), // "real" contains_err is experimental
            Error::NO_SUCH_ENTRY
        );
        assert!(!fs.exists(path!("/test_open.txt")));

        fs.create_dir(b"/tmp\0".try_into().unwrap()).unwrap();
        assert_eq!(fs.available_blocks().unwrap(), 512 - 2 - 2);

        // can create new files
        assert!(!fs.exists(path!("/tmp/test_open.txt")));
        fs.create_file_and_then(b"/tmp/test_open.txt\0".try_into().unwrap(), |file| {
            // can write to files
            assert!(file.write(&[0u8, 1, 2]).unwrap() == 3);
            file.sync()?;
            // surprise surprise, inline files!
            assert_eq!(fs.available_blocks()?, 512 - 2 - 2);
            // no longer exists!
            // file.close()?;
            Ok(())
        })?;
        assert!(fs.exists(path!("/tmp/test_open.txt")));

        // // cannot remove non-empty directories
        assert_eq!(
            fs.remove(b"/tmp\0".try_into().unwrap()).unwrap_err(),
            Error::DIR_NOT_EMPTY
        );

        let metadata = fs.metadata(b"/tmp\0".try_into().unwrap())?;
        assert!(metadata.is_dir());
        assert_eq!(metadata.len(), 0);

        // can move files
        fs.rename(
            b"/tmp/test_open.txt\0".try_into().unwrap(),
            b"moved.txt\0".try_into().unwrap(),
        )?;
        assert_eq!(fs.available_blocks().unwrap(), 512 - 2 - 2);

        let metadata = fs.metadata(b"/moved.txt\0".try_into().unwrap())?;
        assert!(metadata.is_file());
        assert_eq!(metadata.len(), 3);

        fs.remove(b"/tmp/../tmp/.\0".try_into().unwrap()).unwrap();
        assert_eq!(fs.available_blocks().unwrap(), 512 - 2);

        fs.open_file_and_then(b"/moved.txt\0".try_into().unwrap(), |file| {
            assert!(file.len().unwrap() == 3);
            let mut contents: [u8; 3] = Default::default();
            assert!(file.read(&mut contents).unwrap() == 3);
            assert_eq!(contents, [0u8, 1, 2]);

            // alternative approach
            file.seek(SeekFrom::Start(0))?;
            let mut contents_vec = heapless::Vec::<u8, 3>::new();
            assert!(file.read_to_end(&mut contents_vec).unwrap() == 3);
            Ok(())
        })?;

        Ok(())
    })
    .unwrap();
}

#[test]
fn test_unbind() {
    let mut backend = Ram::default();

    {
        let mut storage = RamStorage::new(&mut backend);
        Filesystem::format(&mut storage).unwrap();
        Filesystem::mount_and_then(&mut storage, |fs| {
            fs.create_file_and_then(b"test_unbind.txt\0".try_into().unwrap(), |file| {
                file.write(b"hello world")?;
                assert_eq!(file.len()?, 11);
                Ok(())
            })
        })
        .unwrap();
    }

    let mut storage = RamStorage::new(&mut backend);
    Filesystem::mount_and_then(&mut storage, |fs| {
        let contents: heapless::Vec<_, 37> = fs.read(b"test_unbind.txt\0".try_into().unwrap())?;
        assert_eq!(contents, b"hello world");
        Ok(())
    })
    .unwrap();
}

#[test]
fn test_seek() {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.write(b"test_seek.txt\0".try_into().unwrap(), b"hello world")?;
        fs.open_file_and_then(b"test_seek.txt\0".try_into().unwrap(), |file| {
            file.seek(SeekFrom::End(-5))?;
            let mut buf = [0u8; 5];
            assert_eq!(file.len()?, 11);
            file.read(&mut buf)?;
            assert_eq!(&buf, b"world");
            Ok(())
        })
    })
    .unwrap();
}

#[test]
fn test_chunked() {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);
    let path = b"test_chunked.txt\0".try_into().unwrap();
    let hello = b"hello world";
    let more = b"but wait, there's more";

    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.write(path, hello)?;
        let (data, len) = fs.read_chunk::<1024>(path, OpenSeekFrom::Start(0)).unwrap();
        assert_eq!(&data, hello);
        assert_eq!(len, hello.len());
        let (data, len) = fs.read_chunk::<1024>(path, OpenSeekFrom::Start(3)).unwrap();
        assert_eq!(&data, &hello[3..]);
        assert_eq!(len, hello.len());
        fs.write_chunk(path, more, OpenSeekFrom::End(0)).unwrap();
        let (data, len) = fs
            .read_chunk::<1024>(path, OpenSeekFrom::Start(hello.len() as u32))
            .unwrap();
        assert_eq!(&data, more);
        assert_eq!(len, hello.len() + more.len());
        let (data, len) = fs
            .read_chunk::<1024>(path, OpenSeekFrom::End(-(more.len() as i32)))
            .unwrap();
        assert_eq!(&data, more);
        assert_eq!(len, hello.len() + more.len());

        Ok(())
    })
    .unwrap();
}

#[test]
fn test_file_set_len() {
    let mut backend = OtherRam::default();
    let mut storage = OtherRamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.create_file_and_then(b"test_set_len.txt\0".try_into().unwrap(), |file| {
            file.write(b"hello littlefs")?;
            assert_eq!(file.len()?, 14);

            file.set_len(10).unwrap();
            assert_eq!(file.len()?, 10);

            // note that:
            // a) "tell" can be implemented as follows,
            // b) truncating a file does not change the cursor position
            assert_eq!(file.seek(SeekFrom::Current(0))?, 14);
            Ok(())
        })
    })
    .unwrap();
}

#[test]
fn test_fancy_open() {
    let mut backend = Ram::default();
    let mut storage = RamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();

    let mut buf = [0u8; 5];

    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.open_file_with_options_and_then(
            |options| options.read(true).write(true).create_new(true),
            b"test_fancy_open.txt\0".try_into().unwrap(),
            |file| {
                file.write(b"hello world")?;
                assert_eq!(file.len()?, 11);
                file.seek(SeekFrom::Start(6))?;

                file.read(&mut buf)
            },
        )
    })
    .unwrap();

    assert_eq!(&buf, b"world");
}

#[test]
fn remove_dir_all_where() {
    let mut backend = Ram::default();
    let mut storage = RamStorage::new(&mut backend);

    Filesystem::format(&mut storage).unwrap();

    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.write(path!("test_file"), b"some data").unwrap();
        fs.create_dir(path!("test_dir")).unwrap();
        fs.write(path!("test_dir/test_file"), b"some_inner_data")
            .unwrap();
        fs.write(path!("test_dir/test_file2"), b"some_inner_data")
            .unwrap();
        fs.remove_dir_all_where(path!(""), &|entry| {
            entry.path() != path!("test_dir/test_file")
        })
        .unwrap();
        assert!(fs.metadata(path!("test_dir/test_file")).unwrap().is_file());
        assert_eq!(fs.metadata(path!("test_file")), Err(Error::NO_SUCH_ENTRY));
        assert_eq!(
            fs.metadata(path!("test_dir/test_file2")),
            Err(Error::NO_SUCH_ENTRY)
        );
        Ok(())
    })
    .unwrap();
}

#[test]
fn attributes() {
    let mut backend = Ram::default();
    let mut storage = RamStorage::new(&mut backend);
    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        let mut buffer = [0; Attribute::MAX_SIZE as _];

        let filename = b"some.file\0".try_into().unwrap();
        fs.write(filename, &[])?;
        assert!(fs.attribute(filename, 37, &mut buffer)?.is_none());

        let data = b"top secret";
        fs.set_attribute(filename, 37, data).unwrap();
        let attribute = fs.attribute(filename, 37, &mut buffer)?.unwrap();
        assert_eq!(data, attribute.data());
        assert_eq!(data.len(), attribute.total_size());

        // if the buffer is smaller than the attribute, it is truncated and the
        // full size is returned
        let mut small_buffer = [0; 5];
        let attribute = fs.attribute(filename, 37, &mut small_buffer)?.unwrap();
        assert_eq!(&data[..5], attribute.data());
        assert_eq!(data.len(), attribute.total_size());

        // if the input data is too long, an error is returned
        let long_data = &[0xff; 1024];
        assert!(long_data.len() > Attribute::MAX_SIZE as _);
        assert_eq!(
            Err(Error::NO_SPACE),
            fs.set_attribute(filename, 37, long_data)
        );

        // // not sure if we should have this method (may be quite expensive)
        // let attributes = unsafe { fs.attributes("some.file", &mut storage).unwrap() };
        // assert!(attributes[37]);
        // assert_eq!(attributes.iter().fold(0, |sum, i| sum + (*i as u8)), 1);

        fs.remove_attribute(filename, 37)?;
        assert!(fs.attribute(filename, 37, &mut buffer)?.is_none());

        // // Directories can have attributes too
        let tmp_dir = b"/tmp\0".try_into().unwrap();
        fs.create_dir(tmp_dir)?;

        let data = b"temporary directory";
        fs.set_attribute(tmp_dir, 37, data)?;

        let attribute = fs.attribute(tmp_dir, 37, &mut buffer)?.unwrap();
        assert_eq!(data, attribute.data());
        assert_eq!(data.len(), attribute.total_size());

        fs.remove_attribute(tmp_dir, 37)?;
        assert!(fs.attribute(tmp_dir, 37, &mut buffer)?.is_none());

        Ok(())
    })
    .unwrap();
}

#[test]
fn test_iter_dirs() {
    let mut backend = Ram::default();
    let mut storage = RamStorage::new(&mut backend);
    Filesystem::format(&mut storage).unwrap();
    Filesystem::mount_and_then(&mut storage, |fs| {
        fs.create_dir(path!("/tmp"))?;

        // TODO: we might want "multi-open"
        fs.create_file_and_then(path!("/tmp/file.a"), |file| {
            file.set_len(37)?;
            fs.create_file_and_then(path!("/tmp/file.b"), |file| file.set_len(42))
        })?;

        fs.read_dir_and_then(path!("/tmp"), |dir| {
            let mut found_files: usize = 0;
            let mut sizes = [0usize; 4];

            for (i, entry) in dir.enumerate() {
                let entry = entry?;

                // assert_eq!(entry.file_name(), match i {
                //     0 => b".\0",
                //     1 => b"..\0",
                //     2 => b"file.a\0",
                //     3 => b"file.b\0",
                //     _ => panic!("oh noes"),
                // });

                sizes[i] = entry.metadata().len();
                found_files += 1;
            }

            assert_eq!(sizes, [0, 0, 37, 42]);
            assert_eq!(found_files, 4);

            Ok(())
        })
    })
    .unwrap();
}

#[test]
fn test_mount_or_else_clobber_alloc() {
    let mut backend = Ram::default();
    let mut storage = RamStorage::new(&mut backend);
    let alloc = &mut Allocation::new();
    Filesystem::mount_or_else(alloc, &mut storage, |_, storage, alloc| {
        *alloc = Allocation::new();
        Filesystem::format(storage).unwrap();
        Ok(())
    })
    .unwrap();
}

// // These are some tests that ensure our type constructions
// // actually do what we intend them to do.
// // Since dev-features cannot be optional, trybuild is not `no_std`,
// // and we want to actually test `no_std`...
// #[test]
// #[cfg(feature = "ui-tests")]
// fn test_api_safety() {
//     let t = trybuild::TestCases::new();
//     t.compile_fail("tests/ui/*-fail.rs");
//     t.pass("tests/ui/*-pass.rs");
// }

#[cfg(feature = "unstable-littlefs-patched")]
#[test]
fn shrinking() {
    use crate::{driver::Storage, fs::MountFlags};

    let backend = &mut Ram::default();
    let storage = &mut RamStorage::new(backend);
    let alloc = &mut Allocation::new();

    Filesystem::format(storage).unwrap();
    let fs = Filesystem::mount(alloc, storage).unwrap();
    fs.write(path!("some-file"), &[42; 10]).unwrap();
    fs.write(path!("some-large-file"), &[42; 1024]).unwrap();

    assert_eq!(fs.read::<10>(path!("some-file")).unwrap(), &[42; 10]);
    assert_eq!(
        fs.read::<1024>(path!("some-large-file")).unwrap(),
        &[42; 1024]
    );

    let larger_backend = &mut LargerRam::default();
    larger_backend.buf[..backend.buf.len()].copy_from_slice(&backend.buf);
    let larger_storage = &mut LargerRamStorage::new(larger_backend);
    let larger_alloc = &mut Allocation::new();
    assert!(matches!(
        Filesystem::mount(larger_alloc, larger_storage),
        Err(Error::INVALID)
    ));

    let larger_alloc = &mut Allocation::with_config(crate::fs::Config {
        mount_flags: MountFlags::DISABLE_BLOCK_COUNT_CHECK,
    });

    let fs = Filesystem::mount(larger_alloc, larger_storage).unwrap();
    assert_eq!(fs.read::<10>(path!("some-file")).unwrap(), &[42; 10]);
    assert_eq!(
        fs.read::<1024>(path!("some-large-file")).unwrap(),
        &[42; 1024]
    );

    fs.grow(LargerRamStorage::BLOCK_COUNT).unwrap();
    assert_eq!(fs.read::<10>(path!("some-file")).unwrap(), &[42; 10]);
    assert_eq!(
        fs.read::<1024>(path!("some-large-file")).unwrap(),
        &[42; 1024]
    );

    fs.shrink(RamStorage::BLOCK_COUNT).unwrap();
    assert_eq!(fs.read::<10>(path!("some-file")).unwrap(), &[42; 10]);
    assert_eq!(
        fs.read::<1024>(path!("some-large-file")).unwrap(),
        &[42; 1024]
    );
}

#[cfg(feature = "unstable-littlefs-patched")]
#[test]
fn shrinking_full() {
    use crate::driver::Storage;
    use littlefs2_core::PathBuf;

    let larger_backend = &mut LargerRam::default();
    let larger_storage = &mut LargerRamStorage::new(larger_backend);
    let larger_alloc = &mut Allocation::new();
    Filesystem::format(larger_storage).unwrap();
    let fs = Filesystem::mount(larger_alloc, larger_storage).unwrap();

    for i in 0.. {
        let path = format!("file-{i}");
        let contents = &[0; 1024];
        match fs.write(&PathBuf::try_from(&*path).unwrap(), contents) {
            Ok(_) => continue,
            Err(Error::NO_SPACE) => break,
            Err(err) => panic!("{err:?}"),
        }
    }

    assert!(matches!(
        fs.shrink(RamStorage::BLOCK_COUNT),
        Err(Error::DIR_NOT_EMPTY)
    ))
}
