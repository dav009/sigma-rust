//! Wallet-like features
use derive_more::{From, Into};
use ergo_lib::chain::transaction::TxIoVec;
use wasm_bindgen::prelude::*;

pub mod derivation_path;
pub mod ext_pub_key;

use crate::{
    box_coll::ErgoBoxes, ergo_state_ctx::ErgoStateContext, error_conversion::to_js,
    secret_key::SecretKeys, transaction::reduced::ReducedTransaction, transaction::Transaction,
    transaction::UnsignedTransaction,
};

/// A collection of secret keys. This simplified signing by matching the secret keys to the correct inputs automatically.
#[wasm_bindgen]
#[derive(From, Into)]
pub struct Wallet(ergo_lib::wallet::Wallet);

#[wasm_bindgen]
impl Wallet {
    /// Create wallet instance loading secret key from mnemonic
    /// Returns None if a DlogSecretKey cannot be parsed from the provided phrase
    #[wasm_bindgen]
    pub fn from_mnemonic(mnemonic_phrase: &str, mnemonic_pass: &str) -> Option<Wallet> {
        Some(ergo_lib::wallet::Wallet::from_mnemonic(mnemonic_phrase, mnemonic_pass)?.into())
    }

    /// Create wallet using provided secret key
    #[wasm_bindgen]
    pub fn from_secrets(secret: &SecretKeys) -> Wallet {
        Wallet(ergo_lib::wallet::Wallet::from_secrets(secret.into()))
    }

    /// Sign a transaction:
    /// `tx` - transaction to sign
    /// `boxes_to_spend` - boxes corresponding to [`UnsignedTransaction::inputs`]
    /// `data_boxes` - boxes corresponding to [`UnsignedTransaction::data_inputs`]
    #[wasm_bindgen]
    pub fn sign_transaction(
        &self,
        _state_context: &ErgoStateContext,
        tx: &UnsignedTransaction,
        boxes_to_spend: &ErgoBoxes,
        data_boxes: &ErgoBoxes,
    ) -> Result<Transaction, JsValue> {
        let boxes_to_spend = TxIoVec::from_vec(boxes_to_spend.clone().into()).map_err(to_js)?;
        let data_boxes = {
            let d: Vec<_> = data_boxes.clone().into();
            if d.is_empty() {
                None
            } else {
                Some(TxIoVec::from_vec(d).map_err(to_js)?)
            }
        };
        let tx_context = ergo_lib::wallet::signing::TransactionContext::new(
            tx.clone().into(),
            boxes_to_spend,
            data_boxes,
        )
        .map_err(to_js)?;
        self.0
            .sign_transaction(tx_context, &_state_context.clone().into())
            .map_err(to_js)
            .map(Transaction::from)
    }

    /// Sign a transaction:
    /// `reduced_tx` - reduced transaction, i.e. unsigned transaction where for each unsigned input
    /// added a script reduction result.
    #[wasm_bindgen]
    pub fn sign_reduced_transaction(
        &self,
        reduced_tx: &ReducedTransaction,
    ) -> Result<Transaction, JsValue> {
        self.0
            .sign_reduced_transaction(reduced_tx.clone().into())
            .map_err(to_js)
            .map(Transaction::from)
    }
}
