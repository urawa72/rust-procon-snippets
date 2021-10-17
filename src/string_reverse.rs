use cargo_snippet::snippet;

#[snippet(include = "ExtendString")]
pub trait ExtendString {
    fn reverse(&self) -> String;
}

#[snippet("ExtendString")]
impl ExtendString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
