#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gui::GameInterface;

mod geometry;
mod blocks;
mod tetris;
mod tetronimo;
mod utils;
mod gui;

fn main() {
    // define global options
    let options = eframe::NativeOptions {
        resizable: false,
        ..eframe::NativeOptions::default()
    };
    // Start the main window thread with the Game Interface
    eframe::run_native("Tetris", options, Box::new(|cc| Box::new(GameInterface::new(cc))));
}
