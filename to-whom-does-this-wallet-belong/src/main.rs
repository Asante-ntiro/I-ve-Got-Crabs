use anyhow::Result;
mod wallet;

fn main() -> Result<()>{
    let keypair = wallet::create_keypair();
    println!("keypair: {:?}", keypair);

    Ok(())
}
