#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(test)]
extern crate std;

use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use crc32fast::Hasher;
use miniz_oxide::inflate::TINFLStatus;
use num_enum::TryFromPrimitive;

const PNG_MAGIC_BYTES: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10];

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum BitDepth {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum ColorType {
    Grayscale = 0,
    Rgb = 2,
    Palette = 3,
    GrayscaleAlpha = 4,
    RgbAlpha = 6,
}

impl ColorType {
    pub fn sample_multiplier(&self) -> usize {
        match self {
            ColorType::Grayscale => 1,
            ColorType::Rgb => 3,
            ColorType::Palette => 1,
            ColorType::GrayscaleAlpha => 2,
            ColorType::RgbAlpha => 4,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum PixelType {
    Grayscale1,
    Grayscale2,
    Grayscale4,
    Grayscale8,
    Grayscale16,

    Rgb8,
    Rgb16,

    Palette1,
    Palette2,
    Palette4,
    Palette8,

    GrayscaleAlpha8,
    GrayscaleAlpha16,

    RgbAlpha8,
    RgbAlpha16,
}

impl PixelType {
    fn new(color_type: ColorType, bit_depth: BitDepth) -> Result<Self, DecodeError> {
        let result = match color_type {
            ColorType::Grayscale => match bit_depth {
                BitDepth::One => PixelType::Grayscale1,
                BitDepth::Two => PixelType::Grayscale2,
                BitDepth::Four => PixelType::Grayscale4,
                BitDepth::Eight => PixelType::Grayscale8,
                BitDepth::Sixteen => PixelType::Grayscale16,
            },
            ColorType::Rgb => match bit_depth {
                BitDepth::Eight => PixelType::Rgb8,
                BitDepth::Sixteen => PixelType::Rgb16,
                _ => return Err(DecodeError::InvalidColorTypeBitDepthCombination),
            },
            ColorType::Palette => match bit_depth {
                BitDepth::One => PixelType::Palette1,
                BitDepth::Two => PixelType::Palette2,
                BitDepth::Four => PixelType::Palette4,
                BitDepth::Eight => PixelType::Palette8,
                _ => return Err(DecodeError::InvalidColorTypeBitDepthCombination),
            },
            ColorType::GrayscaleAlpha => match bit_depth {
                BitDepth::Eight => PixelType::GrayscaleAlpha8,
                BitDepth::Sixteen => PixelType::GrayscaleAlpha16,
                _ => return Err(DecodeError::InvalidColorTypeBitDepthCombination),
            },
            ColorType::RgbAlpha => match bit_depth {
                BitDepth::Eight => PixelType::RgbAlpha8,
                BitDepth::Sixteen => PixelType::RgbAlpha16,
                _ => return Err(DecodeError::InvalidColorTypeBitDepthCombination),
            },
        };

        Ok(result)
    }
}

#[inline(always)]
fn u16_to_u8(val: u16) -> u8 {
    (val >> 8) as u8
}

#[derive(Default)]
struct AncillaryChunks<'a> {
    palette: Option<&'a [u8]>,
    transparency: Option<TransparencyChunk<'a>>,
    background: Option<&'a [u8]>,
}

struct ScanlineIterator<'a> {
    image_width: usize, // Width in pixels
    pixel_cursor: usize,
    pixel_type: PixelType,
    scanline: &'a [u8],
    extra_chunks: &'a AncillaryChunks<'a>,
}

impl<'a> ScanlineIterator<'a> {
    fn new(
        image_width: u32,
        pixel_type: PixelType,
        scanline: &'a [u8],
        extra_chunks: &'a AncillaryChunks<'a>,
    ) -> Self {
        Self {
            image_width: image_width as usize,
            pixel_cursor: 0,
            pixel_type,
            scanline,
            extra_chunks,
        }
    }
}

impl<'a> Iterator for ScanlineIterator<'a> {
    type Item = (u8, u8, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixel_cursor >= self.image_width {
            return None;
        }

        let pixel = match self.pixel_type {
            PixelType::Grayscale1 => {
                let byte = self.scanline[self.pixel_cursor / 8];
                let bit_offset = 7 - self.pixel_cursor % 8;
                let grayscale_val = (byte >> bit_offset) & 1;

                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Grayscale(transparent_val))
                        if grayscale_val == transparent_val =>
                    {
                        0
                    },
                    _ => 255,
                };

                let pixel_val = grayscale_val * 255;

                Some((pixel_val, pixel_val, pixel_val, alpha))
            },
            PixelType::Grayscale2 => {
                let byte = self.scanline[self.pixel_cursor / 4];
                let bit_offset = 6 - ((self.pixel_cursor % 4) * 2);
                let grayscale_val = (byte >> bit_offset) & 0b11;

                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Grayscale(transparent_val))
                        if grayscale_val == transparent_val =>
                    {
                        0
                    },
                    _ => 255,
                };

                // TODO - use a lookup table
                let pixel_val = ((grayscale_val as f32 / 3.0) * 255.0) as u8;

                Some((pixel_val, pixel_val, pixel_val, alpha))
            },
            PixelType::Grayscale4 => {
                let byte = self.scanline[self.pixel_cursor / 2];
                let bit_offset = 4 - ((self.pixel_cursor % 2) * 4);
                let grayscale_val = (byte >> bit_offset) & 0b1111;

                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Grayscale(transparent_val))
                        if grayscale_val == transparent_val =>
                    {
                        0
                    },
                    _ => 255,
                };

                // TODO - use a lookup table
                let pixel_val = ((grayscale_val as f32 / 15.0) * 255.0) as u8;
                Some((pixel_val, pixel_val, pixel_val, alpha))
            },
            PixelType::Grayscale8 => {
                let byte = self.scanline[self.pixel_cursor];

                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Grayscale(transparent_val))
                        if byte == transparent_val =>
                    {
                        0
                    },
                    _ => 255,
                };
                Some((byte, byte, byte, alpha))
            },
            PixelType::Grayscale16 => {
                let offset = self.pixel_cursor * 2;
                let grayscale_val =
                    u16::from_be_bytes([self.scanline[offset], self.scanline[offset + 1]]);

                let pixel_val = u16_to_u8(grayscale_val);

                // TODO(bschwind) - This may need to be compared to the original
                //                  16-bit transparency value, instead of the transformed
                //                  8-bit value.
                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Grayscale(transparent_val))
                        if pixel_val == transparent_val =>
                    {
                        0
                    },
                    _ => 255,
                };

                Some((pixel_val, pixel_val, pixel_val, alpha))
            },
            PixelType::Rgb8 => {
                let offset = self.pixel_cursor * 3;
                let r = self.scanline[offset];
                let g = self.scanline[offset + 1];
                let b = self.scanline[offset + 2];

                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Rgb(t_r, t_g, t_b))
                        if r == t_r && g == t_g && b == t_b =>
                    {
                        0
                    },
                    _ => 255,
                };

                Some((r, g, b, alpha))
            },
            PixelType::Rgb16 => {
                let offset = self.pixel_cursor * 6;
                let r = u16::from_be_bytes([self.scanline[offset], self.scanline[offset + 1]]);
                let g = u16::from_be_bytes([self.scanline[offset + 2], self.scanline[offset + 3]]);
                let b = u16::from_be_bytes([self.scanline[offset + 4], self.scanline[offset + 5]]);

                let r = u16_to_u8(r);
                let g = u16_to_u8(g);
                let b = u16_to_u8(b);

                let alpha = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Rgb(t_r, t_g, t_b))
                        if r == t_r && g == t_g && b == t_b =>
                    {
                        0
                    },
                    _ => 255,
                };

                Some((r, g, b, alpha))
            },
            PixelType::Palette1 => {
                let byte = self.scanline[self.pixel_cursor / 8];
                let bit_offset = 7 - self.pixel_cursor % 8;
                let palette_idx = ((byte >> bit_offset) & 1) as usize;

                let offset = palette_idx * 3;

                let palette = self.extra_chunks.palette.unwrap();
                let r = palette[offset];
                let g = palette[offset + 1];
                let b = palette[offset + 2];

                let alpha: u8 = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Palette(data)) => {
                        *data.get(palette_idx).unwrap_or(&255)
                    },
                    Some(_) | None => 255,
                };

                Some((r, g, b, alpha))
            },
            PixelType::Palette2 => {
                let byte = self.scanline[self.pixel_cursor / 4];
                let bit_offset = 6 - ((self.pixel_cursor % 4) * 2);
                let palette_idx = ((byte >> bit_offset) & 0b11) as usize;

                let offset = palette_idx * 3;

                let palette = self.extra_chunks.palette.unwrap();
                let r = palette[offset];
                let g = palette[offset + 1];
                let b = palette[offset + 2];

                let alpha: u8 = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Palette(data)) => {
                        *data.get(palette_idx).unwrap_or(&255)
                    },
                    Some(_) | None => 255,
                };

                Some((r, g, b, alpha))
            },
            PixelType::Palette4 => {
                let byte = self.scanline[self.pixel_cursor / 2];
                let bit_offset = 4 - ((self.pixel_cursor % 2) * 4);
                let palette_idx = ((byte >> bit_offset) & 0b1111) as usize;

                let offset = palette_idx * 3;

                let palette = self.extra_chunks.palette.unwrap();
                let r = palette[offset];
                let g = palette[offset + 1];
                let b = palette[offset + 2];

                let alpha: u8 = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Palette(data)) => {
                        *data.get(palette_idx).unwrap_or(&255)
                    },
                    Some(_) | None => 255,
                };

                Some((r, g, b, alpha))
            },
            PixelType::Palette8 => {
                let offset = self.scanline[self.pixel_cursor] as usize * 3;

                let palette = self.extra_chunks.palette.unwrap();
                let r = palette[offset];
                let g = palette[offset + 1];
                let b = palette[offset + 2];

                let alpha: u8 = match self.extra_chunks.transparency {
                    Some(TransparencyChunk::Palette(data)) => *data.get(offset).unwrap_or(&255),
                    Some(_) | None => 255,
                };

                Some((r, g, b, alpha))
            },
            PixelType::GrayscaleAlpha8 => {
                let offset = self.pixel_cursor * 2;
                let grayscale_val = self.scanline[offset];
                let alpha = self.scanline[offset + 1];

                Some((grayscale_val, grayscale_val, grayscale_val, alpha))
            },
            PixelType::GrayscaleAlpha16 => {
                let offset = self.pixel_cursor * 4;
                let grayscale_val =
                    u16::from_be_bytes([self.scanline[offset], self.scanline[offset + 1]]);
                let alpha =
                    u16::from_be_bytes([self.scanline[offset + 2], self.scanline[offset + 3]]);

                let grayscale_val = u16_to_u8(grayscale_val);
                let alpha = u16_to_u8(alpha);

                Some((grayscale_val, grayscale_val, grayscale_val, alpha))
            },
            PixelType::RgbAlpha8 => {
                let offset = self.pixel_cursor * 4;
                let r = self.scanline[offset];
                let g = self.scanline[offset + 1];
                let b = self.scanline[offset + 2];
                let a = self.scanline[offset + 3];

                Some((r, g, b, a))
            },
            PixelType::RgbAlpha16 => {
                let offset = self.pixel_cursor * 8;
                let r = u16::from_be_bytes([self.scanline[offset], self.scanline[offset + 1]]);
                let g = u16::from_be_bytes([self.scanline[offset + 2], self.scanline[offset + 3]]);
                let b = u16::from_be_bytes([self.scanline[offset + 4], self.scanline[offset + 5]]);
                let a = u16::from_be_bytes([self.scanline[offset + 6], self.scanline[offset + 7]]);

                let r = u16_to_u8(r);
                let g = u16_to_u8(g);
                let b = u16_to_u8(b);
                let a = u16_to_u8(a);

                Some((r, g, b, a))
            },
        };

        self.pixel_cursor += 1;
        pixel
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum CompressionMethod {
    Deflate = 0,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum FilterMethod {
    Adaptive = 0,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum FilterType {
    None = 0,
    Sub = 1,
    Up = 2,
    Average = 3,
    Paeth = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
pub enum InterlaceMethod {
    None = 0,
    Adam7 = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PngHeader {
    pub width: u32,
    pub height: u32,
    pub bit_depth: BitDepth,
    pub color_type: ColorType,
    pub compression_method: CompressionMethod,
    pub filter_method: FilterMethod,
    pub interlace_method: InterlaceMethod,
}

impl PngHeader {
    fn from_chunk(chunk: &Chunk) -> Result<Self, DecodeError> {
        if chunk.chunk_type != ChunkType::ImageHeader {
            return Err(DecodeError::InvalidChunkType);
        }

        if chunk.data.len() < 13 {
            return Err(DecodeError::MissingBytes);
        }

        let width = read_u32(chunk.data, 0);
        let height = read_u32(chunk.data, 4);
        let bit_depth = chunk.data[8];
        let color_type = chunk.data[9];
        let compression_method = chunk.data[10];
        let filter_method = chunk.data[11];
        let interlace_method = chunk.data[12];

        Ok(PngHeader {
            width,
            height,
            bit_depth: TryFrom::try_from(bit_depth).map_err(|_| DecodeError::InvalidBitDepth)?,
            color_type: TryFrom::try_from(color_type).map_err(|_| DecodeError::InvalidColorType)?,
            compression_method: TryFrom::try_from(compression_method)
                .map_err(|_| DecodeError::InvalidCompressionMethod)?,
            filter_method: TryFrom::try_from(filter_method)
                .map_err(|_| DecodeError::InvalidFilterMethod)?,
            interlace_method: TryFrom::try_from(interlace_method)
                .map_err(|_| DecodeError::InvalidInterlaceMethod)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    InvalidMagicBytes,
    MissingBytes,
    HeaderChunkNotFirst,
    EndChunkNotLast,
    InvalidChunkType,
    InvalidChunk,
    Decompress(TINFLStatus),

    IncorrectChunkCrc,
    InvalidBitDepth,
    InvalidColorType,
    InvalidColorTypeBitDepthCombination,
    InvalidCompressionMethod,
    InvalidFilterMethod,
    InvalidFilterType,
    InvalidInterlaceMethod,

    // The width/height specified in the image contains too many
    // bytes to address with a usize on this platform.
    IntegerOverflow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChunkType {
    ImageHeader,
    Palette,
    Transparency,
    Background,
    Srgb,
    ImageData,
    ImageEnd,
    Gamma,
    Unknown([u8; 4]),
}

impl ChunkType {
    fn from_bytes(bytes: &[u8; 4]) -> Self {
        match bytes {
            b"IHDR" => ChunkType::ImageHeader,
            b"PLTE" => ChunkType::Palette,
            b"tRNS" => ChunkType::Transparency,
            b"bKGD" => ChunkType::Background,
            b"sRGB" => ChunkType::Srgb,
            b"IDAT" => ChunkType::ImageData,
            b"IEND" => ChunkType::ImageEnd,
            b"gAMA" => ChunkType::Gamma,
            unknown_chunk_type => {
                // println!("chunk_type: {:?}", alloc::string::String::from_utf8(chunk_type.to_vec()));
                ChunkType::Unknown(*unknown_chunk_type)
            },
        }
    }
}

#[derive(Debug)]
struct Chunk<'a> {
    chunk_type: ChunkType,
    data: &'a [u8],
    _crc: u32,
}

impl<'a> Chunk<'a> {
    fn byte_size(&self) -> usize {
        // length bytes + chunk type bytes + data bytes + crc bytes
        4 + 4 + self.data.len() + 4
    }
}

enum TransparencyChunk<'a> {
    Palette(&'a [u8]),
    Grayscale(u8),
    Rgb(u8, u8, u8),
}

impl<'a> TransparencyChunk<'a> {
    fn from_chunk(chunk: &Chunk<'a>, pixel_type: PixelType) -> Option<Self> {
        match pixel_type {
            PixelType::Grayscale1 => Some(TransparencyChunk::Grayscale(chunk.data[1] & 0b1)),
            PixelType::Grayscale2 => Some(TransparencyChunk::Grayscale(chunk.data[1] & 0b11)),
            PixelType::Grayscale4 => Some(TransparencyChunk::Grayscale(chunk.data[1] & 0b1111)),
            PixelType::Grayscale8 => Some(TransparencyChunk::Grayscale(chunk.data[1])),
            PixelType::Grayscale16 => {
                let val = u16::from_be_bytes([chunk.data[0], chunk.data[1]]);
                Some(TransparencyChunk::Grayscale(u16_to_u8(val)))
            },
            PixelType::Rgb8 => {
                let r = chunk.data[1];
                let g = chunk.data[3];
                let b = chunk.data[5];
                Some(TransparencyChunk::Rgb(r, g, b))
            },
            PixelType::Rgb16 => {
                let r = u16::from_be_bytes([chunk.data[0], chunk.data[1]]);
                let g = u16::from_be_bytes([chunk.data[2], chunk.data[3]]);
                let b = u16::from_be_bytes([chunk.data[4], chunk.data[5]]);
                Some(TransparencyChunk::Rgb(u16_to_u8(r), u16_to_u8(g), u16_to_u8(b)))
            },
            PixelType::Palette1 => Some(TransparencyChunk::Palette(chunk.data)),
            PixelType::Palette2 => Some(TransparencyChunk::Palette(chunk.data)),
            PixelType::Palette4 => Some(TransparencyChunk::Palette(chunk.data)),
            PixelType::Palette8 => Some(TransparencyChunk::Palette(chunk.data)),
            PixelType::GrayscaleAlpha8 => None,
            PixelType::GrayscaleAlpha16 => None,
            PixelType::RgbAlpha8 => None,
            PixelType::RgbAlpha16 => None,
        }
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}

fn read_chunk(bytes: &[u8]) -> Result<Chunk<'_>, DecodeError> {
    if bytes.len() < 4 {
        return Err(DecodeError::MissingBytes);
    }

    let length = read_u32(bytes, 0) as usize;
    let bytes = &bytes[4..];

    if bytes.len() < (4 + length + 4) {
        return Err(DecodeError::MissingBytes);
    }

    let chunk_type = ChunkType::from_bytes(&[bytes[0], bytes[1], bytes[2], bytes[3]]);

    let crc_offset = 4 + length;
    let crc = read_u32(bytes, crc_offset);

    // Offset by 4 to not include the chunk type.
    let data_for_crc = &bytes[..crc_offset];

    let mut hasher = Hasher::new();
    hasher.reset();
    hasher.update(data_for_crc);

    if crc != hasher.finalize() {
        return Err(DecodeError::IncorrectChunkCrc);
    }

    Ok(Chunk { chunk_type, data: &data_for_crc[4..], _crc: crc })
}

fn defilter(
    filter_type: FilterType,
    bytes_per_pixel: usize,
    x: usize,
    current_scanline: &[u8],
    last_scanline: &[u8],
) -> u8 {
    match filter_type {
        FilterType::None => current_scanline[x],
        FilterType::Sub => {
            if let Some(idx) = x.checked_sub(bytes_per_pixel) {
                current_scanline[x].wrapping_add(current_scanline[idx])
            } else {
                current_scanline[x]
            }
        },
        FilterType::Up => current_scanline[x].wrapping_add(last_scanline[x]),
        FilterType::Average => {
            let raw_val = if let Some(idx) = x.checked_sub(bytes_per_pixel) {
                current_scanline[idx]
            } else {
                0
            };

            (current_scanline[x] as u16 + ((raw_val as u16 + last_scanline[x] as u16) / 2)) as u8
        },
        FilterType::Paeth => {
            if let Some(idx) = x.checked_sub(bytes_per_pixel) {
                let left = current_scanline[idx];
                let above = last_scanline[x];
                let upper_left = last_scanline[idx];

                let predictor = paeth_predictor(left as i16, above as i16, upper_left as i16);

                current_scanline[x].wrapping_add(predictor)
            } else {
                let left = 0;
                let above = last_scanline[x];
                let upper_left = 0;

                let predictor = paeth_predictor(left as i16, above as i16, upper_left as i16);

                current_scanline[x].wrapping_add(predictor)
            }
        },
    }
}

fn process_scanlines(
    header: &PngHeader,
    scanline_data: &mut [u8],
    output_rgba: &mut [[u8; 4]],
    ancillary_chunks: &AncillaryChunks,
    pixel_type: PixelType,
) -> Result<(), DecodeError> {
    let mut cursor = 0;
    let bytes_per_pixel: usize =
        (header.bit_depth as usize * header.color_type.sample_multiplier()).div_ceil(8);

    match header.interlace_method {
        InterlaceMethod::None => {
            // TODO(bschwind) - Deduplicate this logic.
            let bytes_per_scanline = (header.width as u64
                * header.bit_depth as u64
                * header.color_type.sample_multiplier() as u64)
                .div_ceil(8);
            let bytes_per_scanline: usize =
                bytes_per_scanline.try_into().map_err(|_| DecodeError::IntegerOverflow)?;

            let mut last_scanline = vec![0u8; bytes_per_scanline];

            for y in 0..header.height {
                let filter_type = FilterType::try_from(scanline_data[cursor])
                    .map_err(|_| DecodeError::InvalidFilterType)?;
                cursor += 1;

                let current_scanline = &mut scanline_data[cursor..(cursor + bytes_per_scanline)];

                for x in 0..(bytes_per_scanline) {
                    let unfiltered_byte =
                        defilter(filter_type, bytes_per_pixel, x, current_scanline, &last_scanline);
                    current_scanline[x] = unfiltered_byte;
                }

                let scanline_iter = ScanlineIterator::new(
                    header.width,
                    pixel_type,
                    current_scanline,
                    ancillary_chunks,
                );

                for (idx, (r, g, b, a)) in scanline_iter.enumerate() {
                    let (output_x, output_y) = (idx, y);

                    let output_idx = (output_y as u64 * header.width as u64) + (output_x as u64);
                    let output_idx: usize =
                        output_idx.try_into().map_err(|_| DecodeError::IntegerOverflow)?;

                    output_rgba[output_idx] = [r, g, b, a];
                }

                last_scanline.copy_from_slice(current_scanline);
                cursor += bytes_per_scanline;
            }
        },
        InterlaceMethod::Adam7 => {
            let max_bytes_per_scanline = header.width as usize * bytes_per_pixel;
            let mut last_scanline = vec![0u8; max_bytes_per_scanline];

            // Adam7 Interlacing Pattern
            // 1 6 4 6 2 6 4 6
            // 7 7 7 7 7 7 7 7
            // 5 6 5 6 5 6 5 6
            // 7 7 7 7 7 7 7 7
            // 3 6 4 6 3 6 4 6
            // 7 7 7 7 7 7 7 7
            // 5 6 5 6 5 6 5 6
            // 7 7 7 7 7 7 7 7

            for pass in 1..=7 {
                let (pass_width, pass_height) = match pass {
                    1 => {
                        let pass_width = header.width.div_ceil(8);
                        let pass_height = header.height.div_ceil(8);
                        (pass_width, pass_height)
                    },
                    2 => {
                        let pass_width = (header.width / 8) + ((header.width % 8) / 5);
                        let pass_height = header.height.div_ceil(8);
                        (pass_width, pass_height)
                    },
                    3 => {
                        let pass_width = ((header.width / 8) * 2) + (header.width % 8).div_ceil(4);
                        let pass_height = (header.height / 8) + ((header.height % 8) / 5);
                        (pass_width, pass_height)
                    },
                    4 => {
                        let pass_width = ((header.width / 8) * 2) + (header.width % 8 + 1) / 4;
                        let pass_height = header.height.div_ceil(4);
                        (pass_width, pass_height)
                    },
                    5 => {
                        let pass_width = (header.width / 2) + (header.width % 2);
                        let pass_height = ((header.height / 8) * 2) + (header.height % 8 + 1) / 4;
                        (pass_width, pass_height)
                    },
                    6 => {
                        let pass_width = header.width / 2;
                        let pass_height = (header.height / 2) + (header.height % 2);
                        (pass_width, pass_height)
                    },
                    7 => {
                        let pass_width = header.width;
                        let pass_height = header.height / 2;
                        (pass_width, pass_height)
                    },
                    _ => (0, 0),
                };

                // Skip empty passes.
                if pass_width == 0 || pass_height == 0 {
                    continue;
                }

                let bytes_per_scanline = (pass_width as u64
                    * header.bit_depth as u64
                    * header.color_type.sample_multiplier() as u64)
                    .div_ceil(8);
                let bytes_per_scanline: usize =
                    bytes_per_scanline.try_into().expect("bytes_per_scanline overflowed a usize");

                let last_scanline = &mut last_scanline[..(bytes_per_scanline)];
                for byte in last_scanline.iter_mut() {
                    *byte = 0;
                }

                for y in 0..pass_height {
                    let filter_type = FilterType::try_from(scanline_data[cursor])
                        .map_err(|_| DecodeError::InvalidFilterType)?;
                    cursor += 1;

                    let current_scanline =
                        &mut scanline_data[cursor..(cursor + bytes_per_scanline)];

                    for x in 0..(bytes_per_scanline) {
                        let unfiltered_byte = defilter(
                            filter_type,
                            bytes_per_pixel,
                            x,
                            current_scanline,
                            last_scanline,
                        );
                        current_scanline[x] = unfiltered_byte;
                    }

                    let scanline_iter = ScanlineIterator::new(
                        pass_width,
                        pixel_type,
                        current_scanline,
                        ancillary_chunks,
                    );

                    for (idx, (r, g, b, a)) in scanline_iter.enumerate() {
                        // Put rgba in output_rgba
                        let (output_x, output_y) = match pass {
                            1 => (idx * 8, y * 8),
                            2 => (idx * 8 + 4, y * 8),
                            3 => (idx * 4, y * 8 + 4),
                            4 => (idx * 4 + 2, y * 4),
                            5 => (idx * 2, y * 4 + 2),
                            6 => (idx * 2 + 1, y * 2),
                            7 => (idx, y * 2 + 1),
                            _ => (0, 0),
                        };

                        let output_idx =
                            (output_y as u64 * header.width as u64) + (output_x as u64);
                        let output_idx: usize =
                            output_idx.try_into().map_err(|_| DecodeError::IntegerOverflow)?;

                        output_rgba[output_idx] = [r, g, b, a];
                    }

                    last_scanline.copy_from_slice(current_scanline);

                    cursor += bytes_per_scanline;
                }
            }
        },
    }

    Ok(())
}

fn paeth_predictor(a: i16, b: i16, c: i16) -> u8 {
    // TODO(bschwind) - Accept i16 or convert once and store in a temp.
    // a = left pixel
    // b = above pixel
    // c = upper left
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();

    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}

/// Decodes the provided PNG into RGBA pixels.
///
/// The returned [`PngHeader`] contains the image’s size, and other PNG metadata which is not
/// necessary to make use of the pixels (the returned format is always 8-bit-per-component RGBA).
///
/// The returned [`Vec`] contains the pixels, represented as `[r, g, b, a]` arrays.
/// Its length will be equal to `header.width * header.height`.
/// (If you need a `Vec<u8>` of length `header.width * header.height * 4` instead, you can use
/// [`Vec::into_flattened()`] to convert it.)
pub fn decode(bytes: &[u8]) -> Result<(PngHeader, Vec<[u8; 4]>), DecodeError> {
    if bytes.len() < PNG_MAGIC_BYTES.len() {
        return Err(DecodeError::MissingBytes);
    }

    if &bytes[0..PNG_MAGIC_BYTES.len()] != PNG_MAGIC_BYTES {
        return Err(DecodeError::InvalidMagicBytes);
    }

    let bytes = &bytes[PNG_MAGIC_BYTES.len()..];

    let header_chunk = read_chunk(bytes)?;
    let header = PngHeader::from_chunk(&header_chunk)?;

    let mut bytes = &bytes[header_chunk.byte_size()..];

    let mut compressed_data: Vec<u8> =
        Vec::with_capacity(header.width as usize * header.height as usize * 3);

    let pixel_type = PixelType::new(header.color_type, header.bit_depth)?;
    let mut ancillary_chunks = AncillaryChunks::default();

    while !bytes.is_empty() {
        let chunk = read_chunk(bytes)?;

        match chunk.chunk_type {
            ChunkType::ImageData => compressed_data.extend_from_slice(chunk.data),
            ChunkType::Palette => ancillary_chunks.palette = Some(chunk.data),
            ChunkType::Transparency => {
                ancillary_chunks.transparency = TransparencyChunk::from_chunk(&chunk, pixel_type)
            },
            ChunkType::Background => ancillary_chunks.background = Some(chunk.data),
            ChunkType::ImageEnd => break,
            _ => {},
        }

        bytes = &bytes[chunk.byte_size()..];
    }

    let mut scanline_data = miniz_oxide::inflate::decompress_to_vec_zlib(&compressed_data)
        .map_err(|miniz_oxide::inflate::DecompressError { status, output: _ }| {
            DecodeError::Decompress(status)
        })?;

    // For now, output data is always RGBA, 1 byte per channel.
    let mut output_rgba = vec![[0u8; 4]; header.width as usize * header.height as usize];

    process_scanlines(
        &header,
        &mut scanline_data,
        &mut output_rgba,
        &ancillary_chunks,
        pixel_type,
    )?;

    Ok((header, output_rgba))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_suite_test() {
        use image::EncodableLayout;

        for entry in
            std::fs::read_dir("test_pngs/png_suite").expect("Shaders directory should exist")
        {
            let entry = entry.unwrap();
            let path = entry.path();

            if let Some(extension) = path.extension().and_then(|os_str| os_str.to_str()) {
                if extension.to_ascii_lowercase().as_str() == "png" {
                    let png_bytes = std::fs::read(&path).unwrap();

                    let (_header, decoded): (PngHeader, Vec<[u8; 4]>) = if path
                        .file_stem()
                        .expect("expected png path to be a file")
                        .to_string_lossy()
                        .starts_with('x')
                    {
                        assert!(decode(&png_bytes).is_err());
                        continue;
                    } else {
                        decode(&png_bytes).unwrap()
                    };
                    let decoded: Vec<u8> = decoded.into_flattened();

                    // Uncomment to inspect output.png for debugging.
                    // let image_buf: image::ImageBuffer<image::Rgba<u8>, _> =
                    //     image::ImageBuffer::from_vec(
                    //         _header.width,
                    //         _header.height,
                    //         decoded.clone(),
                    //     )
                    //     .unwrap();

                    // image_buf.save("output.png").unwrap();

                    let comparison_image = image::open(path).unwrap();
                    let comarison_rgba8 = comparison_image.to_rgba8();

                    let comparison_bytes = comarison_rgba8.as_bytes();
                    assert_eq!(decoded.len(), comparison_bytes.len());

                    for (idx, (test_byte, comparison_byte)) in
                        decoded.iter().zip(comparison_bytes.iter()).enumerate()
                    {
                        let start_idx = idx.saturating_sub(16);
                        let end_idx = (idx + 16).min(decoded.len());
                        assert_eq!(test_byte, comparison_byte, "incorrect byte at index {}, decoded slice: {:?}, comparison_slice: {:?}", idx, &decoded[start_idx..end_idx], &comparison_bytes[start_idx..end_idx]);
                    }
                }
            }
        }
    }

    #[test]
    fn test_trailing_zero() {
        let path = "test_pngs/trailing_zero.png";
        let png_bytes = std::fs::read(path).unwrap();
        let (_header, _decoded) = decode(&png_bytes)
            .expect("A PNG with trailing zeroes after the ImageEnd chunk should be readable");
    }
}
