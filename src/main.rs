mod bit;
fn bit(byte: u8) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::new();
    for i in 0..8 {
        let bit_position: i32 = i % 8;
        bits.push((byte & (1 << bit_position)) != 0);
    }
    return bits;
}
fn bitbin(byte: u8) -> Vec<u8> {
    let rawbit: Vec<bool> = bit(byte);
    let mut bit: Vec<u8> = Vec::new();
    for i in 0..rawbit.len() {
        if rawbit[i] {
            bit.push(1);
        }
        else {
            bit.push(0);
        }
    }
    return bit;
}
fn encgt(inputtext: String, key: String) -> String {
    let mut nottext: String = String::new();
    let mut notkey: String = String::new();
    let mut rawxored: u8 = 0;
    let mut xorvec: Vec<u8> = Vec::new();
    let mut xored: String = String::new();
    let mut inputtex: Vec<u8> = Vec::new();
    let mut ke: Vec<u8> = Vec::new();
    let mut nottex: Vec<u8> = Vec::new();
    let mut notke: Vec<u8> = Vec::new();
    let mut len: usize = 0;
    for q in 0..inputtext.len() {
        inputtex = bitbin(inputtext.as_bytes()[q]);
    }
    for w in 0..key.len() {
        ke = bitbin(key.as_bytes()[w]);
    }
    for i in 0..inputtex.len() {
        nottext = bit::bit::bitnot(inputtex[i]).to_string();
    }
    for o in 0..ke.len() {
        notkey = bit::bit::bitnot(ke[o]).to_string();
    }
    for e in 0..nottext.len() {
        nottex = bitbin(nottext.as_bytes()[e]);
    }
    for r in 0..notke.len() {
        notke = bitbin(notkey.as_bytes()[r]);
    }
    len = nottext.len() + notkey.len();
    for p in 0..len {
        rawxored = bit::bit::bitxor(nottex[p], notke[p]);
        xorvec.push(rawxored);
    }
    xored = from_utf8(&xorvec).unwrap().to_string();
    return hex::encode(xored);
}
use std::str::from_utf8;

use eframe::egui;

fn main() {
    let native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    inputtext: String,
    key: String,
    encdec: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Jumbler Encryptor");
        ui.label("Text to encrypt or decrypt:");
        ui.text_edit_multiline(&mut self.inputtext);
        ui.label("Key:");
        ui.text_edit_singleline(&mut self.key);
        let mut endo = String::from(encgt(self.inputtext.clone(), self.key.clone()));
        let mut bytes = String::from("abc");
        if ui.button("Encrypt").clicked() {
         self.encdec = endo;
        }
        if ui.button("Decrypt").clicked() {
         self.encdec = bytes;
        }
        ui.label("Encrypted/decrypted text:");
        ui.text_edit_multiline(&mut self.encdec);
        if ui.button("Copy").clicked() {
         ui.output_mut(|o| o.copied_text = self.encdec.clone().to_string());
       }});
   }
}