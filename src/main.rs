use my_qipan::{Board, GameState, Color, BOARD_SIZE};
use std::io::{self, Write};

fn display_board(board: &Board) {
    println!("      1    2    3    4");
    for row in 0..BOARD_SIZE {
        print!("{} | ", 4 - row);
        for col in 0..BOARD_SIZE {
            match board.camp[row][col] {
                Color::Red => print!("R({}) ", board.num[row][col]),
                Color::Black => print!("B({}) ", board.num[row][col]),
                Color::Empty => print!(".({}) ", board.num[row][col]),
            }
        }
        println!("|");
        if row < BOARD_SIZE - 1 {
            println!("  +---+---+---+---+");
        }
    }
    
    // 显示当前玩家
    print!("\n当前玩家: ");
    match board.currentplayer {
        Color::Red => println!("红方"),
        Color::Black => println!("黑方"),
        _ => {}
    }
    
    // 显示回合数和棋子数量
    println!("回合数: {}, 红方: {}, 黑方: {}", 
             board.rounds, board.redcount, board.blackcount);
}

fn get_user_input() -> Result<(usize, usize), String> {
    print!("请输入: ");
    io::stdout().flush().map_err(|_| "输出刷新失败")?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| format!("读取输入失败: {}", e))?;
    
    let coords: Vec<usize> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if coords.len() != 2 {
        return Err("请输入两个数字！".to_string());
    }
    
    Ok((coords[0], coords[1]))
}

fn main() {
    let mut gameboard = Board::new();
    display_board(&gameboard);
    
    println!("红方先行");
    println!("请按x y坐标系的形式放下棋子");
    
    loop {
        match get_user_input() {
            Ok((x, y)) => {
                match gameboard.make_move(x, y) {
                    Ok(GameState::Playing) => {
                        display_board(&gameboard);
                    },
                    Ok(GameState::RedWins) => {
                        display_board(&gameboard);
                        println!("红方胜利！");
                        break;
                    },
                    Ok(GameState::BlackWins) => {
                        display_board(&gameboard);
                        println!("黑方胜利！");
                        break;
                    },
                    Ok(GameState::Draw) => {
                        display_board(&gameboard);
                        println!("游戏平局！");
                        break;
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}