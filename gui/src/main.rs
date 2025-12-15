#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
// use egui::{TextStyle, TextWrapMode};
use egui_extras::{TableBuilder, Column};
use csv::ReaderBuilder;
use chrono::{ Utc};
use schedule::{Transmission, load_transmission_schedule};

pub fn ui_table(ui: &mut egui::Ui, transmissions: &Vec<Transmission>)
{
    // Create a scrollable table with 4 columns
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .header(20.0, |mut header| {
            header.col(|ui| { ui.label("Time (UTC)"); });
            header.col(|ui| { ui.label("WAX Station"); });
            header.col(|ui| { ui.label("Frequencies"); });
            header.col(|ui| { ui.label("Comments"); });
        })
        .body(|mut body| {
            for t in transmissions {
                body.row(20.0, |mut row_ui| {
                    row_ui.col(|ui| {
                        ui.label(format!("{}", t.time_of_day));
                    });
                    row_ui.col(|ui| {
                        ui.label(&t.station_name);
                    });
                    row_ui.col(|ui| {
                        ui.label(&t.frequencies);
                    });
                    row_ui.col(|ui| {
                        ui.label(&t.comment);
                    });
                });
            }
        });
}

fn main() -> eframe::Result<()> {

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // let data = load_csv("./schedules/schedule.csv");

    eframe::run_native(
        "WFAX Transmission Schedule",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    transmissions: Vec<Transmission>,       // Transmissions, imported from a CSV file
    current_utc_time: Utc,                  // Current time (UTC)
    current_index: i32,                     // Index of the table to highlight
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            transmissions: load_transmission_schedule("./schedules/schedule.csv"),
            current_utc_time: Utc,
            current_index: 0,
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui_table(ui, &self.transmissions);
        });
    }
}