mod presentation;
mod infrastructure;
mod domain;
mod application;

use oqs::*;

fn main() -> Result<()> {
    println!("Hello, world!");
    oqs::init();
    let sigalg = sig::Sig::new(sig::Algorithm::Dilithium2)?;
    let kemalg = kem::Kem::new(kem::Algorithm::Kyber512)?;

    let (pk, sk) = sigalg.keypair()?;
    println!("pk: {:?}", pk);
    let (kem_pk, kem_sk) = kemalg.keypair()?;

    let (a_sig_pk, a_sig_sk) = sigalg.keypair()?;

    let signature = sigalg.sign(kem_pk.as_ref(), &a_sig_sk)?;
    sigalg.verify(kem_pk.as_ref(), &signature, &a_sig_pk)?;
    
    Ok(())
}
