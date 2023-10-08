use crate::base::IFields;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum CheckerValue {
    #[default] Empty,
    NotUsed,
    White,
    Black,
    WhiteKing,
    BlackKing,
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct MoveCheckers {
    pub from: (u8, u8),
    pub to: (u8, u8)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FieldsCheckers {
    pub fields: [CheckerValue; 64],
}

impl Default for FieldsCheckers {
    fn default() -> Self {
        Self {
            fields: [CheckerValue::Empty; 64],
        }
    }
}

impl IFields for FieldsCheckers
{
    type MoveValue = MoveCheckers;
    type Value = CheckerValue;

    fn set_value(&mut self, m: Self::MoveValue, value: Self::Value) -> bool
    {
        //web_sys::console::log_1(&format!("{:?}",value).into());
        let (x, y) = m.to;
        if self.fields[(x * 8 + y) as usize] != CheckerValue::Empty {
            return false;
        }

        let mut is_beat = false;
        let current_value = self.fields[(m.from.0 * 8 + m.from.1) as usize];
        //web_sys::console::log_1(&format!("Current value {:?}" ,current_value).into());


        if current_value == CheckerValue::White {
            let from = m.from.0 * 8 + m.from.1;
            let to = m.to.0 * 8 + m.to.1;
            //web_sys::console::log_1(&format!("{:?}",from).into());
            //web_sys::console::log_1(&format!("{:?}",to).into());
            is_beat = (from - 18 == to && self.is_black(from - 9)) ||
                      (from - 14 == to && self.is_black(from - 7));

           // web_sys::console::log_1(&format!("is_beat {:?} , form: {}, from-7: {}, value-7: {:?} {}",is_beat, from, from - 7, self.fields[(from - 7) as usize], from - 14 != to).into());
           //web_sys::console::log_1(&format!("CheckerValue::White is_beat {:?}" ,is_beat).into());
           let possible_move = from - 7 != to && from - 9 != to && !is_beat;
                      
            
            
            if possible_move {
                return false;
            }
        }

        if current_value == CheckerValue::WhiteKing {
            let from = m.from.0 * 8 + m.from.1;
            let to = m.to.0 * 8 + m.to.1;
            is_beat = (from - 18 == to && self.is_black(from - 9)) ||
                      (from - 14 == to && self.is_black(from - 7)) ||
                      (from + 18 == to && self.is_black(from + 9)) ||
                      (from + 14 == to && self.is_black(from + 7));

            let possible_move = (from - 7 == to || from - 9 == to) || (from + 7 == to || from + 9 == to) || is_beat;


            //web_sys::console::log_1(&format!("CheckerValue::WhiteKing is_beat {:?}" ,is_beat).into());
            if possible_move == false {
                //web_sys::console::log_1(&format!("CheckerValue::possible_move {:?}" ,possible_move).into());
                return false;
            }
        }

        if current_value == CheckerValue::BlackKing {
            let from = m.from.0 * 8 + m.from.1;
            let to = m.to.0 * 8 + m.to.1;
            is_beat = (from - 18 == to && self.is_white(from - 9)) ||
                      (from - 14 == to && self.is_white(from - 7)) ||
                      (from + 18 == to && self.is_white(from + 9)) ||
                      (from + 14 == to && self.is_white(from + 7));
            let possible_move = (from - 7 == to || from - 9 == to) || (from + 7 == to || from + 9 == to) || is_beat;
            //web_sys::console::log_1(&format!("CheckerValue::BlackKing is_beat {:?}" ,is_beat).into());
            if possible_move == false {
                //web_sys::console::log_1(&format!("CheckerValue::possible_move {:?}" ,possible_move).into());
                return false;
            }
        }

        if current_value == CheckerValue::Black {
            let from = m.from.0 * 8 + m.from.1;
            let to = m.to.0 * 8 + m.to.1;
            
            
            is_beat = (from + 18 == to && self.is_white(from + 9)) ||
                      (from + 14 == to && self.is_white(from + 7));

            //web_sys::console::log_1(&format!("CheckerValue::White is_beat {:?} , form: {}, from+7: {}, value+7: {:?} {}",is_beat, from, from + 7, self.fields[(from + 7) as usize], from + 14 != to).into());

            let possible_move = from + 7 != to && from + 9 != to && !is_beat;
            
            if possible_move {
                //web_sys::console::log_1(&format!("possible_move {:?} return false" ,possible_move).into());
                return false;
            }
        }

        if self.is_beat_possible(value) && is_beat == false {
            //web_sys::console::log_1(&format!("is_beat {:?}, is_beat_possible(value) {:?}, return false" ,is_beat, self.is_beat_possible(value)).into());
            return false;
        }

        self.fields[(x * 8 + y) as usize] = current_value;// self.fields[(m.from.0 * 8 + m.from.1) as usize];
        if x == 0 &&  value == CheckerValue::White {
            self.fields[(x * 8 + y) as usize] = CheckerValue::WhiteKing;
            //web_sys::console::log_1(&format!("WhiteKing").into());
        } else if x == 7 && value == CheckerValue::Black {
            self.fields[(x * 8 + y) as usize] = CheckerValue::BlackKing;
            //web_sys::console::log_1(&format!("BlackKing").into());
        }

        
        let (x, y) = m.from;
        self.fields[(x * 8  + y) as usize] = CheckerValue::Empty;

        if is_beat {
            let from = m.from.0 * 8 + m.from.1;
            let to = m.to.0 * 8 + m.to.1;
            if current_value == CheckerValue::White {
                if from - 18 == to {
                    self.fields[(from - 9) as usize] = CheckerValue::Empty;
                }
                else {
                    self.fields[(from - 7) as usize] = CheckerValue::Empty;
                }
            }
            else if current_value == CheckerValue::WhiteKing {
                //web_sys::console::log_1(&format!("CheckerValue::White, form: {}, to: {}", from, to).into()); 
                if from - 18 == to {
                    
                    self.fields[(from - 9) as usize] = CheckerValue::Empty;
                }
                else if from - 14 == to {
                    self.fields[(from - 7) as usize] = CheckerValue::Empty;
                } else if from + 18 == to {
                    self.fields[(from + 9) as usize] = CheckerValue::Empty;
                }
                else {
                    self.fields[(from + 7) as usize] = CheckerValue::Empty;
                }
            }
            else if current_value == CheckerValue::BlackKing {
                //web_sys::console::log_1(&format!("CheckerValue::White, form: {}, to: {}", from, to).into()); 
                if from - 18 == to {
                    
                    self.fields[(from - 9) as usize] = CheckerValue::Empty;
                }
                else if from - 14 == to {
                    self.fields[(from - 7) as usize] = CheckerValue::Empty;
                } else if from + 18 == to {
                    self.fields[(from + 9) as usize] = CheckerValue::Empty;
                }
                else {
                    self.fields[(from + 7) as usize] = CheckerValue::Empty;
                }
            }
            else {
                if from + 18 == to {
                    self.fields[(from + 9) as usize] = CheckerValue::Empty;
                }
                else {
                    self.fields[(from + 7) as usize] = CheckerValue::Empty;
                }
            }

            if self.is_beat_possible_for_checker(current_value, to as usize) {
                return false;
            }
        }

        true
    }

    fn has_winner(&mut self) -> bool {
        return false;
    }

    fn reset(&mut self)
    {

    }

}

impl FieldsCheckers {
    

    pub fn new() -> Self {
        let mut fields = [CheckerValue::Empty; 64];

        let mut offset = 1;
        for (i, value) in fields.iter_mut().enumerate() {           
            if i < 24
            {
                let result = if (i / 8) % 2 == 1 {1} else {0};
                if (i + offset) % 2 == result
                {
                    *value = CheckerValue::Black;
                }
                else {
                    *value = CheckerValue::NotUsed;
                }
            }
            else if i > 39
            {
                let result = if (i / 8) % 2 == 1 {1} else {0};
                if (i + offset) % 2 == result
                {
                    *value = CheckerValue::White;
                }
                else {
                    *value = CheckerValue::NotUsed;
                }
            }
            else {
                let result = if (i / 8) % 2 == 1 {1} else {0};
                if (i + offset) % 2 == result
                {
                    *value = CheckerValue::Empty;
                }
                else {
                    *value = CheckerValue::NotUsed;
                }
            }
        }
                      
        //web_sys::console::log_1(format!("{:?}",fields).as_str());
        FieldsCheckers {
            fields: fields
        }
        
    }

    fn is_black( &self, index: u8) -> bool {
        return self.fields[index as usize] == CheckerValue::Black || self.fields[index as usize] == CheckerValue::BlackKing;
    }

    fn is_white( &self, index: u8) -> bool {
        return self.fields[index as usize] == CheckerValue::White || self.fields[index as usize] == CheckerValue::WhiteKing;
    }

    fn is_beat_possible_for_checker(&mut self, value: CheckerValue, pos: usize) -> bool {
        web_sys::console::log_1(&format!("is_beat_possible_for_checker {:?}",pos ).into());

        if value == CheckerValue::BlackKing
        {
            if pos >= 18 {
                if  self.fields[pos - 18] == CheckerValue::Empty && self.is_white((pos - 9) as u8){
                        return true;
                }
            }

            if pos >= 14 {
                if  self.fields[pos - 14] == CheckerValue::Empty && self.is_white((pos - 7) as u8) {
                        return true;
                }
            }

            if pos < 64-18 {
                if  self.fields[pos + 18] == CheckerValue::Empty && self.is_white((pos + 9) as u8) {
                    return true;
                }
            }

            if pos < 64 - 14 {
                if  self.fields[pos + 14] == CheckerValue::Empty && self.is_white((pos + 7) as u8) {
                        return true;
                }
            }
        }


        if value == CheckerValue::WhiteKing
        {
            if pos >= 18 {
                if  self.fields[pos - 18] == CheckerValue::Empty && self.is_black((pos - 9) as u8){
                        return true;
                }
            }

            if pos >= 14 {
                if  self.fields[pos - 14] == CheckerValue::Empty && self.is_black((pos - 7) as u8) {
                        return true;
                }
            }

            if pos < 64-18 {
                if  self.fields[pos + 18] == CheckerValue::Empty && self.is_black((pos + 9) as u8) {
                    return true;
                }
            }

            if pos < 64 - 14 {
                if  self.fields[pos + 14] == CheckerValue::Empty && self.is_black((pos + 7) as u8) {
                        return true;
                }
            }
        }

        
        if value == CheckerValue::White
        {
            

            if pos >= 18 {
                if  self.fields[pos - 18] == CheckerValue::Empty && self.is_black((pos - 9) as u8){
                        return true;
                }
            }

            if pos >= 14 {
                if  self.fields[pos - 14] == CheckerValue::Empty && self.is_black((pos - 7) as u8) {
                        return true;
                }
            }
                
        }
        else if value == CheckerValue::Black
        {
            if pos < 64-18 {
                if  self.fields[pos + 18] == CheckerValue::Empty && self.is_white((pos + 9) as u8) {
                    return true;
                }
            }

            if pos < 64 - 14 {
                if  self.fields[pos + 14] == CheckerValue::Empty && self.is_white((pos + 7) as u8) {
                        return true;
                }
            }
        }
        false
    }

    fn is_beat_possible(&mut self, value: CheckerValue) -> bool {
        if value == CheckerValue::White
        {
            for i in 15..64 {
                if self.fields[i as usize] == CheckerValue::White {
                if i >= 18 {
                    if  self.fields[i as usize - 18] == CheckerValue::Empty && self.is_black(i  - 9) {
                            return true;
                    }
                }
                if  self.fields[i as usize - 14] == CheckerValue::Empty && self.is_black(i - 7) {
                        return true;
                }
                }
            }
        }
        else if value == CheckerValue::Black
        {
            for i in 0..49 {
                if self.fields[i as usize] == CheckerValue::Black {
                    if i <= 64-18 {
                        if  self.fields[i as usize + 18] == CheckerValue::Empty && self.is_white(i + 9) {
                                return true;
                        }
                    }
                    if  self.fields[i as usize + 14] == CheckerValue::Empty && self.is_white(i + 7) {
                            return true;
                    }
                    }
            }
        }

        false
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_initilized_correctly() {
        let checker = FieldsCheckers::new();
        let fields = &checker.fields;

        assert_eq!(fields[1], CheckerValue::Black);
        assert_eq!(fields[3], CheckerValue::Black);
        assert_eq!(fields[5], CheckerValue::Black);
        assert_eq!(fields[7], CheckerValue::Black);   
        
        assert_eq!(fields[8], CheckerValue::Black);
        assert_eq!(fields[10], CheckerValue::Black);
        assert_eq!(fields[12], CheckerValue::Black);
        assert_eq!(fields[14], CheckerValue::Black);

        assert_eq!(fields[17], CheckerValue::Black);
        assert_eq!(fields[19], CheckerValue::Black);
        assert_eq!(fields[21], CheckerValue::Black);
        assert_eq!(fields[23], CheckerValue::Black);

        assert_eq!(fields[40], CheckerValue::White);
        assert_eq!(fields[42], CheckerValue::White);
        assert_eq!(fields[44], CheckerValue::White);
        assert_eq!(fields[46], CheckerValue::White);

        assert_eq!(fields[49], CheckerValue::White);
        assert_eq!(fields[51], CheckerValue::White);
        assert_eq!(fields[53], CheckerValue::White);
        assert_eq!(fields[55], CheckerValue::White);

        assert_eq!(fields[56], CheckerValue::White);
        assert_eq!(fields[58], CheckerValue::White);
        assert_eq!(fields[60], CheckerValue::White);
        assert_eq!(fields[62], CheckerValue::White);

    }

    #[test]
    fn move_draught() {
        let mut checker = FieldsCheckers::new();
        //let fields = checker.fields;

        let m = MoveCheckers {
            from: (0, 5),
            to: (1, 4),
        };
        assert_eq!(checker.set_value(m, CheckerValue::White), true);


    }

}


