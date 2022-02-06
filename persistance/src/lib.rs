
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Test<'a> {
    Ass(&'a str),
    Jake(i32, f64),
    Queen(Box<Test<'a>>, Box<Test<'a>>),
    King(bool),
}

#[cfg(test)]
mod tests {
    use crate::Test;

    #[test]
    fn it_works() {
        let test = Test::Queen(
            Box::new(Test::Jake(1, 2.0)),
            Box::new(Test::King(false))
        );
        let s = serde_yaml::to_string(&test).expect("panic");
        assert_eq!(s, "---\nQueen:\n  - Jake:\n      - 1\n      - 2.0\n  - King: false\n");
    }

}
