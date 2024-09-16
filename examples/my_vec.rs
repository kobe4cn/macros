fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
}

#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr;$n:expr) => {
        Vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        {

            <[_]>::into_vec(Box::new([$($x),+]))
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec
        }
    };
}

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}
