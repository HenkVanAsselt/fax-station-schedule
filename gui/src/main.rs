#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
#![allow(unused_imports)]

use eframe::egui;
use std::time::Duration;
// use egui::{TextStyle, TextWrapMode};
use chrono::Utc;
use egui::scroll_area::ScrollAreaOutput;
use egui::{Color32, Ui};
use egui_extras::{Column, Table, TableBuilder};
use schedule::{
    Transmission, get_next_transmission, get_next_transmission_index, load_transmission_schedule,
    print_next_transmission,
};

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
    current_index: usize,             // Index of the table to highlight
    // next_transmission: Option<Transmission>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            transmissions: load_transmission_schedule("./schedules/schedule.csv").unwrap_or_else(|err| {
                eprintln!("Error loading transmission schedule: {}", err);
                Vec::new()
            }),
            // current_utc_time: Utc,
            current_index: 0,
            // next_transmission: None,
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
                None => eprintln!("No upcoming transmissions found."),
            }

            let next_transmission = get_next_transmission(self.transmissions.clone());
            match next_transmission {
                Some(val) => print_next_transmission(val),
                None => eprintln!("No upcoming transmissions found."),
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
                        // let mut i = 0;
                        for (i, t) in self.transmissions.clone().iter().enumerate() {
                            let row_height = 20.0;
                            body.row(row_height, |mut row_ui| {
                                // Highlight the row with the next transmission 
                                if i == (self.current_index) {
                                    row_ui.set_selected(true);
                                }
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
