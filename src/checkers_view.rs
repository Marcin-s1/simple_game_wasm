use wasm_bindgen::prelude::*;
use crate::base::IFields;
use crate::checkers;
use crate::checkers::*;
use crate::tictactoe::*;
use std::convert::From;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum CheckerCell {
    #[default] Empty,
    NotUsed,
    White,
    Black,
    WhiteKing,
    BlackKing
}

impl From<CheckerValue> for CheckerCell {
    fn from(item: CheckerValue) -> Self {
        match item {
            CheckerValue::Empty => CheckerCell::Empty,
            CheckerValue::NotUsed => CheckerCell::NotUsed,
            CheckerValue::White => CheckerCell::White,
            CheckerValue::Black => CheckerCell::Black,
            CheckerValue::WhiteKing => CheckerCell::WhiteKing,
            CheckerValue::BlackKing => CheckerCell::BlackKing
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState2 {
    NotStarted,
    MovePlayer1,
    MovePlayer2,
    Finished,
}

impl From<TicTacToeState> for GameState2 {
    fn from(item: TicTacToeState) -> Self {
        match item {
            TicTacToeState::NotStarted => GameState2::NotStarted,
            TicTacToeState::MovePlayer1 => GameState2::MovePlayer1,
            TicTacToeState::MovePlayer2 => GameState2::MovePlayer2,
            TicTacToeState::Finished(_) => GameState2::Finished,
        }
    }
}

#[wasm_bindgen]
pub struct CheckersView {
    count: u32,
    width: u32,
    height: u32,
    game: TicTacToe<FieldsCheckers>,
    cells: [CheckerCell; 64],
}

#[wasm_bindgen]
impl CheckersView {
    pub fn new(width: u32, height: u32) -> Self {
        let player1 = Player::new("test".to_string(), CheckerValue::White);
        let player2 = Player::new("test2".to_string(), CheckerValue::Black);
        
        CheckersView {
            count: 0,
            width: width,
            height: height,
            game: TicTacToe::new(FieldsCheckers::new(), player1, player2),
            cells: [CheckerCell::Empty; 64]
        }
    }

    pub fn join_player1(&mut self, name: &str) {
        let action: Action<<checkers::FieldsCheckers as IFields>::MoveValue> = Action::JoinPlayer1(name.to_string());
        self.game.play(action);
    }

    pub fn join_player2(&mut self, name: &str) {
        let action: Action<<checkers::FieldsCheckers as IFields>::MoveValue> = Action::JoinPlayer2(name.to_string());
        self.game.play(action);
    }

    pub fn start(&mut self) -> GameState2 {
        alert("Hello, tic-tac-toe!");
        let action: Action<<checkers::FieldsCheckers as IFields>::MoveValue> = Action::Start;
        let result = self.game.play(action);
        GameState2::from(result)
    }

    pub fn move_player1(&mut self, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> GameState2 {
        //alert(format!("move_player2 from: {} {}, to {} {}", from_x, from_y, to_x, to_y).as_str());
        let action: Action<<checkers::FieldsCheckers as IFields>::MoveValue> = Action::MovePlayer1(
            MoveCheckers{from: (from_x, from_y), to:  (to_x, to_y)}
        );
        let result: TicTacToeState = self.game.play(action);
        GameState2::from(result)
    }

    pub fn move_player2(&mut self, from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> GameState2 {
       // alert(format!("move_player2 from: {} {}, to {} {}", from_x, from_y, to_x, to_y).as_str());
        let action: Action<<checkers::FieldsCheckers as IFields>::MoveValue> = Action::MovePlayer2(
            MoveCheckers{from: (from_x, from_y), to:  (to_x, to_y)}
        );
        let result = self.game.play(action);
        GameState2::from(result)
    }


    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn cells(&mut self) -> *const CheckerCell {
        let fields = self.game.get_fields();

        for (i, value ) in fields.fields.iter().enumerate() {
            self.cells[i] =  CheckerCell::from(value.clone());
            //alert(format!("{:?}", self.cells[i]).as_str());
        }

        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        self.count += 1;
    }

}
