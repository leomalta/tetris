use crate::geometry::Position;
use crate::tetris::{DisplayState, Event, Tetris};
use crate::utils::*;
use eframe::egui;
use egui::{mutex::Mutex, vec2, Color32, Context, Vec2};
use std::sync::Arc;

pub struct GameInterface {
    // Texture to hold the image of tetris logo
    logo: egui::TextureHandle,
    // Game engine
    engine: Arc<Mutex<Tetris>>,
    // display size of each tetris block
    block_size: f32,
}

const FRAME_BORDER: f32 = 1.25;
const STATS_PANEL_WIDTH: f32 = 4.0;

impl eframe::App for GameInterface {
    /// Main thread drawing function (event entry point)
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // get the user input event in context, if any
        let input_event = get_input_from_context(ctx);

        // run the event and get the display state from the engine
        let state = self.run_and_get_display_state(input_event);

        // set window size base on the game state area
        frame.set_window_size(vec2(
            (state.scene_area.x as f32 + FRAME_BORDER + STATS_PANEL_WIDTH) * self.block_size,
            (state.scene_area.y as f32 + FRAME_BORDER) * self.block_size,
        ));

        // draw the state
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_stats(ui, &state);
            self.show_game(ui, &state);
        });
    }
}

impl GameInterface {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // set the font style and size
        set_font_style(cc);

        Self {
            // Load the logo texture (only once, at object creation)
            logo: cc.egui_ctx.load_texture(
                "logo",
                load_image_from_path(std::path::Path::new("./resources/logo.png"))
                    .unwrap_or_default(),
                egui::TextureFilter::default(),
            ),
            // Create the game engine to be shared via a mutex
            engine: Arc::new(Mutex::new(Tetris::new(Position::new(10, 20)))),
            block_size: 25.,
        }
    }

    /// Access the game engine to run an event and retrive the resulting game state
    fn run_and_get_display_state(&mut self, input: Option<Event>) -> DisplayState {
        // Lock the engine in this thread
        let mut game = self.engine.lock();

        // execute user input, if any
        input.and_then(|event| game.run(event));

        // return the display state (and release lock on engine)
        game.get_display_state()
    }

    /// Build and draw the right panel (stats) using the game display state
    fn show_stats(&self, ui: &mut egui::Ui, state: &DisplayState) {
        // Define the drawer for the next tetronimo
        let next_tetronimo_drawer = |ui: &mut egui::Ui| {
            // allocate the painter area (4x4 blocks)
            let (response, painter) = ui.allocate_painter(
                Vec2::splat(STATS_PANEL_WIDTH * self.block_size),
                egui::Sense::focusable_noninteractive(),
            );
            // add the shapes of the next tetronimo to the painter
            painter.extend(
                build_blocks(
                    &state.next,
                    self.block_size,
                    response.rect.left_top(),
                    Color32::GOLD,
                    Color32::BLACK,
                )
                .collect(),
            );
            response
        };
        // Define the drawer for the entire stats panel
        let stats_panel_drawer = |ui: &mut egui::Ui| {
            // Next tetronimo area (using the drawer defined above)
            ui.label("Next:");
            egui::Frame::canvas(ui.style()).show(ui, next_tetronimo_drawer);

            // Total score area:
            ui.separator();
            ui.label(format!("Score: {:?}", state.score));

            // Logo area (square with the size of the panel width)
            ui.separator();
            ui.image(&self.logo, Vec2::splat(ui.available_width()));

            // Start and reset buttons
            ui.separator();
            if ui.button("Start").clicked() {
                // Create the thread and start running the game engine
                GameInterface::start(Arc::clone(&self.engine), ui.ctx().clone());
            };
            if ui.button("Reset").clicked() {
                // Reset the engine state
                self.engine.lock().reset();
            };
        };
        // Finally, draw the right-side panel (using the drawer defined above)
        egui::SidePanel::right("stats").show_inside(ui, |ui| {
            ui.set_width(STATS_PANEL_WIDTH * self.block_size);
            ui.vertical_centered(stats_panel_drawer);
        });
    }

    /// Build and draw the central panel (main game area) using the game display state
    fn show_game(&self, ui: &mut egui::Ui, state: &DisplayState) {
        let game_scene_drawer = |ui: &mut egui::Ui| {
            // allocate the painter area (main game area)
            let (response, painter) = ui.allocate_painter(
                Vec2::new(
                    state.scene_area.x as f32 * self.block_size,
                    state.scene_area.y as f32 * self.block_size,
                ),
                egui::Sense::hover(),
            );
            // add the block shapes of the whole scence to the painter
            painter.extend(
                build_game_blocks(state, self.block_size, response.rect.left_top()).collect(),
            );
            response
        };
        // Draw the central panel (passing the drawer defined above)
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(ui.style()))
            .show_inside(ui, game_scene_drawer);
    }

    /// Start the thread running the game engine
    fn start(game: Arc<Mutex<Tetris>>, ctx: Context) {
        std::thread::spawn(move || {
            // local one-line function to get the lock on the engine and run an event
            // this ensures the lock is freed right after each execution
            let lock_and_run = || game.lock().run(Event::MoveDown);

            // Loop running a MoveDown event and waiting a given amount of time,
            // while game engine active
            while let Some(interval) = lock_and_run() {
                ctx.request_repaint();
                std::thread::sleep(interval);
            }
        });
    }
}
