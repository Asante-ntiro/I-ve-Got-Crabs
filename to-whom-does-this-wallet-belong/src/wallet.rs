use secp256k1::{PublicKey, SecretKey, rand::{rngs, SeedableRng}};
use anyhow::{Result, Ok};
use web3::{*, transports::Http};


pub fn create_keypair() -> Result<(SecretKey, PublicKey)> {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(6);
    Ok(secp.generate_keypair(&mut rng))
}

pub fn connect_to_web3(url: &str) -> Result<Web3<Http>> {
    let transport = web3::transports::Http::new(url)?;
    Ok(web3::Web3::new(transport))
}
