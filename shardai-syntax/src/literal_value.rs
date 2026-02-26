#[derive(Clone, Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil
}