use macros::EnumFromDarling;
#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right(u32, u32),
}
#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}
fn main() {
    let up: Direction<i32> = DirectionUp::new(42).into();
    let left: Direction<i32> = 10.into();
    println!("{:?},{:?}", up, left);
}
impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}
