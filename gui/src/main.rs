#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
// use egui::{TextStyle, TextWrapMode};
use egui_extras::{TableBuilder, Column};
use csv::ReaderBuilder;

fn load_csv(path: &str) -> Vec<Vec<String>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .expect("Cannot open CSV file");

    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result.expect("Invalid record");
        rows.push(record.iter().map(|s| s.to_string()).collect());
    }
    rows
}

pub fn ui_table(ui: &mut egui::Ui, data: &[Vec<String>]) {
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
            for row in data {
                body.row(20.0, |mut row_ui| {
                    for col in row.iter().take(4) {
                        row_ui.col(|ui| { ui.label(col); });
                    }
                });
            }
        });
}

fn main() -> eframe::Result<()> {
    let data = load_csv("./schedules/schedule.csv");

    eframe::run_native(
        "WFAX Transmission Schedule",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp { data }))),
    )
}

struct MyApp {
    data: Vec<Vec<String>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui_table(ui, &self.data);
        });
    }
}