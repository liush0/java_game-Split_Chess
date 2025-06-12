use wasm_bindgen::prelude::*;
use crate::{Board, GameState, Color, BOARD_SIZE};

// 导入 JS 函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 用于调试的宏
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Web 游戏包装器
#[wasm_bindgen]
pub struct WebGame {
    board: Board,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGame {
        unsafe{console_log!("创建新游戏");}
        WebGame {
            board: Board::new(),
        }
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, x: usize, y: usize) -> String {
        match self.board.make_move(x, y) {
            Ok(GameState::Playing) => "playing".to_string(),
            Ok(GameState::RedWins) => "red_wins".to_string(),
            Ok(GameState::BlackWins) => "black_wins".to_string(),
            Ok(GameState::Draw) => "draw".to_string(),
            Err(e) => format!("error:{}", e),
        }
    }

    #[wasm_bindgen]
    pub fn get_board_state(&self) -> String {
        let mut result = String::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let color = match self.board.camp[row][col] {
                    Color::Red => "R",
                    Color::Black => "B",
                    Color::Empty => "E",
                };
                result.push_str(&format!("{}{}|", color, self.board.num[row][col]));
            }
            result.push('\n');
        }
        result
    }

    #[wasm_bindgen]
    pub fn get_game_info(&self) -> String {
        format!("{}|{}|{}|{}", 
                self.board.rounds,
                self.board.redcount,
                self.board.blackcount,
                match self.board.currentplayer {
                    Color::Red => "red",
                    Color::Black => "black",
                    _ => "none"
                }
        )
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.board = Board::new();
        unsafe {console_log!("游戏重置");}
    }
}

impl Default for WebGame {
    fn default() -> Self {
        WebGame::new()
    }
}