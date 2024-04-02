//! A simple script to generate and verify the proof of a given program.

use wp1_core::{utils, SP1Prover, SP1Stdin, SP1Verifier};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 800;
    stdin.write(&n);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    let output = proof.stdout.read::<u32>();

    println!("[script] output: {:?}", output);

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the poseidon-precompiles program!");
}
