//! A tx for IBC.
//! This tx executes an IBC operation according to the given IBC message as the
//! tx_data. This tx uses an IBC message wrapped inside
//! `key::ed25519::SignedTxData` as its input as declared in `ibc` crate.

use std::cell::RefCell;
use std::rc::Rc;

use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, tx_data: Vec<u8>) -> TxResult {
    let signed = SignedTxData::try_from_slice(&tx_data[..])
        .wrap_err("failed to decode SignedTxData")?;
    let data = signed.data.ok_or_err_msg("Missing data")?;

    let ctx = Rc::new(RefCell::new(ctx.clone()));
    let mut actions = IbcActions::new(ctx.clone());
    let module = IbcTransferModule::new(ctx);
    actions.add_route(module.module_id(), module);

    actions.execute(&data).into_storage_result()
}
