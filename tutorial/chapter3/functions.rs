#[allow(unused_variables)]

fn main() {

    let fun: u32 = another_function(5);
    println!("{}", fun);
}

fn another_function(i: u32) -> u32 {
    i + 32
}
