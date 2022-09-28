#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gui::MyApp;

mod geometry;
mod prototype;
mod stash;
mod tetris;
mod tetronimo;
mod utils;
mod gui;

fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native("Tetris", options, Box::new(|cc| Box::new(MyApp::new(cc))));
}
