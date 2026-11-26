// SPDX-License-Identifier: Apache-2.0

#[cfg(any(
    not(any(
        feature = "product-bitbox02",
        feature = "product-bitbox02nova",
        feature = "product-bitbox03"
    )),
    all(feature = "product-bitbox02", feature = "product-bitbox02nova"),
    all(feature = "product-bitbox02", feature = "product-bitbox03"),
    all(feature = "product-bitbox02nova", feature = "product-bitbox03"),
))]
compile_error!("Select exactly one simulator product feature");

#[cfg(any(
    not(any(feature = "edition-multi", feature = "edition-btc-only")),
    all(feature = "edition-multi", feature = "edition-btc-only"),
))]
compile_error!("Select exactly one simulator edition feature");

#[cfg(not(feature = "product-bitbox03"))]
mod bitbox02;
#[cfg(not(feature = "product-bitbox03"))]
pub use bitbox02::*;
#[cfg(feature = "product-bitbox03")]
mod bitbox03;
#[cfg(feature = "product-bitbox03")]
pub use bitbox03::*;

use femtovg::{Canvas, ImageId, Paint, Path, renderer::OpenGl};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

pub struct Geometry {
    screen_width: usize,
    screen_height: usize,
    padding_left: usize,
    padding_right: usize,
    padding_top: usize,
    padding_bottom: usize,
    margin: usize,
    pub initial_scale: f64,
    background: [u8; 3],
}

impl Geometry {
    pub fn window_size(&self) -> (usize, usize) {
        (
            self.screen_width + 2 * self.margin + self.padding_left + self.padding_right,
            self.screen_height + 2 * self.margin + self.padding_top + self.padding_bottom,
        )
    }

    fn scale(&self, size: PhysicalSize<u32>) -> (f32, f32) {
        let (width, height) = self.window_size();
        (
            size.width as f32 / width as f32,
            size.height as f32 / height as f32,
        )
    }

    pub fn window_to_logical(
        &self,
        window: &Window,
        position: PhysicalPosition<f64>,
    ) -> (i32, i32) {
        let (width_scale, height_scale) = self.scale(window.inner_size());
        (
            (position.x as f32 / width_scale) as i32,
            (position.y as f32 / height_scale) as i32,
        )
    }

    pub fn screen_rect(&self, size: PhysicalSize<u32>) -> [f32; 4] {
        let (width_scale, height_scale) = self.scale(size);
        [
            (self.margin + self.padding_left) as f32 * width_scale,
            (self.margin + self.padding_top) as f32 * height_scale,
            self.screen_width as f32 * width_scale,
            self.screen_height as f32 * height_scale,
        ]
    }

    pub fn render_background(
        &self,
        canvas: &mut Canvas<OpenGl>,
        bg_id: ImageId,
        size: PhysicalSize<u32>,
    ) {
        // fill background
        let mut bg = Path::new();
        bg.rect(0.0, 0.0, size.width as f32, size.height as f32);
        let [r, g, b] = self.background;
        canvas.fill_path(&bg, &Paint::color(femtovg::Color::rgb(r, g, b)));

        let (width_scale, height_scale) = self.scale(size);
        let x = self.margin as f32 * width_scale;
        let y = self.margin as f32 * height_scale;
        let width =
            (self.padding_left + self.screen_width + self.padding_right) as f32 * width_scale;
        let height =
            (self.padding_top + self.screen_height + self.padding_bottom) as f32 * height_scale;
        let mut device = Path::new();
        device.rect(x, y, width, height);
        canvas.fill_path(&device, &Paint::image(bg_id, x, y, width, height, 0.0, 1.0));
    }
}
