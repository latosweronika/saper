use crate::board::{Board, Cell, RevealResult};

use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
enum BoardSize {
    Small,
    Medium,
    Large,
}

impl BoardSize { 
    fn dimensions(&self) -> (usize, usize) {
        match self {
            BoardSize::Small => (8, 8),   
            BoardSize::Medium => (10, 10),
            BoardSize::Large => (14, 14),
        }
    }

    fn mines(self) -> usize {
            let (w, h) = self.dimensions();
            (w * h * 15 / 100) as usize 
    }

    fn label(self) -> &'static str {
        match self {
            BoardSize::Small => "8x8",
            BoardSize::Medium => "10x10",
            BoardSize::Large => "14x14",
        }
    }
    
}

pub struct MinesweeperApp {
    board: Board,
    game_over: bool,
    board_size: BoardSize,
}

impl MinesweeperApp {
    pub fn new() -> Self {
        let board_size = BoardSize::Small;
        let (width, height) = board_size.dimensions();
        let mines = board_size.mines();

        Self {
            board: Board::new(width, height, mines),
            game_over: false,
            board_size,
        }
    }
    fn restart(&mut self) {
        let (width, height) = self.board_size.dimensions();

        self.board = Board::new(
            width,
            height,
            self.board_size.mines(),
        );

        self.game_over = false;
    }
    
}

impl eframe::App for MinesweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    
        egui::CentralPanel::default()
            .show(ctx, |ui| {

                ui.horizontal(|ui|{
                    ui.label("Plansza:");
                    
                    ui.selectable_value( &mut self.board_size, BoardSize::Small, BoardSize::Small.label());
                    ui.selectable_value( &mut self.board_size, BoardSize::Medium, BoardSize::Medium.label());
                    ui.selectable_value(&mut self.board_size, BoardSize::Large, BoardSize::Large.label());
                    
                     if ui.button("Nowa gra").clicked() {
                        self.restart();
                    }
                });

                if self.game_over{
                    ui.heading("Koniec gry");
                    
                }
                for y in 0..self.board.height() {
                    ui.horizontal(|ui| {
                        
                        for x in 0..self.board.width() {
                            
                            let tile = self.board.tile(x, y);
                            let button_text = if tile.revealed() {
                                match tile.cell() {
                                    Cell::Mine => "*".to_string(),
                                    Cell::Empty(0) => " ".to_string(),
                                    Cell::Empty(n) => n.to_string(),
                                }
                            } else if tile.flagged() {
                                "F".to_string()
                            } else {
                                "■".to_string()
                            };
                            let response = ui.add( egui::Button::new(button_text).min_size(egui::vec2(40.0, 40.0)));

                            if response.clicked() && !self.game_over {
                                if let RevealResult::MineHit = self.board.reveal(x, y) {
                                    self.game_over = true;
                                    self.board.reveal_all(); 
                                }
                            }
                            if response.secondary_clicked() {
                                self.board.flag(x, y);
                            }
                        }
                    });
                }

                if self.board.check_win() && !self.game_over {
                    ui.heading("Wygrałeś!");
                }
                if ui.button("Restart").clicked(){
                    self.restart();
                }
            });

    }
}

