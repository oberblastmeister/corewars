mod instruction;
mod parser;
mod process;
mod run_instruction;

use eframe::egui;
use egui::{Color32, Pos2, Rect, Ui};
use parser::parse_file;
use parser::parse_instruction;
use process::*;

impl Game {
    fn draw(&self, ui: &mut Ui) {
        let size = ui.available_size();

        let n = self.memory.len();
        let row_width = 100;

        let (tile_size, offset) = if size.x > size.y {
            (size.y / (row_width as f32), (size.x - size.y) / 2.)
        } else {
            (size.x / (row_width as f32), (size.y - size.x) / 2.)
        };

        for i in 0..n {
            let x = i % row_width;
            let y = i / row_width;
            let (xa, ya) = if size.x > size.y {
                (x as f32 * tile_size + offset, y as f32 * tile_size)
            } else {
                (x as f32 * tile_size, y as f32 * tile_size + offset)
            };

            let rect =
                Rect::from_two_pos(Pos2::new(xa, ya), Pos2::new(xa + tile_size, ya + tile_size));

            ui.painter().rect_filled(rect, 0.0, self.tile_color_at(i));
        }

        // for x in 0..n {
        //     for y in 0..n {
        //         let (xa, ya) = if size.x > size.y {
        //             (x as f32 * tile_size + offset, y as f32 * tile_size)
        //         } else {
        //             (x as f32 * tile_size, y as f32 * tile_size + offset)
        //         };

        //         let rect = Rect::from_two_pos(
        //             Pos2::new(xa, ya),
        //             Pos2::new(xa + tile_size, ya + tile_size),
        //         );

        //         ui.painter()
        //             .rect_filled(rect, 0.0, self.tile_color_at(x, y));
        //     }
        // }
    }

    pub fn tile_color_at(&self, i: usize) -> Color32 {
        match self.visualization[i] {
            None => Color32::from_rgb(236, 212, 179),
            Some(visualization) => Color32::from_rgb(53, 45, 45),
        }
        // // Color32::from_rgb(236, 212, 179)
        // if (x % 2 == 0 && y % 2 == 0) || (x % 2 == 1 && y % 2 == 1) {
        //     Color32::from_rgb(236, 212, 179)
        // } else {
        //     Color32::from_rgb(53, 45, 45)
        // }
        // } else {
        //     if self.player {
        //         Color32::from_rgb(53, 45, 45)
        //     } else {
        //         Color32::from_rgb(236, 212, 179)
        //     }
        // }
    }
}

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            let result = parse_file("erm.s");

            for r in result {
                println!("{}", r);
            }

            let c4 = vec![parse_instruction("mov 0 1").unwrap()];
            let c3 = vec![parse_instruction("mov 0 1").unwrap()];
            let c2 = vec![parse_instruction("mov 0 1").unwrap()];
            let c1 = vec![parse_instruction("mov 0 1").unwrap()];
            let c0 = vec![parse_instruction("mov 0 1").unwrap()];

            let mut game = Game::new(5, 100);

            game.start_game(vec![c0, c1, c2, c3, c4]);

            game.debug_print_memory();

            Ok(Box::new(game))
        }),
    );
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");

            self.draw(ui);
        });

        self.run_cycle();
    }
}
