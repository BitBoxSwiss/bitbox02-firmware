use delog::{hex_str, hexstr};

fn main() {
    let buf = [1u8, 2, 3, 0xA1, 0xB7, 0xFF, 0x3];
    println!("'{}'", hexstr!(&buf));
    println!("'{}'", hex_str!(&buf));
    println!("'{:4}'", hex_str!(&buf));
    println!("'{:<4}'", hex_str!(&buf));
    println!("'{:>4}'", hex_str!(&buf));
    println!("'{}'", hex_str!(&buf, 2));
    println!("'{:02x}'", hex_str!(&buf, 2));
    println!("'{}'", hex_str!(&buf, 4));
    println!("'{:4}'", hex_str!(&buf, 2));
    println!("'{:<4}'", hex_str!(&buf, 2));
    println!("'{:>4}'", hex_str!(&buf, 2));
    println!("'{}'", hex_str!(&buf[..], 4));
    println!("'{}'", hex_str!(&buf, 3));
    println!("'{}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:2}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:3x}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:>3x}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:<3x}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:4}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:5x}'", hex_str!(&buf, 2, sep: "|"));
    println!("'{:6}'", hex_str!(&buf, 2, sep: "|"));
}
