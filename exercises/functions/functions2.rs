// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)


fn main() {
    call_me(30);
}

fn call_me(num: u8) {
    for i in &[1,2,3,4] {
        println!("Ring! Call number {}", i + 1);
    }
}
