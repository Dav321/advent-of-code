#[allow(unused)]
pub trait Day {
    fn new(input: &str) -> Self;
    fn solve0(&self) -> i64;
    fn solve1(&self) -> i64;
}