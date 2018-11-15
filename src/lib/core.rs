#[derive(Debug, PartialEq, Clone)]
pub enum LispCell {
    Atom(String),
    Quoted(Box<LispCell>),
    List { contents: Vec<LispCell> },
}

#[derive(Debug, PartialEq)]
pub struct LispProgram {
    pub text: String,
    pub entry: Option<Box<LispCell>>,
}
