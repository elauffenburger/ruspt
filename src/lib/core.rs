#[derive(Debug, PartialEq, Clone)]
pub enum LispCell {
    Atom(String),
    Number(f32),
    Str(String),
    Quoted(Box<LispCell>),
    List { contents: Vec<LispCell> },
}

#[derive(Debug, PartialEq)]
pub struct LispProgram {
    pub text: String,
    pub entry: Option<Box<LispCell>>,
}
