use anyhow::Result;
mod wallet;

use crate::wallet::{create_transaction, sign_and_send};
use fltk::{
    app,
    button::Button,
    enums::{CallbackTrigger, Color, Font, FrameType},
    frame::Frame,
    input,
    prelude::*,
    window::Window,
};
use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;
use tokio;
use web3;
use web3::types::Address;

const URL: &str = "https://eth-sepolia.g.alchemy.com/v2/GIYccgwMYvdmZBjpaYKMxrbeLAvIgBaw";

#[derive(Clone, Debug)]
pub enum WalletMessage {
    NewWallet,
    Send,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(500, 800)
        .with_label("Simple Wallet");
    let mut but = Button::new(195, 450, 120, 45, "Create Wallet");
    let mut but2 = Button::new(200, 300, 100, 35, " SEND ");
    let mut inp1 = input::Input::new(200, 200, 225, 35, "To: ");
    let mut frame = Frame::default()
        .with_size(600, 100)
        .center_of(&wind)
        .with_label("0 Wallets");

    frame.set_label_color(Color::White);
    frame.set_label_font(Font::TimesBold);
    frame.set_label_size(24);

    wind.set_color(Color::DarkCyan);

    but.set_color(Color::White);
    but.set_label_color(Color::DarkMagenta);
    but.set_label_font(Font::TimesBold);
    but.set_frame(FrameType::FlatBox);
    but.clear_visible_focus();

    but2.set_color(Color::White);
    but2.set_label_color(Color::DarkMagenta);
    but2.set_label_font(Font::TimesBold);
    but2.set_frame(FrameType::FlatBox);
    but2.clear_visible_focus();

    inp1.set_frame(FrameType::FlatBox);

    wind.end();
    wind.show();

    inp1.set_value("Please Enter Address Here");
    inp1.set_trigger(CallbackTrigger::Changed);

    let (s, r) = app::channel::<WalletMessage>();

    but.emit(s.clone(), WalletMessage::NewWallet);
    but2.emit(s, WalletMessage::Send);

    let web3 = wallet::connect_to_web3(URL)?;
    let mut keypairs: Vec<(PublicKey, SecretKey)> = Vec::new();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                WalletMessage::NewWallet => {
                    let (seckey, pubkey) = match wallet::create_keypair() {
                        Ok(val) => val,
                        Err(_e) => unimplemented!(),
                    };
                    keypairs.push((pubkey, seckey));
                    frame.set_label(&format!("{} Wallets", keypairs.len()));
                    println!("keypairs {:?}", keypairs);
                }
                WalletMessage::Send => {
                    let to_adrs = Address::from_str(&inp1.value().as_str())?;
                    let tx_object = create_transaction(to_adrs, 7)?;
                    let result = sign_and_send(web3.clone(), tx_object, keypairs[0].1).await;
                    match result {
                        Ok(val) => frame.set_label(&format!("{}", val)),
                        Err(e) => frame.set_label(&format!("{}", e)),
                    };
                }
            }
        }
    }
    app.run().unwrap();

    Ok(())
}
