#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
#![allow(unused_imports)]

use eframe::egui;
use std::time::Duration;
// use egui::{TextStyle, TextWrapMode};
use chrono::Utc;
use egui::scroll_area::ScrollAreaOutput;
use egui_extras::{Column, Table, TableBuilder};
use schedule::{
    Transmission, get_next_transmission, get_next_transmission_index, load_transmission_schedule,
    print_next_transmission,
};

// pub fn ui_table(ui: &mut egui::Ui, transmissions: &Vec<Transmission>) -> ScrollAreaOutput<()>

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    eframe::run_native(
        "WFAX Transmission Schedule",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    transmissions: Vec<Transmission>, // Transmissions, imported from a CSV file
    current_utc_time: Utc,            // Current time (UTC)
    current_index: usize,             // Index of the table to highlight
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            transmissions: match load_transmission_schedule("./schedules/schedule.csv") {
                Ok(transmissions) => transmissions,
                Err(err) => {
                    eprintln!("Error loading transmission schedule: {}", err);
                    Vec::new()
                }
            },
            current_utc_time: Utc,
            current_index: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let index = get_next_transmission_index(self.transmissions.clone());
            match index {
                Some(val) => {
                    println!("Next transmission index: {:?}", val);
                    self.current_index = val;
                }
                None => println!("No upcoming transmissions found."),
            }

            let next_transmission = get_next_transmission(self.transmissions.clone());
            match next_transmission {
                Some(val) => print_next_transmission(val, false),
                None => println!("No upcoming transmissions found."),
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                let _table = TableBuilder::new(ui)
                    .scroll_to_row(self.current_index, None)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label("Time (UTC)");
                        });
                        header.col(|ui| {
                            ui.label("WAX Station");
                        });
                        header.col(|ui| {
                            ui.label("Frequencies");
                        });
                        header.col(|ui| {
                            ui.label("Comments");
                        });
                    })
                    .body(|mut body| {
                        for t in &self.transmissions.clone() {
                            body.row(20.0, |mut row_ui| {
                                row_ui.col(|ui| {
                                    ui.label(format!("{}", t.transmission_time));
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

            });
        });

        ctx.request_repaint_after(Duration::from_secs(10));
    }
}
