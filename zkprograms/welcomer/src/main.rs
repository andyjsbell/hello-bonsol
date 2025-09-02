use risc0_zkvm::{
    guest::{env, sha::Impl},
    sha::{Digest, Sha256},
};

fn main() {
    let mut guess = Vec::new();
    env::read_slice(&mut guess);
    let digest = Impl::hash_bytes(&[guess.as_slice()].concat());
    env::commit_slice(digest.as_bytes());
}
