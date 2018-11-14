#[derive(Debug, PartialEq)]
pub enum LispCell {
    Atom(String),
    List { contents: Vec<LispCell> },
    Func { operator: Box<LispCell>, operands: Vec<LispCell> },
}

#[derive(Debug, PartialEq)]
pub struct LispProgram {
    pub text: String,
    pub entry: Option<Box<LispCell>>,
}