pub struct InterpretedNum {
    pub value: f64,
}

pub struct InterpretedBool {
    pub value: bool,
}

pub struct InterpretedStr {
    pub value: String,
}

pub enum InterpretedParsed {
    IntepretedNum { value: f64 },
    InterpretedStr { value: String },
    InterpretedBool { value: bool },
}
