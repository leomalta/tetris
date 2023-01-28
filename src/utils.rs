use crate::{
    geometry::Position,
    tetris::{DisplayState, Event},
};
use eframe::epaint::RectShape;
use egui::{
    pos2, Color32, FontData, FontDefinitions, FontFamily, Key, Pos2, Rect, Rounding, Style,
    TextStyle,
};

/// Returns the egui Rect position ready to be painted by the GUI
fn get_rect(block_pos: Position, block_size: f32, gui_ref_pos: Pos2) -> Rect {
    Rect::from_min_max(
        pos2(
            (block_pos.x as f32 * block_size) + gui_ref_pos.x,
            (block_pos.y as f32 * block_size) + gui_ref_pos.y,
        ),
        pos2(
            (1. + block_pos.x as f32) * block_size + gui_ref_pos.x,
            (1. + block_pos.y as f32) * block_size + gui_ref_pos.y,
        ),
    )
}

/// Returns an iterator over all shapes from the blocks in input, ready to be painted
pub fn build_blocks(
    blocks: &[Position],
    block_size: f32,
    reference: Pos2,
    fill: Color32,
    stroke: Color32,
) -> impl Iterator<Item = egui::Shape> + '_ {
    blocks.iter().map(move |&block| {
        RectShape {
            rect: get_rect(block, block_size, reference),
            rounding: Rounding::default(),
            fill,
            stroke: egui::Stroke::new(1.0, stroke),
        }
        .into()
    })
}

/// Returns an iterator over all shapes from the blocks of:
/// - player tetronimo
/// - projection
/// - stash of blocks in the scene
pub fn build_game_blocks(
    state: &DisplayState,
    block_size: f32,
    reference: Pos2,
) -> impl Iterator<Item = egui::Shape> + '_ {
    build_blocks(
        &state.player,
        block_size,
        reference,
        Color32::KHAKI,
        Color32::BLACK,
    )
    .chain(build_blocks(
        &state.projection,
        block_size,
        reference,
        Color32::TRANSPARENT,
        Color32::WHITE,
    ))
    .chain(build_blocks(
        &state.blocks,
        block_size,
        reference,
        Color32::DARK_RED,
        Color32::BLACK,
    ))
}

/// Get the user input event from the Context, if any
pub fn get_input_from_context(ctx: &egui::Context) -> Option<Event> {
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

/// Load an image from the specified path
pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    image::io::Reader::open(path)?.decode().map(|image| {
        egui::ColorImage::from_rgba_unmultiplied(
            [image.width() as _, image.height() as _],
            image.to_rgba8().as_flat_samples().as_slice(),
        )
    })
}

/// Sets the font type and size for the GUI
pub fn set_font_style(cc: &eframe::CreationContext<'_>) {
    // Sets font type
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "milky_coffee".to_owned(),
        FontData::from_static(include_bytes!("../resources/Milky_Coffee.ttf")),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "milky_coffee".to_owned());
    cc.egui_ctx.set_fonts(fonts);

    // Sets font sizes
    let mut style = Style::default();
    style.text_styles.entry(TextStyle::Body).or_default().size = 24.;
    style.text_styles.entry(TextStyle::Button).or_default().size = 20.;
    cc.egui_ctx.set_style(style);
}
