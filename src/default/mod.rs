use std::fmt::Debug;
pub trait Format: Debug {
    fn left(&self, hold: usize) -> String;
    fn right(&self, hold: usize) -> String;
    fn mid(&self, hold: usize) -> String;
    fn left_lim(&self, hold: usize) -> String;
    fn right_lim(&self, hold: usize) -> String;
    fn mid_lim(&self, hold: usize) -> String;
    fn cut(&self, hold: usize) -> String;
}

pub fn hold_spaces(num: usize) -> String {
    let mut result = "".to_string();
    for _ in 0..num {
        result += " ";
    }
    result
}

impl Format for String {
    fn left(&self, hold: usize) -> String {
        if self.len() > hold {
            return format!("{}", self);
        }
        format!("{}{}", self, hold_spaces(hold-self.len()))
    }

    fn right(&self, hold: usize) -> String {
        if self.len() > hold {
            return format!("{}", self);
        }
        format!("{}{}", hold_spaces(hold-self.len()), self)
    }

    fn mid(&self, hold: usize) -> String {
        let temp = hold-self.len();
        let half = temp/2;
        format!("{}{}{}", hold_spaces(temp - half), self, hold_spaces(half))
    }

    fn left_lim(&self, hold: usize) -> String {
        if self.len() > hold {
            return self.cut(hold-3)+"...";           
        }
        self.left(hold)
    }

    fn right_lim(&self, hold: usize) -> String {
        if self.len() > hold {
            return self.cut(hold-3)+"...";            
        }
        self.right(hold)
    }

    fn mid_lim(&self, hold: usize) -> String {
        if self.len() > hold {
            return self.cut(hold-3)+"...";         
        }
        self.mid(hold)
    }

    fn cut(&self, hold: usize) -> String {
        if self.len() <= hold {
            return format!("{}", self);
        }
        let mut c = self.chars();
        let mut result = "".to_string();
        for _ in 0..hold {
            if let Some(x) = c.next() {
                result.push(x);
            }
        }
        result
    }
}
