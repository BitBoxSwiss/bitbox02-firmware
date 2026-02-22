fn main() {
    let image_bytes = include_bytes!("../test_pngs/paeth.png");
    let (header, image_data) = png_decoder::decode(image_bytes).unwrap();

    println!("Header: {:#?}", header);
    println!("Image data size: {}x{}x4 {}", header.width, header.height, image_data.len());
}
