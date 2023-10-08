use crate::base::IFields;
extern crate web_sys;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum Value {
    X,
    O,
    #[default] NoValue,
}

#[derive(Copy, Clone, Default, Debug)]
pub enum WinnerLine {
    #[default] None,
    Horizontally1,
    Horizontally2,
    Horizontally3,
    Vertically1,
    Vertically2,
    Vertically3,
    DiagonallyFromLeftToRight,
    DiagonallyFromoRightToLeft ,

}

#[derive(Default, Clone, Debug)]
pub struct Fields {
    pub fields: [Value; 9],
    winner_line: WinnerLine,
}

impl IFields for Fields
{
    type MoveValue = Move;
    type Value = Value;

    fn reset(&mut self)
    {
        self.fields =  [Value::NoValue; 9];
        self.winner_line = WinnerLine::None;
    }
    
    fn set_value(&mut self, m: Move, value: Value) -> bool {
        let (x, y) = (m.x, m.y);
        web_sys::console::log_1(&format!("Fields::set_value {x} {y}").into());
        if x > 2 || y > 2 {
            return false;
        }
        web_sys::console::log_1(&format!("Fields::set_value2 {x} {y}").into());
        let pos = (x * 3 + y) as usize;
        if self.fields[pos] != Value::NoValue {
            return false;
        }
        web_sys::console::log_1(&format!("Fields::set_value {x} {y} {:?}", self.fields[pos]).into());

        self.fields[pos] = value;
        true
    }

    fn has_winner(&mut self) -> bool {
        let mut first: u8 = 0;
        let mut result = (0..self.fields.len() - 2).enumerate().step_by(3).any(|(index, _)| {
            first = index as u8;
            self.fields[index as usize] != Value::NoValue && 
            self.fields[index as usize] == self.fields[index+1 as usize] && 
            self.fields[index as usize] == self.fields[index+2 as usize]});
       
       if result {
            self.winner_line = match first {
                0 => WinnerLine::Horizontally1,
                3 => WinnerLine::Horizontally2,
                6 => WinnerLine::Horizontally3,
                _ => WinnerLine::None,
            };
            return result;
        }
        result = (0..self.fields.len() - 6).enumerate().any(|(index, _)| {
            first = index as u8;
            self.fields[index as usize] != Value::NoValue && self.fields[index as usize] == self.fields[index+3 as usize] && self.fields[index as usize] == self.fields[index+6 as usize]});
        if result {
            self.winner_line = match first {
                0 => WinnerLine::Vertically1,
                1 => WinnerLine::Vertically2,
                2 => WinnerLine::Vertically3,
                _ => WinnerLine::None,
            };
            
            return result;
        }
        result = self.fields[0] != Value::NoValue && self.fields[0] == self.fields[4] && self.fields[0] == self.fields[8];
        if result {
            self.winner_line = WinnerLine::DiagonallyFromLeftToRight;
            return result;
        }
        result = self.fields[2] != Value::NoValue && self.fields[2] == self.fields[4] && self.fields[2] == self.fields[6];
        if result {
            self.winner_line = WinnerLine::DiagonallyFromoRightToLeft;
        }


        return result;
    }

}

impl Fields {
    pub fn new() -> Self{
        Fields {
            fields: [Value::NoValue; 9],
            winner_line: WinnerLine::None,
        }
    }

    pub fn get_winner_line(&self) -> WinnerLine {
        self.winner_line.clone()
    }

}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Player<FieldParam: IFields + Default + Clone> {
    pub name: String,
    pub value: FieldParam::Value,
}

impl<FieldParam: IFields + Default + Clone> Player<FieldParam> {
    pub fn new (name: String, value: FieldParam::Value) -> Self {
        
        
        Player {
            name: name,
            value: value,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TicTacToeState {
    NotStarted,
    MovePlayer1,
    MovePlayer2,
    Finished(String),
}

#[derive(Default, Clone)]
pub struct Move{
    pub x: u8,
    pub y: u8,
}

pub enum Action<Move> {
    JoinPlayer1(String),
    JoinPlayer2(String),
    Start,
    MovePlayer1(Move),
    MovePlayer2(Move),
    Finished,
}

pub struct TicTacToe<FieldParam: IFields + Default + Clone> {
    player1: Option<Player<FieldParam>>,
    player2: Option<Player<FieldParam>>,
    fields: FieldParam,
    state: TicTacToeState,
}

pub trait IGame {
    type Action;

    fn play(&mut self, action: Self::Action) -> TicTacToeState; 
}

impl<FieldParam: IFields + Default + Clone> IGame for TicTacToe<FieldParam> {

    type Action = Action<<FieldParam as IFields>::MoveValue>;

    fn play(&mut self, action: Self::Action) -> TicTacToeState {
        match action {
            Action::JoinPlayer1(name) if self.state == TicTacToeState::NotStarted => {
                self.player1.as_mut().unwrap().name = name;
            },
            Action::JoinPlayer2(name) if self.state == TicTacToeState::NotStarted => {
                self.player2.as_mut().unwrap().name = name;
            },
            Action::Start if self.can_start() == true =>{
                self.fields.reset();
                self.state = TicTacToeState::MovePlayer1;
            },
            Action::MovePlayer1(m) if self.state == TicTacToeState::MovePlayer1 => {                
                let current_player = self.player1.as_ref().unwrap();
                self.move_current_player(&current_player.clone(), m);
             
            },
            Action::MovePlayer2(m) if self.state == TicTacToeState::MovePlayer2 => {
                let current_player = self.player2.as_ref().unwrap();
                self.move_current_player(&current_player.clone(), m);

            },
            Action::Finished => (),
            _ => (),
        }
        self.state.clone()
    }

}

impl<FieldParam: IFields + Default + Clone> TicTacToe<FieldParam> {
    pub fn new(field: FieldParam, p1: Player::<FieldParam>, p2: Player::<FieldParam>) -> Self {       
        TicTacToe {
            player1: Some(p1),
            player2: Some(p2),
            fields: field,
            state: TicTacToeState::NotStarted,
        }
    }

   pub fn get_player1(&self) -> Option<Player<FieldParam>> {
        self.player1.clone()
    }

    pub fn get_player2(&self) -> Option<Player<FieldParam>> {
        self.player2.clone()
    }

    pub fn get_state(&self) -> TicTacToeState {
        self.state.clone()
    }

    pub fn get_fields(&self) -> &FieldParam {
        &self.fields
    }

    pub fn get_winner(&self) -> Option<String> {
        match &self.state {
            TicTacToeState::Finished(winner) => Some(winner.clone()),
            _ => None
        }
    }

    fn can_start(&self) -> bool {
        match (&self.state, &self.player1, &self.player2) {
            (TicTacToeState::NotStarted, Some(_), Some(_)) => true,
            (TicTacToeState::Finished(_), Some(_), Some(_)) => true,
            _ => false,
        }
    }

    fn move_current_player(&mut self, current_player: &Player<FieldParam>, m: <FieldParam as IFields>::MoveValue)
    {
        web_sys::console::log_1(&"move_current_player".into());
        let is_moved = self.fields.set_value(m, current_player.value);
        if is_moved == false {
            return;
        }

        //web_sys::console::log_1(&format!("move_current_player {:?}", self.fields[0]).into());
        let is_winner = is_moved && self.fields.has_winner();
        if is_winner {
            self.state = TicTacToeState::Finished(current_player.name.clone());
            return;    
        }
        if self.state == TicTacToeState::MovePlayer1 {
            self.state =  TicTacToeState::MovePlayer2;
        }
        else {
            self.state =  TicTacToeState::MovePlayer1;
        }
    }

}


/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_test() {
        let mut tictactoe = TicTacToe::new();
        assert_eq!(tictactoe.get_state(), TicTacToeState::NotStarted);
        
        tictactoe.play(Action::JoinPlayer1("Test1".to_string()));
        tictactoe.play(Action::JoinPlayer2("Test2".to_string()));
        assert_eq!(tictactoe.get_state(), TicTacToeState::NotStarted);
        assert_eq!(tictactoe.get_player1(), Some(Player{name: "Test1".to_string()}));
        assert_eq!(tictactoe.get_player2(), Some(Player{name: "Test2".to_string()}));

        tictactoe.play(Action::Start);
        tictactoe.play(Action::MovePlayer1(0, 0));
        tictactoe.play(Action::MovePlayer2(2, 0));
        tictactoe.play(Action::MovePlayer1(0, 1));
        tictactoe.play(Action::MovePlayer2(2, 1));
        tictactoe.play(Action::MovePlayer1(0, 2));

        assert_eq!(tictactoe.get_state(), TicTacToeState::Finished);
        

    }
}

 */