use std::error::Error;

use eframe::epaint::RectShape;
use egui::{Color32, FontData, FontDefinitions, FontFamily, Key, Pos2, Rect, Rounding, TextStyle};

use crate::{
    geometry::Block,
    tetris::{DisplayState, Event},
};

pub fn build_shapes(
    blocks: &[Block],
    block_size: f32,
    reference: Pos2,
    fill: Color32,
    stroke: Color32,
) -> Vec<egui::Shape> {
    blocks
        .iter()
        .map(|block| {
            egui::Shape::Rect(RectShape {
                rect: Rect {
                    min: Pos2 {
                        x: (block.x as f32 * block_size) + reference.x,
                        y: (block.y as f32 * block_size) + reference.y,
                    },
                    max: Pos2 {
                        x: (1. + block.x as f32) * block_size + reference.x,
                        y: (1. + block.y as f32) * block_size + reference.y,
                    },
                },
                rounding: Rounding::default(),
                fill,
                stroke: egui::Stroke::new(1.0, stroke),
            })
        })
        .collect()
}

pub fn build_scene_shapes(
    state: &DisplayState,
    block_size: f32,
    reference: Pos2,
) -> Vec<egui::Shape> {
    build_shapes(
        &state.player,
        block_size,
        reference,
        Color32::KHAKI,
        Color32::BLACK,
    )
    .into_iter()
    .chain(
        build_shapes(
            &state.projection,
            block_size,
            reference,
            Color32::TRANSPARENT,
            Color32::WHITE,
        )
        .into_iter(),
    )
    .chain(
        build_shapes(
            &state.blocks,
            block_size,
            reference,
            Color32::DARK_RED,
            Color32::BLACK,
        )
        .into_iter(),
    )
    .collect::<Vec<_>>()
}

pub fn gather_input(ctx: &egui::Context) -> Option<Event> {
    if ctx.input().key_pressed(Key::ArrowRight) {
        return Some(Event::MoveRight);
    } else if ctx.input().key_pressed(Key::ArrowLeft) {
        return Some(Event::MoveLeft);
    } else if ctx.input().key_pressed(Key::ArrowUp) {
        return Some(Event::Rotate);
    } else if ctx.input().key_pressed(Key::ArrowDown) {
        return Some(Event::MoveDown);
    } else if ctx.input().key_pressed(Key::Space) {
        return Some(Event::Drop);
    }
    None
}

pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

#[derive(Debug)]
pub struct UiSetupError;
impl std::fmt::Display for UiSetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: UI Setup")
    }
}
impl Error for UiSetupError {}

pub fn set_style(cc: &eframe::CreationContext<'_>) -> Result<(), UiSetupError> {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "milky_coffee".to_owned(),
        FontData::from_static(include_bytes!("../resources/Milky_Coffee.ttf")),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .ok_or(UiSetupError)?
        .insert(0, "milky_coffee".to_owned());

    cc.egui_ctx.set_fonts(fonts);

    let mut style: egui::Style = (*cc.egui_ctx.style()).clone();
    style
        .text_styles
        .get_mut(&TextStyle::Body)
        .ok_or(UiSetupError)?
        .size = 24.;
    style
        .text_styles
        .get_mut(&TextStyle::Button)
        .ok_or(UiSetupError)?
        .size = 20.;
    cc.egui_ctx.set_style(style);
    Ok(())
}
