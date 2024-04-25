fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.1;

    let six_point_four = x.0;

    let one = x.1;

    println!("{one}, {six_point_four}");


    let a = ["cat", "goat"];

    println!("{}", a[1])
}