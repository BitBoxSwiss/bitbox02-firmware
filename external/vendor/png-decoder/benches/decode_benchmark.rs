use criterion::{criterion_group, criterion_main, Criterion};
use image::{codecs::png::PngDecoder, ImageDecoder};

const SCREENSHOT_PNG_BYTES: &[u8] = include_bytes!("../test_pngs/skyline.png");
const TRANSPARENT_TEXTURE_PNG_BYTES: &[u8] = include_bytes!("../test_pngs/512x512_transparent.png");

fn decode_screenshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("Screenshot Decode (1080x1920)");

    group.bench_function("png-decoder", |b| {
        b.iter(|| {
            png_decoder::decode(SCREENSHOT_PNG_BYTES).unwrap();
        });
    });

    group.bench_function("image", |b| {
        b.iter(|| {
            let decoder = PngDecoder::new(SCREENSHOT_PNG_BYTES).unwrap();
            let mut buf: Vec<u8> = vec![0; decoder.total_bytes() as usize];
            decoder.read_image(&mut buf).unwrap();
        });
    });

    group.finish();
}

fn decode_transparent_texture(c: &mut Criterion) {
    let mut group = c.benchmark_group("Transparent Texture Decode (512x512)");

    group.bench_function("png-decoder", |b| {
        b.iter(|| {
            png_decoder::decode(TRANSPARENT_TEXTURE_PNG_BYTES).unwrap();
        });
    });

    group.bench_function("image", |b| {
        b.iter(|| {
            let decoder = PngDecoder::new(TRANSPARENT_TEXTURE_PNG_BYTES).unwrap();
            let mut buf: Vec<u8> = vec![0; decoder.total_bytes() as usize];
            decoder.read_image(&mut buf).unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, decode_screenshot, decode_transparent_texture);
criterion_main!(benches);
