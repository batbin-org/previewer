use conv::ValueInto;
use image::Pixel;
use imageproc::definitions::Clamp;
use imageproc::drawing::Canvas;
use imageproc::pixelops::weighted_sum;
use rusttype::{point, Font, Scale};
use std::cmp::max;

// custom draw_text forked from improc
pub fn draw_text_mut_w<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: i32,
    y: i32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
) -> i32
where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let imw = canvas.width() as i32;
    let imh = canvas.height() as i32;
    let offset = point(0.0, font.v_metrics(scale).ascent);

    let mut mxw = 0;
    for g in font.layout(text, scale, offset) {
        if let Some(bb) = g.pixel_bounding_box() {
            mxw = max(mxw, bb.max.x);
            g.draw(|gx, gy, gv| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;

                let imx = gx + x;
                let imy = gy + y;

                if imx >= 0 && imx < imw && imy >= 0 && imy < imh {
                    let pixel = canvas.get_pixel(imx as u32, imy as u32);
                    let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                    canvas.draw_pixel(imx as u32, imy as u32, weighted_color);
                }
            })
        }
    }

    return mxw;
}
