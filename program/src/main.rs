//! A simple program to be proven inside the zkVM.

#![no_main]
wp1_zkvm::entrypoint!(main);

pub fn main() {
    let stdin = wp1_zkvm::io::read::<u32>();
    println!("[program] stdin: {:?}", stdin);

    let a = stdin;
    let a_right_shift_1 = a >> 1;
    let a_right_shift_2 = a >> 2;
    let a_right_shift_3 = a >> 3;

    let output = a ^ a_right_shift_1 ^ a_right_shift_2 ^ a_right_shift_3;

    wp1_zkvm::io::write::<u32>(&output);
}
