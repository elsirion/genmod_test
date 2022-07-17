#![feature(type_name_of_val)]

use api::genum::{Enum, EnumNil};
use api::gtry::GTryAny;
use api::module::OutPoint;
use api::Federation;
use hlist::Cons;
use mod_mint::Mint;
use mod_wallet::Wallet;

// TODO: This should be automated with a macro
type Fed = Federation<Cons<Wallet, Cons<Mint, hlist::Nil>>>;

#[tokio::main]
async fn main() {
    let fed = Federation::new().attach_module(Mint).attach_module(Wallet);

    // TODO: find a better way to construct enums (e.g. by having each type belong to a module, currently ambiguities prevent some solutions)
    let mint_out = Enum::<(), Enum<(), EnumNil>>::Next(Enum::<(), EnumNil>::Payload(()));
    let wallet_out = Enum::<(), Enum<(), EnumNil>>::Payload(());

    let out_point = OutPoint {
        txid: [0; 32],
        out_idx: 0,
    };

    let res = fed.process_output(mint_out, out_point).await;
    fed.process_output(wallet_out, out_point).await.unwrap();

    println!("res type:   {}", std::any::type_name_of_val(&res));

    let mint_in = Enum::Next(Enum::Payload(()));
    let err = fed.process_input(mint_in).await.to_any().err().unwrap();
    println!("err: {}", err);

    let wallet_in = Enum::Payload(());
    let err = fed.process_input(wallet_in).await.to_any().err().unwrap();
    println!("err: {}", err);
}
