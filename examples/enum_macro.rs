use macros::EnumFrom;
#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right(u32, u32),
}
#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}
fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("{:?},{:?}", up, left);
}
impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}
