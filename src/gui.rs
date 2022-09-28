use crate::geometry::Area;
use crate::tetris::{DisplayState, Event, Game};
use crate::utils::*;
use eframe::egui;
use egui::{mutex::Mutex, vec2, Color32, Context, Vec2};
use std::sync::Arc;

pub struct MyApp {
    logo: egui::TextureHandle,
    game: Arc<Mutex<Game>>,
    block_size: f32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let state;
        {
            // set window size
            let mut game = self.game.lock();
            frame.set_window_size(vec2(
                (game.area.max_x as f32 + 5.75) * self.block_size,
                (game.area.max_y as f32 + 0.75) * self.block_size,
            ));

            // ----------- get the input-------------
            if let Some(event) = gather_input(ctx) {
                game.run(event);
            }

            // ----------- get the display state-------------
            state = game.get_display_state();
        }

        // ----------- create the game panel -------------
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_stats(ui, &state);
            self.show_game(ui, &state);
        });
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        match set_style(cc) {
            Ok(_) => (),
            Err(error) => {
                println!("{}", error);
            }
        }

        Self {
            logo: cc.egui_ctx.load_texture(
                "logo",
                load_image_from_path(std::path::Path::new("./resources/logo.png"))
                    .unwrap_or_default(),
                egui::TextureFilter::default(),
            ),
            game: Arc::new(Mutex::new(Game::new(Area::new(10, 20)))),
            block_size: 25.,
        }
    }

    fn start(&self, ctx: Context) {
        self.game.lock().active = true;
        let game = Arc::clone(&self.game);
        std::thread::spawn(move || {
            let mut level = 0;
            while game.lock().active {
                std::thread::sleep(std::time::Duration::from_millis(1500 / (level + 3)));
                let mut game = game.lock();
                level = game.get_level();
                game.run(Event::MoveDown);
                ctx.request_repaint();
            }
        });
    }

    fn show_stats(&self, ui: &mut egui::Ui, state: &DisplayState) {
        let next_tetronimo_drawer = |ui: &mut egui::Ui| {
            let (response, painter) = ui.allocate_painter(
                Vec2::splat(4. * self.block_size),
                egui::Sense::focusable_noninteractive(),
            );

            painter.extend(build_shapes(
                &state.next,
                self.block_size,
                response.rect.left_top(),
                Color32::GOLD,
                Color32::BLACK,
            ));
            response
        };
        let stats_panel_drawer = |ui: &mut egui::Ui| {
            ui.label("Next:");
            egui::Frame::canvas(ui.style()).show(ui, next_tetronimo_drawer);

            ui.separator();
            ui.label(format!("Score: {:?}", state.score));

            ui.separator();
            ui.image(&self.logo, Vec2::splat(ui.available_width()));

            ui.separator();
            if ui.button("Start").clicked() {
                self.start(ui.ctx().clone());
            };
            if ui.button("Reset").clicked() {
                self.game.lock().reset();
            };
        };

        egui::SidePanel::right("stats").show_inside(ui, |ui| {
            ui.set_width(4. * self.block_size);
            ui.vertical_centered(stats_panel_drawer);
        });
    }

    fn show_game(&self, ui: &mut egui::Ui, state: &DisplayState) {
        let game_scene_drawer = |ui: &mut egui::Ui| {
            let area = self.game.lock().area;
            let (response, painter) = ui.allocate_painter(
                Vec2::new(
                    area.max_x as f32 * self.block_size,
                    area.max_y as f32 * self.block_size,
                ),
                egui::Sense::focusable_noninteractive(),
            );

            painter.extend(build_scene_shapes(
                state,
                self.block_size,
                response.rect.left_top(),
            ));
            response
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::canvas(ui.style()))
            .show_inside(ui, game_scene_drawer);
    }
}
