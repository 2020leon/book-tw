fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("三"),
        _ => println!("anything"),
    }
    // ANCHOR_END: here
}
