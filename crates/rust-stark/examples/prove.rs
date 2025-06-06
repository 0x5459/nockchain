use rust_stark::{generate_proof, verify_proof, Felt};
use hoonc::build_jam;
use nockapp::{Noun, NounExt};
use nockapp::utils::{create_context, NOCK_STACK_SIZE};
use nockvm::jets::hot::URBIT_HOT_STATE;
use nockvm::jets::cold::Cold;
use nockvm::mem::NockStack;
use nockvm::noun::Atom;
use ibig::UBig;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let trace: Vec<Felt> = (0u32..8).map(|v| Felt::new(v as u64)).collect();
    let proof = generate_proof(&trace);
    println!("Rust proof root: {:?}", proof.root);
    println!("proof verifies? {}", verify_proof(&trace, &proof));

    // Compile the Hoon verifier jam
    let jam = build_jam("verify-root.hoon", PathBuf::from("hoon"), None, false, true)
        .await
        .expect("compile hoon verifier");
    println!("compiled verifier jam -> {} bytes", jam.len());

    // Setup a minimal Nock interpreter context
    let mut stack = NockStack::new(NOCK_STACK_SIZE, 0);
    let cold = Cold::new(&mut stack);
    let mut ctx = create_context(stack, URBIT_HOT_STATE, cold, None);

    // Decode the jam and call it with the proof root
    let trap = Noun::cue_bytes_slice(&mut ctx.stack, &jam).expect("invalid jam");
    let root_big = UBig::from_le_bytes(&proof.root);
    let root_atom = Atom::from_ubig(&mut ctx.stack, &root_big).as_noun();
    let res = nockvm::jets::util::slam(&mut ctx, trap, root_atom).expect("run verifier");
    println!("Hoon verifier result: {:?}", res);
}
