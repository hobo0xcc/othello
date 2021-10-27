pub mod board;
pub mod moai;

use board::*;

use eframe::{egui, epi};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct OthelloApp {
    // #[cfg_attr(feature = "persistence", serde(skip))]
    board: board::Board,
}

impl Default for OthelloApp {
    fn default() -> Self {
        Self {
            board: Default::default(),
        }
    }
}

impl epi::App for OthelloApp {
    fn name(&self) -> &str {
        "othello"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>, _storage: Option<&dyn epi::Storage>) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        ctx.set_pixels_per_point(3.0);
        egui::CentralPanel::default().show(&ctx, |ui| {
            egui::Grid::new("Board").spacing(egui::vec2(0.0, 0.0)).show(ui, |ui| {
                let white_color = egui::Color32::from_rgb(255, 255, 255);
                let black_color = egui::Color32::from_rgb(0, 0, 0);

                while self.board.player == 1 && !self.board.is_game_ended() {
                    if self.board.legal_moves().len() == 0 {
                        self.board.player = self.board.next_player();
                        break;
                    }
                    self.play_ai();
                    if self.board.legal_moves().len() == 0 {
                        self.board.player = self.board.next_player();
                    }
                }

                let mut painters = Vec::new();
                let mut responses = Vec::new();
                let mut position = None;

                for y in 0..8 {
                    let mut column_response = Vec::new();
                    let mut column_painter = Vec::new();
                    for x in 0..8 {
                        let (response, painter) = ui.allocate_painter(egui::vec2(40.0, 40.0), egui::Sense::click_and_drag());
                        if response.clicked() {
                            position = Some((x, y));
                            // if self.board.is_legal_move(self.board.player_disk(), x, y) {
                            //     self.board = self.board.play(x, y);
                            // }
                        }
                        let rect = response.rect;
                        painter.rect(rect, 0.0, egui::Color32::from_rgb(0, 200, 0), egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 0, 0)));
                        column_response.push(response);
                        column_painter.push(painter);

                    }
                    responses.push(column_response);
                    painters.push(column_painter);
                    ui.end_row();
                }

                if let Some((x, y)) = position {
                    if self.board.is_legal_move(self.board.player_disk(), x, y) {
                        self.board = self.board.play(x, y);
                    }
                }

                for y in 0..8 {
                    for x in 0..8 {
                        let rect = responses[y][x].rect;
                        match self.board.disks[y][x] {
                            Disk::White => {
                                painters[y][x].circle_filled(rect.center(), 18.0, white_color);
                            }
                            Disk::Black => {
                                painters[y][x].circle_filled(rect.center(), 18.0, black_color);
                            }
                            _ => {}
                        }
                    }
                }

                if self.board.is_game_ended() {
                    return;
                }
            });

            let (black, white) = self.board.num_disk();
            ui.add(egui::Label::new(format!("Black: {}", black)).heading().monospace());
            ui.add(egui::Label::new(format!("White: {}", white)).heading().monospace());
            let resp = ui.add(egui::Button::new("Reset"));
            if resp.clicked() {
                self.board = Board::default();
            }
        });
    }
}

impl OthelloApp {
    pub fn play_ai(&mut self) {
        let board = moai::BitBoard::from_strings(self.board.to_strings(), self.board.player);
        let mut mcts = moai::MCTS::new(1.0, 1);
        let (position, count) = mcts.run(board, 1000);
        println!("{}", count);
        let (mut x, mut y) = (0, 0);
        for i in 0..64 {
            if position & (1 << i) != 0 {
                x = (63 - i) % 8;
                y = (63 - i) / 8;
            }
        }

        self.board = self.board.play(x, y);
    }
}