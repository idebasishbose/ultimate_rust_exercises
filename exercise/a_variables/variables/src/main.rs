const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready, new_var): (i32, i32, i32) = (STARTING_MISSILES, READY_AMOUNT, 1);
    println!("Firing {ready} of my {missiles} missiles...");
    println!("{} missiles left", (missiles - ready))
}
