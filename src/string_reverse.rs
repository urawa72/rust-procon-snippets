pub trait ExtendString {
    fn reverse(&self) -> String;
}

impl ExtendString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
