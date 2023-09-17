use anyhow::Result;
mod wallet;


//const URL :&str = ""
fn main() -> Result<()>{
    let keypair = wallet::create_keypair();
    println!("keypair: {:?}", keypair);

//    let web3 = wallet::connect_to_web3(url)

    Ok(())
}
