#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;

use eframe::egui::{self, TextBuffer};
use reqwest::header::{HeaderMap, HeaderName};

fn main() {
    tracing_subscriber::fmt::init();
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native("Request spammer", options, Box::new(|_cc| {Box::new(App::default())}));
}

#[derive(PartialEq, Eq, Debug)]
enum Methods {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
    Trace,
}

struct App {
    headers: HeaderMap,
    body: String,
    spamming: bool,
    method: Methods,
    hname: String,
    hvalue: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            headers: HeaderMap::new(),
            body: "".to_string(),
            spamming: false,
            method: Methods::Get,
            hname: "".to_string(),
            hvalue: "".to_string(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FreeShit request spammer");
            ui.vertical(|ui| {
                if ui.button("SPAM").clicked() {
                    //
                }
                egui::ComboBox::from_label("Method")
                .selected_text(format!("{:?}", self.method))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.method, Methods::Get, "Get");
                    ui.selectable_value(&mut self.method, Methods::Post, "Post");
                    ui.selectable_value(&mut self.method, Methods::Put, "Put");
                    ui.selectable_value(&mut self.method, Methods::Delete, "Delete");
                    ui.selectable_value(&mut self.method, Methods::Head, "Head");
                    ui.selectable_value(&mut self.method, Methods::Options, "Options");
                    ui.selectable_value(&mut self.method, Methods::Connect, "Connect");
                    ui.selectable_value(&mut self.method, Methods::Patch, "Patch");
                    ui.selectable_value(&mut self.method, Methods::Trace, "Trace");
                });
                ui.label("Body: ");
                ui.text_edit_multiline(&mut self.body);
                ui.label("Header Name");
                ui.text_edit_singleline(&mut self.hname);
                ui.label("Header Value");
                ui.text_edit_singleline(&mut self.hvalue);
                if ui.button("Add Header").clicked() {
                    self.headers.append(HeaderName::from_bytes(self.hname.as_bytes()).unwrap(), self.hvalue.parse().unwrap());
                    self.hname = "".to_string();
                    self.hvalue = "".to_string();
                }
                ui.label("Headers: ");
                let heads = self.headers.clone();
                for (header_name, header_value) in heads.iter() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}: {}", header_name, header_value.to_str().unwrap().to_string()));
                        if ui.button("Del").clicked() {
                            self.headers.remove(header_name);
                        }
                    });
                }
            });
        });
    }
}
