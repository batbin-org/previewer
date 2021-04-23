use rusttype::{point, Font, PositionedGlyph, Rect, Scale};
use std::cmp::max;
use conv::ValueInto;
use image::Pixel;
use imageproc::definitions::Clamp;
use imageproc::drawing::Canvas;
use imageproc::pixelops::weighted_sum;

// These functions were taken from imageproc so that the
// draw_text_mut function could be slightly modified to 
// return the width used

fn layout_glyphs(
    scale: Scale,
    font: &Font,
    text: &str,
    mut f: impl FnMut(PositionedGlyph, Rect<i32>),
) -> (i32, i32) {
    let v_metrics = font.v_metrics(scale);

    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = max(w, bb.max.x);
            h = max(h, bb.max.y);
            f(g, bb);
        }
    }

    (w, h)
}

pub fn draw_text_mut_w<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: i32,
    y: i32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &'a str,
) -> i32 where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let image_width = canvas.width() as i32;
    let image_height = canvas.height() as i32;

    let (w, _) = layout_glyphs(scale, font, text, |g, bb| {
        g.draw(|gx, gy, gv| {
            let gx = gx as i32 + bb.min.x;
            let gy = gy as i32 + bb.min.y;

            let image_x = gx + x;
            let image_y = gy + y;

            if (0..image_width).contains(&image_x) && (0..image_height).contains(&image_y) {
                let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
            }
        })
    });

    return w;
}