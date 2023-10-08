mod utils;
mod tictactoe;
mod checkers;
mod base;
mod logs;
mod checkers_view;

use tictactoe::*;
use wasm_bindgen::prelude::*;
use std::fmt;
use std::convert::From;
use crate::base::IFields;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tic-tac-toe!");
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    X,
    O,
    NoValue,
}

impl From<Value> for Cell {
    fn from(item: Value) -> Self {
        match item {
            Value::X => Cell::X,
            Value::O => Cell::O,
            Value::NoValue => Cell::NoValue,            
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    NotStarted,
    MovePlayer1,
    MovePlayer2,
    Finished,
}

impl From<TicTacToeState> for GameState {
    fn from(item: TicTacToeState) -> Self {
        match item {
            TicTacToeState::NotStarted => GameState::NotStarted,
            TicTacToeState::MovePlayer1 => GameState::MovePlayer1,
            TicTacToeState::MovePlayer2 => GameState::MovePlayer2,
            TicTacToeState::Finished(_) => GameState::Finished,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WinnerLineState {
    None,
    Horizontally1,
    Horizontally2,
    Horizontally3,
    Vertically1,
    Vertically2,
    Vertically3,
    DiagonallyFromLeftToRight,
    DiagonallyFromoRightToLeft ,
}

impl From<WinnerLine> for WinnerLineState {
    fn from(item: WinnerLine) -> Self {
        match item {
            WinnerLine::None => WinnerLineState::None,
            WinnerLine::Horizontally1 => WinnerLineState::Horizontally1,
            WinnerLine::Horizontally2 => WinnerLineState::Horizontally2,
            WinnerLine::Horizontally3 => WinnerLineState::Horizontally3,
            WinnerLine::Vertically1 => WinnerLineState::Vertically1,
            WinnerLine::Vertically2 => WinnerLineState::Vertically2,
            WinnerLine::Vertically3 => WinnerLineState::Vertically3,
            WinnerLine::DiagonallyFromLeftToRight => WinnerLineState::DiagonallyFromLeftToRight,
            WinnerLine::DiagonallyFromoRightToLeft => WinnerLineState::DiagonallyFromoRightToLeft,
        }
    }
}


#[wasm_bindgen]
pub struct TicTacToeView {
    count: u32,
    width: u32,
    height: u32,
    tictactoe: TicTacToe<Fields>,
    cells: [Cell; 9],
}

#[wasm_bindgen]
impl TicTacToeView {
    pub fn new(width: u32, height: u32) -> Self {
        utils::set_panic_hook();
        let player1 = Player::new("test".to_string(), Value::X);
        let player2 = Player::new("test2".to_string(), Value::O);


        TicTacToeView{
            count: 0,
            width: width,
            height: height,
            tictactoe: TicTacToe::new(Fields::new(), player1, player2),
            cells: [Cell::NoValue; 9]
        }
    }

    pub fn join_player1(&mut self, name: &str) {
        let action: Action<<tictactoe::Fields as IFields>::MoveValue> = Action::JoinPlayer1(name.to_string());
        self.tictactoe.play(action);
    }

    pub fn join_player2(&mut self, name: &str) {
        let action: Action<<tictactoe::Fields as IFields>::MoveValue> = Action::JoinPlayer2(name.to_string());
        self.tictactoe.play(action);
    }

    pub fn start(&mut self) -> GameState {
        let action: Action<<tictactoe::Fields as IFields>::MoveValue> = Action::Start;
        let result: TicTacToeState = self.tictactoe.play(action);
        GameState::from(result)
    }

    pub fn move_player1(&mut self, x: u8, y: u8) -> GameState {
        let action: Action<<tictactoe::Fields as IFields>::MoveValue> = Action::MovePlayer1(Move{x, y});
        let result: TicTacToeState = self.tictactoe.play(action);
        GameState::from(result)
    }

    pub fn move_player2(&mut self, x: u8, y: u8) -> GameState {
        println!("move_player2");
        let action: Action<<tictactoe::Fields as IFields>::MoveValue> = Action::MovePlayer2(Move{x, y});
        let result = self.tictactoe.play(action);
        GameState::from(result)
    }

    pub fn get_winner(&self) -> String {
        let winner = self.tictactoe.get_winner();
        winner.unwrap_or("".to_string())
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_winner_line(&self) -> WinnerLineState {
        let fields = self.tictactoe.get_fields();
        WinnerLineState::from(fields.get_winner_line())
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cells(&mut self) -> *const Cell {
        let fields = self.tictactoe.get_fields();
        //web_sys::console::log_1(&format!("move_current_player {:?}", fields.fields[0]).into());
        for (i, value ) in fields.fields.iter().enumerate() {
            self.cells[i] =  Cell::from(value.clone());
        }
        
        self.cells.as_ptr()
    }
    
    pub fn tick(&mut self) {
        self.count += 1;
    }
}

impl fmt::Display for TicTacToeView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<span id=\"field1\">-<span> | - | -\n")?;
        write!(f, "---------\n")?;
        write!(f, "- | - | -\n")?;
        write!(f, "---------\n")?;
        write!(f, "- | - | -\n")?;

        Ok(())
    }
}
