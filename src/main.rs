use std::time::{Duration, Instant};

use eframe::egui;
use rand::Rng;
use egui::Color32;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const BLOCK_SIZE: f32 = 23.0;

struct Tetromino {
    shape: Vec<Vec<bool>>,
    color: Color32,
    x: usize,
    y: usize,
}

struct TetrisGame {
    board: Vec<Vec<Color32>>,
    current_piece: Tetromino,
    game_over: bool,
    score: u32,
    last_update: Instant,
    update_interval: Duration,
}

impl TetrisGame {
    fn new() -> Self {
        let mut game = TetrisGame {
            board: vec![vec![Color32::TRANSPARENT; BOARD_WIDTH]; BOARD_HEIGHT],
            current_piece: Self::generate_piece(),
            game_over: false,
            score: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_secs_f32(0.75),
        };
        game.spawn_piece();
        game
    }

    fn generate_piece() -> Tetromino {
        let shapes = vec![
            (
                vec![
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                ],
                Color32::KHAKI
            ),
            (
                vec![vec![true, false, false], vec![true, true, true]],
                Color32::BLUE
            ),
            (
                vec![vec![false, false, true], vec![true, true, true]],
                Color32::GOLD
            ),
            (vec![vec![true, true], vec![true, true]], Color32::YELLOW),
            (
                vec![vec![false, true, true], vec![true, true, false]],
                Color32::GREEN
            ),
            (
                vec![vec![false, true, false], vec![true, true, true]],
                Color32::BROWN
            ),
            (
                vec![vec![true, true, false], vec![false, true, true]],
                Color32::RED,
            ),
        ];

        let (shape, color) = shapes[rand::thread_rng().gen_range(0..shapes.len())].clone();
        Tetromino {
            shape,
            color,
            x: BOARD_WIDTH / 2 - 1,
            y: 0,
        }
    }

    fn spawn_piece(&mut self) {
        if !self.game_over {
            self.current_piece = Self::generate_piece();
            if self.piece_collides() {
                self.game_over = true;
            }
        }
    }

    fn piece_collides(&self) -> bool {
        for (dy, row) in self.current_piece.shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = self.current_piece.x + dx;
                    let board_y = self.current_piece.y + dy;
                    if board_x >= BOARD_WIDTH
                        || board_y >= BOARD_HEIGHT
                        || self.board[board_y][board_x] != Color32::TRANSPARENT
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn move_piece(&mut self, dx: i32, dy: i32) {
        self.current_piece.x = (self.current_piece.x as i32 + dx).max(0) as usize;
        self.current_piece.y = (self.current_piece.y as i32 + dy).max(0) as usize;
        if self.piece_collides() {
            self.current_piece.x = (self.current_piece.x as i32 - dx).max(0) as usize;
            self.current_piece.y = (self.current_piece.y as i32 - dy).max(0) as usize;
            if dy > 0 {
                self.lock_piece();
            }
        }
    }

    fn rotate_piece(&mut self) {
        let old_shape = self.current_piece.shape.clone();
        let rows = self.current_piece.shape.len();
        let cols = self.current_piece.shape[0].len();
        self.current_piece.shape = vec![vec![false; rows]; cols];
        for (y, row) in old_shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                self.current_piece.shape[x][rows - 1 - y] = cell;
            }
        }
        if self.piece_collides() {
            self.current_piece.shape = old_shape;
        }
    }

    fn lock_piece(&mut self) {
        for (dy, row) in self.current_piece.shape.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                if cell {
                    let board_x = self.current_piece.x + dx;
                    let board_y = self.current_piece.y + dy;
                    self.board[board_y][board_x] = self.current_piece.color;
                }
            }
        }
        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        self.board.retain(|row| {
            let full = row.iter().all(|&cell| cell != Color32::TRANSPARENT);
            if full {
                lines_cleared += 1;
            }
            !full
        });
        for _ in 0..lines_cleared {
            self.board.insert(0, vec![Color32::TRANSPARENT; BOARD_WIDTH]);
        }
        self.score += lines_cleared * 100;
    }

    fn update(&mut self) {
        let now = Instant::now();
        if now - self.last_update >= self.update_interval {
            if !self.game_over {
                self.move_piece(0, 1);
                self.last_update = now;
            }
        }
    }
}

impl eframe::App for TetrisGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tetris Game in Rust");
            ui.label(format!("Score: {}", self.score));

            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(
                    BOARD_WIDTH as f32 * BLOCK_SIZE,
                    BOARD_HEIGHT as f32 * BLOCK_SIZE,
                ),
                egui::Sense::click_and_drag(),
            );

            // The grid
            for x in 0..=BOARD_WIDTH {
                painter.line_segment(
                    [
                        response.rect.min + egui::Vec2::new(x as f32 * BLOCK_SIZE, 0.0),
                        response.rect.min
                            + egui::Vec2::new(
                                x as f32 * BLOCK_SIZE,
                                BOARD_HEIGHT as f32 * BLOCK_SIZE,
                            ),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::GRAY),
                );
            }
            for y in 0..=BOARD_HEIGHT {
                painter.line_segment(
                    [
                        response.rect.min + egui::Vec2::new(0.0, y as f32 * BLOCK_SIZE),
                        response.rect.min
                            + egui::Vec2::new(
                                BOARD_WIDTH as f32 * BLOCK_SIZE,
                                y as f32 * BLOCK_SIZE,
                            ),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::GRAY),
                );
            }

            // The board
            for (y, row) in self.board.iter().enumerate() {
                for (x, &cell) in row.iter().enumerate() {
                    if cell != Color32::TRANSPARENT {
                        painter.rect_filled(
                            egui::Rect::from_min_size(
                                response.rect.min
                                    + egui::Vec2::new(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE),
                                egui::Vec2::splat(BLOCK_SIZE),
                            ),
                            0.0,
                            cell,
                            // match cell {
                            //     BlockColor::Cyan => egui::Color32::from_rgb(0, 255, 255),
                            //     BlockColor::Blue => egui::Color32::from_rgb(0, 0, 255),
                            //     BlockColor::Orange => egui::Color32::from_rgb(255, 165, 0),
                            //     BlockColor::Yellow => egui::Color32::from_rgb(255, 255, 0),
                            //     BlockColor::Green => egui::Color32::from_rgb(0, 255, 0),
                            //     BlockColor::Purple => egui::Color32::from_rgb(128, 0, 128),
                            //     BlockColor::Red => egui::Color32::from_rgb(255, 0, 0),
                            //     Color32::TRANSPARENT => unreachable!(),
                            // },
                        );
                    }
                }
            }

            for (dy, row) in self.current_piece.shape.iter().enumerate() {
                for (dx, &cell) in row.iter().enumerate() {
                    if cell {
                        painter.rect_filled(
                            egui::Rect::from_min_size(
                                response.rect.min
                                    + egui::Vec2::new(
                                        (self.current_piece.x + dx) as f32 * BLOCK_SIZE,
                                        (self.current_piece.y + dy) as f32 * BLOCK_SIZE,
                                    ),
                                egui::Vec2::splat(BLOCK_SIZE),
                            ),
                            0.0,
                            self.current_piece.color,
                            // match self.current_piece.color {
                            //     BlockColor::Cyan => egui::Color32::from_rgb(0, 255, 255),
                            //     BlockColor::Blue => egui::Color32::from_rgb(0, 0, 255),
                            //     BlockColor::Orange => egui::Color32::from_rgb(255, 165, 0),
                            //     BlockColor::Yellow => egui::Color32::from_rgb(255, 255, 0),
                            //     BlockColor::Green => egui::Color32::from_rgb(0, 255, 0),
                            //     BlockColor::Purple => egui::Color32::from_rgb(128, 0, 128),
                            //     BlockColor::Red => egui::Color32::from_rgb(255, 0, 0),
                            //     Color32::TRANSPARENT => unreachable!(),
                            // },
                        );
                    }
                }
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                self.move_piece(-1, 0);
            }

            if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                self.move_piece(1, 0);
            }

            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.move_piece(0, 1);
            }

            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.rotate_piece();
            }

            if self.game_over {
                ui.label("Game Over!");
                if ui.button("Restart").clicked() {
                    *self = TetrisGame::new();
                }
            }
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Tetris",
        native_options,
        Box::new(|_cc| Box::new(TetrisGame::new())),
    )
}
