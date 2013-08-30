pub struct Indent {
    offset: uint
}

impl Indent {
    pub fn new() -> Indent {
        Indent {
            offset: 0
        }
    }
}

impl ToStr for Indent {
    fn to_str(&self) -> ~str {
        " ".repeat(self.offset)
    }
}
