// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call 2number {}", i + 1);
    }
}


fn bull(num: i32) -> i32{
    num * num
}