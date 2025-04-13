mod instruction;
mod parser;
mod process;
mod run_instruction;

use eframe::egui;
use egui::{Color32, Pos2, Rect, Ui};
use parser::parse_file;
use parser::parse_instruction;
use process::Game;

pub struct Board {
    // true = white, false = black
    player: bool,
}

impl Board {
    pub fn draw_board(&self, ui: &mut Ui) {
        let size = ui.available_size();

        let n = 100;

        let (tile_size, offset) = if size.x > size.y {
            (size.y / (n as f32), (size.x - size.y) / 2.)
        } else {
            (size.x / (n as f32), (size.y - size.x) / 2.)
        };

        for x in 0..n {
            for y in 0..n {
                let (xa, ya) = if size.x > size.y {
                    (x as f32 * tile_size + offset, y as f32 * tile_size)
                } else {
                    (x as f32 * tile_size, y as f32 * tile_size + offset)
                };

                let rect = Rect::from_two_pos(
                    Pos2::new(xa, ya),
                    Pos2::new(xa + tile_size, ya + tile_size),
                );

                ui.painter()
                    .rect_filled(rect, 0.0, self.tile_color_at(x, y));
            }
        }
    }

    pub fn tile_color_at(&self, x: usize, y: usize) -> Color32 {
        if (x % 2 == 0 && y % 2 == 0) || (x % 2 == 1 && y % 2 == 1) {
            if self.player {
                Color32::from_rgb(236, 212, 179)
            } else {
                Color32::from_rgb(53, 45, 45)
            }
        } else {
            if self.player {
                Color32::from_rgb(53, 45, 45)
            } else {
                Color32::from_rgb(236, 212, 179)
            }
        }
    }
}

fn main() {
    // -> eframe::Result {

    let result = parse_file("erm.s");

    for r in result {
        println!("{}", r);
    }

    let c1 = vec![parse_instruction("jmp 0 <-1").unwrap()];
    let c0 = vec![parse_instruction("mov 0 1").unwrap()];

    let mut g = Game::new(2, 100);

    g.start_game(vec![c0, c1]);

    g.debug_print_memory();

    //     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    //     let options = eframe::NativeOptions {
    //         viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    //         ..Default::default()
    //     };
    //     eframe::run_native(
    //         "My egui App",
    //         options,
    //         Box::new(|cc| {
    //             // This gives us image support:
    //             // egui_extras::install_image_loaders(&cc.egui_ctx);
    //
    //             Ok(Box::<MyApp>::default())
    //         }),
    //     )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            Board { player: true }.draw_board(ui);
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}
