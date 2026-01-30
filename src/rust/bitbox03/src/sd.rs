use bitbox_hal as hal;

pub struct BitBox03Sd;

impl hal::sd::Sd for BitBox03Sd {
    async fn sdcard_inserted(&mut self) -> bool {
        todo!()
    }

    async fn list_subdir(
        &mut self,
        _subdir: Option<&str>,
    ) -> Result<alloc::vec::Vec<alloc::string::String>, ()> {
        todo!()
    }

    async fn erase_file_in_subdir(&mut self, _filename: &str, _dir: &str) -> Result<(), ()> {
        todo!()
    }

    async fn load_bin(
        &mut self,
        _filename: &str,
        _dir: &str,
    ) -> Result<zeroize::Zeroizing<alloc::vec::Vec<u8>>, ()> {
        todo!()
    }

    async fn write_bin(&mut self, _filename: &str, _dir: &str, _data: &[u8]) -> Result<(), ()> {
        todo!()
    }
}
