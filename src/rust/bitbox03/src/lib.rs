pub mod display {
    pub struct Rect {
        pub x: i32,
        pub y: i32,
        pub w: u16,
        pub h: u16,
    }
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    pub struct Rgb565(pub u16);

    pub struct Image<'a> {
        pub w: u16,
        pub h: u16,
        pub pixels: &'a [u16],
    }
    pub trait Display {
        type Error;

        fn size(&self) -> (u16, u16);

        fn clear(&mut self, c: Rgb565) -> Result<(), Self::Error>;
        fn fill_rect(&mut self, r: Rect) -> Result<(), Self::Error>;
        fn blit(&mut self) -> Result<(), Self::Error>;

        //fn present(&mut self) -> Result<(), Self::Error>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
