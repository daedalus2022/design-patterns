
  use wallet_facade::WalletFacade;


mod wallet_facade;
mod account;
mod wallet;
mod ledger;
mod notificaton;
mod security_code;

fn main() -> Result<(),String>{
    let mut wallet = WalletFacade::new("abc".into(), 1234);
    println!();

    wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;

    println!();

    wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5)

}
