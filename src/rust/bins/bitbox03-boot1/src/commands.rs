// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use bitbox_boot_utils::{
    FIRMWARE_ADDR, FIRMWARE_MAX_LEN, FLASH_PAGE_SIZE, IMAGE_HEADER_INVALID_CODE_SIZE,
    IMAGE_HEADER_LEN, IMAGE_HEADER_MAGIC_FIRMWARE, IMAGE_SIGNATURE_COUNT, IMMUTABLE_PAGE_ADDR,
    P256_SIGNATURE_LEN,
};
use bitbox03_boot_utils::ImmutablePage;
use sha2::{Digest, Sha256};

use crate::protocol::{
    BOOT_OP_LEN, FIRMWARE_CHUNK_LEN, MAX_FIRMWARE_NUM_CHUNKS, OP_ERASE, OP_HARDWARE, OP_HASHES,
    OP_REBOOT, OP_SCREEN_ROTATE, OP_SET_SHOW_FIRMWARE_HASH, OP_STATUS_ERR_ERASE,
    OP_STATUS_ERR_INVALID_CMD, OP_STATUS_ERR_LEN, OP_STATUS_ERR_LOAD_FLAG, OP_STATUS_ERR_WRITE,
    OP_STATUS_OK, OP_VERSIONS, OP_WRITE_FIRMWARE_CHUNK, OP_WRITE_SIG_DATA,
    parse_chunk_index_and_data, parse_num_chunks,
};

pub trait Backend {
    fn read(&self, addr: usize, out: &mut [u8]);
    fn write_page(&mut self, addr: usize, page: &[u8; FLASH_PAGE_SIZE]) -> Result<(), ()>;
    fn reboot(&mut self) -> !;
    fn hardware_type(&self) -> u8;
}

#[cfg(target_arch = "arm")]
pub struct FlashBackend;

#[cfg(target_arch = "arm")]
impl Backend for FlashBackend {
    fn read(&self, addr: usize, out: &mut [u8]) {
        bitbox_platform_stm32u5::flash::read(addr, out);
    }

    fn write_page(&mut self, addr: usize, page: &[u8; FLASH_PAGE_SIZE]) -> Result<(), ()> {
        bitbox_platform_stm32u5::flash::write_page(addr, page).map_err(|_| ())
    }

    fn reboot(&mut self) -> ! {
        cortex_m::interrupt::disable();
        cortex_m::peripheral::SCB::sys_reset()
    }

    fn hardware_type(&self) -> u8 {
        1
    }
}

pub struct BootloaderApi<B> {
    backend: B,
    loading_ready: bool,
    firmware_num_chunks: u16,
}

impl<B> BootloaderApi<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            loading_ready: false,
            firmware_num_chunks: 0,
        }
    }
}

impl<B: Backend> BootloaderApi<B> {
    pub fn handle(&mut self, input: &[u8]) -> Vec<u8> {
        let Some(&op) = input.first() else {
            return Vec::new();
        };

        match op {
            OP_VERSIONS => self.api_versions(op),
            OP_HASHES => self.api_get_hashes(op, &input[1..]),
            OP_SET_SHOW_FIRMWARE_HASH => self.api_set_show_firmware_hash(op, &input[1..]),
            OP_ERASE => self.api_firmware_erase(op, &input[1..]),
            OP_REBOOT => self.backend.reboot(),
            OP_WRITE_FIRMWARE_CHUNK => self.api_write_chunk(op, &input[1..]),
            OP_WRITE_SIG_DATA => self.status_response(op, OP_STATUS_OK),
            OP_SCREEN_ROTATE => self.api_screen_rotate(op),
            OP_HARDWARE => self.api_hardware(op),
            _ => self.status_response(op, OP_STATUS_ERR_INVALID_CMD),
        }
    }

    fn api_versions(&self, op: u8) -> Vec<u8> {
        let firmware_version = self.current_firmware_monotonic_version().unwrap_or(0);
        let key_version = self.immutable_page().map(|page| page.version).unwrap_or(0);

        let mut response = self.status_response(op, OP_STATUS_OK);
        response.extend_from_slice(&firmware_version.to_le_bytes());
        response.extend_from_slice(&key_version.to_le_bytes());
        response
    }

    fn api_get_hashes(&self, op: u8, input: &[u8]) -> Vec<u8> {
        if input.len() < 2 {
            return self.status_response(op, OP_STATUS_ERR_LEN);
        }

        let mut response = self.status_response(op, OP_STATUS_OK);
        response.extend_from_slice(&self.current_firmware_hash().unwrap_or([0; 32]));
        response.extend_from_slice(&self.key_material_hash().unwrap_or([0; 32]));
        response
    }

    fn api_set_show_firmware_hash(&self, op: u8, input: &[u8]) -> Vec<u8> {
        if input.len() != 1 {
            return self.status_response(op, OP_STATUS_ERR_LEN);
        }

        let mut response = self.status_response(op, OP_STATUS_OK);
        response.push(0);
        response
    }

    fn api_firmware_erase(&mut self, op: u8, input: &[u8]) -> Vec<u8> {
        let Some(firmware_num_chunks) = parse_num_chunks(input) else {
            return self.status_response(op, OP_STATUS_ERR_LEN);
        };
        if firmware_num_chunks > MAX_FIRMWARE_NUM_CHUNKS {
            return self.status_response(op, OP_STATUS_ERR_LEN);
        }

        if self.erase_unused_pages(firmware_num_chunks).is_err() {
            return self.status_response(op, OP_STATUS_ERR_ERASE);
        }

        self.firmware_num_chunks = firmware_num_chunks;
        self.loading_ready = firmware_num_chunks > 0;
        self.status_response(op, OP_STATUS_OK)
    }

    fn api_write_chunk(&mut self, op: u8, input: &[u8]) -> Vec<u8> {
        if !self.loading_ready {
            return self.status_response(op, OP_STATUS_ERR_LOAD_FLAG);
        }
        self.loading_ready = false;
        let Some((chunk_num, chunk)) = parse_chunk_index_and_data(input) else {
            return self.status_response(op, OP_STATUS_ERR_LEN);
        };
        if chunk_num >= self.firmware_num_chunks || chunk_num >= MAX_FIRMWARE_NUM_CHUNKS {
            return self.status_response(op, OP_STATUS_ERR_LEN);
        }

        let result = self.write_chunk(chunk_num, chunk);

        match result {
            Ok(()) => {
                self.loading_ready = true;
                self.status_response(op, OP_STATUS_OK)
            }
            Err(status) => self.status_response(op, status),
        }
    }

    fn api_screen_rotate(&self, op: u8) -> Vec<u8> {
        if self.loading_ready {
            return self.status_response(op, OP_STATUS_ERR_LOAD_FLAG);
        }
        self.status_response(op, OP_STATUS_OK)
    }

    fn api_hardware(&self, op: u8) -> Vec<u8> {
        let mut response = self.status_response(op, OP_STATUS_OK);
        response.push(self.backend.hardware_type());
        response
    }

    fn erase_unused_pages(&mut self, firmware_num_chunks: u16) -> Result<(), ()> {
        let firmware_len = firmware_num_chunks as usize * FIRMWARE_CHUNK_LEN;
        let erase_start = if firmware_len % FLASH_PAGE_SIZE == 0 {
            FIRMWARE_ADDR + firmware_len
        } else {
            align_down(FIRMWARE_ADDR + firmware_len, FLASH_PAGE_SIZE)
        };
        let empty_page = [0xff; FLASH_PAGE_SIZE];

        for page_addr in (erase_start..FIRMWARE_ADDR + FIRMWARE_MAX_LEN).step_by(FLASH_PAGE_SIZE) {
            let mut page = [0u8; FLASH_PAGE_SIZE];
            self.backend.read(page_addr, &mut page);
            if page != empty_page {
                self.backend.write_page(page_addr, &empty_page)?;
            }
        }
        Ok(())
    }

    fn write_chunk(&mut self, chunk_num: u16, chunk: &[u8]) -> Result<(), u8> {
        if chunk.len() != FIRMWARE_CHUNK_LEN {
            return Err(OP_STATUS_ERR_LEN);
        }

        let chunk_addr = FIRMWARE_ADDR + chunk_num as usize * FIRMWARE_CHUNK_LEN;
        let page_addr = align_down(chunk_addr, FLASH_PAGE_SIZE);
        let page_offset = chunk_addr - page_addr;

        let mut current_page = [0u8; FLASH_PAGE_SIZE];
        self.backend.read(page_addr, &mut current_page);

        let mut new_page = current_page;
        new_page[page_offset..page_offset + FIRMWARE_CHUNK_LEN].copy_from_slice(chunk);
        if new_page == current_page {
            return Ok(());
        }

        self.backend
            .write_page(page_addr, &new_page)
            .map_err(|_| OP_STATUS_ERR_WRITE)
    }

    fn current_firmware_monotonic_version(&self) -> Option<u32> {
        Some(self.current_firmware_header()?.monotonic_version)
    }

    fn current_firmware_hash(&self) -> Option<[u8; 32]> {
        let header = self.current_firmware_header()?;
        if header.code_size == IMAGE_HEADER_INVALID_CODE_SIZE {
            return None;
        }
        let payload_len = header.code_size as usize;
        if payload_len == 0 || payload_len > FIRMWARE_MAX_LEN - IMAGE_HEADER_LEN {
            return None;
        }

        let mut hasher = Sha256::new();
        hasher.update(header_signed_bytes(&header));

        let mut remaining = payload_len;
        let mut offset = 0usize;
        let mut chunk = [0u8; FLASH_PAGE_SIZE];
        while remaining > 0 {
            let read_len = remaining.min(chunk.len());
            self.backend.read(
                FIRMWARE_ADDR + IMAGE_HEADER_LEN + offset,
                &mut chunk[..read_len],
            );
            hasher.update(&chunk[..read_len]);
            remaining -= read_len;
            offset += read_len;
        }

        Some(double_hash(&hasher.finalize()))
    }

    fn key_material_hash(&self) -> Option<[u8; 32]> {
        let page = self.immutable_page()?;
        let mut hasher = Sha256::new();
        hasher.update(page.version.to_le_bytes());
        for pubkey in page.root_pubkeys {
            hasher.update(pubkey);
        }
        Some(double_hash(&hasher.finalize()))
    }

    fn immutable_page(&self) -> Option<ImmutablePage> {
        let mut page_bytes = [0u8; FLASH_PAGE_SIZE];
        self.backend.read(IMMUTABLE_PAGE_ADDR, &mut page_bytes);
        ImmutablePage::from_page_bytes(&page_bytes).ok()
    }

    fn current_firmware_header(&self) -> Option<bitbox_boot_utils::ImageHeader> {
        let mut header_bytes = [0u8; IMAGE_HEADER_LEN];
        self.backend.read(FIRMWARE_ADDR, &mut header_bytes);
        let header = bitbox_boot_utils::ImageHeader::from_bytes(&header_bytes).ok()?;
        if header.magic != IMAGE_HEADER_MAGIC_FIRMWARE {
            return None;
        }
        Some(header)
    }

    fn status_response(&self, op: u8, status: u8) -> Vec<u8> {
        let mut response = Vec::with_capacity(BOOT_OP_LEN);
        response.push(op);
        response.push(status);
        response
    }
}

fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

fn header_signed_bytes(header: &bitbox_boot_utils::ImageHeader) -> &[u8] {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            (header as *const bitbox_boot_utils::ImageHeader).cast::<u8>(),
            core::mem::size_of::<bitbox_boot_utils::ImageHeader>(),
        )
    };
    &bytes
        [..bitbox_boot_utils::ImageHeader::STRUCT_LEN - P256_SIGNATURE_LEN * IMAGE_SIGNATURE_COUNT]
}

fn double_hash(first_hash: &[u8]) -> [u8; 32] {
    let second_hash = Sha256::digest(first_hash);
    let mut out = [0u8; 32];
    out.copy_from_slice(&second_hash);
    out
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use alloc::vec;
    use bitbox_boot_utils::{
        FLASH_BASE_NS, FLASH_TOTAL_SIZE, IMAGE_HEADER_MAGIC_BOOT1,
        IMAGE_HEADER_MARKETING_VERSION_LEN, IMMUTABLE_PAGE_VERSION, P256_PUBLIC_KEY_LEN,
    };
    struct MockBackend {
        flash: Vec<u8>,
        hardware_type: u8,
    }

    impl MockBackend {
        fn new() -> Self {
            Self {
                flash: vec![0xff; FLASH_TOTAL_SIZE],
                hardware_type: 1,
            }
        }

        fn offset(addr: usize) -> usize {
            addr - FLASH_BASE_NS
        }

        fn write_bytes(&mut self, addr: usize, bytes: &[u8]) {
            let start = Self::offset(addr);
            self.flash[start..start + bytes.len()].copy_from_slice(bytes);
        }
    }

    impl Backend for MockBackend {
        fn read(&self, addr: usize, out: &mut [u8]) {
            let start = Self::offset(addr);
            out.copy_from_slice(&self.flash[start..start + out.len()]);
        }

        fn write_page(&mut self, addr: usize, page: &[u8; FLASH_PAGE_SIZE]) -> Result<(), ()> {
            self.write_bytes(addr, page);
            Ok(())
        }

        fn reboot(&mut self) -> ! {
            panic!("reboot")
        }

        fn hardware_type(&self) -> u8 {
            self.hardware_type
        }
    }

    fn make_header(
        magic: [u8; 4],
        code_size: u32,
        monotonic_version: u32,
    ) -> [u8; IMAGE_HEADER_LEN] {
        let mut header = [0u8; IMAGE_HEADER_LEN];
        header[..4].copy_from_slice(&magic);
        header[4..8].copy_from_slice(&(IMAGE_HEADER_LEN as u32).to_le_bytes());
        header[8..12].copy_from_slice(&code_size.to_le_bytes());
        header[12..16].copy_from_slice(&1u32.to_le_bytes());
        let marketing = b"v1.1.2";
        header[16..16 + marketing.len()].copy_from_slice(marketing);
        header[16 + IMAGE_HEADER_MARKETING_VERSION_LEN..20 + IMAGE_HEADER_MARKETING_VERSION_LEN]
            .copy_from_slice(&monotonic_version.to_le_bytes());
        header
    }

    fn install_firmware(backend: &mut MockBackend, code_size: usize, monotonic_version: u32) {
        let header = make_header(
            IMAGE_HEADER_MAGIC_FIRMWARE,
            code_size as u32,
            monotonic_version,
        );
        let mut image = vec![0xff; IMAGE_HEADER_LEN + code_size];
        image[..IMAGE_HEADER_LEN].copy_from_slice(&header);
        for (i, byte) in image[IMAGE_HEADER_LEN..].iter_mut().enumerate() {
            *byte = (i & 0xff) as u8;
        }
        backend.write_bytes(FIRMWARE_ADDR, &image);
    }

    fn install_immutable_page(backend: &mut MockBackend, version: u32) {
        let mut immutable_page = ImmutablePage::blank();
        immutable_page.version = version;
        for key_idx in 0..IMAGE_SIGNATURE_COUNT {
            for byte_idx in 0..P256_PUBLIC_KEY_LEN {
                immutable_page.root_pubkeys[key_idx][byte_idx] =
                    (key_idx * P256_PUBLIC_KEY_LEN + byte_idx) as u8;
            }
        }
        let page = immutable_page.to_page_bytes();
        backend.write_bytes(IMMUTABLE_PAGE_ADDR, &page);
    }

    #[test]
    fn test_versions() {
        let mut backend = MockBackend::new();
        install_firmware(&mut backend, 512, 23);
        install_immutable_page(&mut backend, IMMUTABLE_PAGE_VERSION);
        let mut api = BootloaderApi::new(backend);

        let response = api.handle(&[OP_VERSIONS]);

        assert_eq!(response[..2], [OP_VERSIONS, OP_STATUS_OK]);
        assert_eq!(u32::from_le_bytes(response[2..6].try_into().unwrap()), 23);
        assert_eq!(
            u32::from_le_bytes(response[6..10].try_into().unwrap()),
            IMMUTABLE_PAGE_VERSION
        );
    }

    #[test]
    fn test_write_chunk_requires_erase() {
        let mut api = BootloaderApi::new(MockBackend::new());
        let request = vec![OP_WRITE_FIRMWARE_CHUNK, 0u8]
            .into_iter()
            .chain(core::iter::repeat_n(0xaa, FIRMWARE_CHUNK_LEN))
            .collect::<Vec<_>>();

        let response = api.handle(&request);

        assert_eq!(
            response,
            vec![OP_WRITE_FIRMWARE_CHUNK, OP_STATUS_ERR_LOAD_FLAG]
        );
    }

    #[test]
    fn test_erase_and_write_chunk_with_u16_index() {
        let mut api = BootloaderApi::new(MockBackend::new());

        assert_eq!(
            api.handle(&[OP_ERASE, 0x2c, 0x01]),
            vec![OP_ERASE, OP_STATUS_OK]
        );

        let chunk_num = 0x0100u16;
        let mut request = Vec::with_capacity(1 + 2 + FIRMWARE_CHUNK_LEN);
        request.push(OP_WRITE_FIRMWARE_CHUNK);
        request.extend_from_slice(&chunk_num.to_le_bytes());
        request.extend(core::iter::repeat_n(0x5a, FIRMWARE_CHUNK_LEN));
        assert_eq!(
            api.handle(&request),
            vec![OP_WRITE_FIRMWARE_CHUNK, OP_STATUS_OK]
        );

        let mut readback = [0u8; FIRMWARE_CHUNK_LEN];
        api.backend.read(
            FIRMWARE_ADDR + chunk_num as usize * FIRMWARE_CHUNK_LEN,
            &mut readback,
        );
        assert_eq!(readback, [0x5a; FIRMWARE_CHUNK_LEN]);
    }

    #[test]
    fn test_hashes() {
        let mut backend = MockBackend::new();
        install_firmware(&mut backend, 600, 11);
        install_immutable_page(&mut backend, IMMUTABLE_PAGE_VERSION);
        let mut api = BootloaderApi::new(backend);

        let response = api.handle(&[OP_HASHES, 0, 0]);

        assert_eq!(response[..2], [OP_HASHES, OP_STATUS_OK]);
        assert_ne!(response[2..34], [0; 32]);
        assert_ne!(response[34..66], [0; 32]);
    }

    #[test]
    fn test_write_sig_data_is_noop() {
        let mut api = BootloaderApi::new(MockBackend::new());
        assert_eq!(
            api.handle(&[OP_WRITE_SIG_DATA, 0xaa, 0xbb]),
            vec![OP_WRITE_SIG_DATA, OP_STATUS_OK]
        );
    }

    #[test]
    #[should_panic(expected = "reboot")]
    fn test_reboot() {
        let mut api = BootloaderApi::new(MockBackend::new());
        let _ = api.handle(&[OP_REBOOT]);
    }

    #[test]
    fn test_invalid_command() {
        let mut api = BootloaderApi::new(MockBackend::new());
        assert_eq!(api.handle(&[b'x']), vec![b'x', OP_STATUS_ERR_INVALID_CMD]);
    }

    #[test]
    fn test_versions_ignores_non_firmware_header() {
        let mut backend = MockBackend::new();
        let header = make_header(IMAGE_HEADER_MAGIC_BOOT1, 512, 42);
        backend.write_bytes(FIRMWARE_ADDR, &header);
        let mut api = BootloaderApi::new(backend);

        let response = api.handle(&[OP_VERSIONS]);
        assert_eq!(u32::from_le_bytes(response[2..6].try_into().unwrap()), 0);
    }
}
