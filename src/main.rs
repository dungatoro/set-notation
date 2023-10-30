macro_rules! set {
    ( x: x < $y:tt ) => {
        InEq { upp: Some( Bound::Less($y) ), low: None, }
    };
    ( x: x <= $y:tt ) => {
        InEq { upp: Some( Bound::LessOrEq($y) ), low: None, }
    };
    ( x: x > $y:tt ) => {
        InEq { upp: None, low: Some( Bound::Greater($y) ), }
    };
    ( x: x >= $y:tt ) => {
        InEq { upp: None, low: Some( Bound::GreaterEq($y) ), }
    };
    ( x: $y:tt < x < $z:tt ) => {
        InEq { upp: Some( Bound::Less($z) ), low: Some( Bound::Greater($y) ), }
    };
    ( x: $y:tt <= x < $z:tt ) => {
        InEq { upp: Some( Bound::Less($z) ), low: Some( Bound::GreaterEq($y) ), }
    };
    ( x: $y:tt < x <= $z:tt ) => {
        InEq { upp: Some( Bound::LessOrEq($z) ), low: Some( Bound::Greater($y) ), }
    };
    ( x: $y:tt <= x <= $z:tt ) => {
        InEq { upp: Some( Bound::LessOrEq($z) ), low: Some( Bound::GreaterEq($y) ),
        }
    };
}

#[derive(Clone)]
enum Bound {
    Less(i32),
    LessOrEq(i32),
    Greater(i32),
    GreaterEq(i32),
}

struct InEq {
    upp: Option<Bound>,
    low: Option<Bound>,
}

impl std::fmt::Display for InEq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = String::from("{ x: ");
        match self.low.clone() {
            None => set += "x",
            Some(bound) => {
                match bound {
                    Bound::Greater(n) => set += &format!("{} < x", n),
                    Bound::GreaterEq(n) => set += &format!("{} <= x", n),
                    _ => set += "x",
                }
            }
        }
        match self.upp.clone() {
            None => set += "}", 
            Some(bound) => {
                match bound {
                    Bound::Less(n) => set += &format!(" < {} }}", n),
                    Bound::LessOrEq(n) => set += &format!(" <= {} }}", n),
                    _ => set += "x",
                }
            }
        }
        write!(f, "{}", set)
    }
}

fn main() {
    let set = set! { x: x > 4 };
    println!("{}", set);
}

