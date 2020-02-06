use crate::{
    http_api::action::ListRequiredFields,
    swap_protocols::{
        ledger::{bitcoin, Ethereum},
        rfc003::{
            actions::Accept,
            messages::{self, IntoAcceptMessage},
            DeriveIdentities, Ledger,
        },
        SwapId,
    },
};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct OnlyRedeem<L: Ledger> {
    pub alpha_ledger_redeem_identity: L::Identity,
}

impl ListRequiredFields for Accept<Ethereum, bitcoin::Mainnet> {
    fn list_required_fields() -> Vec<siren::Field> {
        ethereum_bitcoin_accept_required_fields()
    }
}

impl ListRequiredFields for Accept<Ethereum, bitcoin::Testnet> {
    fn list_required_fields() -> Vec<siren::Field> {
        ethereum_bitcoin_accept_required_fields()
    }
}

impl ListRequiredFields for Accept<Ethereum, bitcoin::Regtest> {
    fn list_required_fields() -> Vec<siren::Field> {
        ethereum_bitcoin_accept_required_fields()
    }
}

fn ethereum_bitcoin_accept_required_fields() -> Vec<siren::Field> {
    vec![siren::Field {
        name: "alpha_ledger_redeem_identity".to_owned(),
        class: vec!["ethereum".to_owned(), "address".to_owned()],
        _type: Some("text".to_owned()),
        value: None,
        title: Some("Alpha ledger redeem identity".to_owned()),
    }]
}

impl IntoAcceptMessage<Ethereum, bitcoin::Regtest> for OnlyRedeem<Ethereum> {
    fn into_accept_message(
        self,
        id: SwapId,
        secret_source: &dyn DeriveIdentities,
    ) -> messages::Accept<Ethereum, bitcoin::Regtest> {
        let beta_ledger_refund_identity = crate::bitcoin::PublicKey::from_secret_key(
            &*crate::SECP,
            &secret_source.derive_refund_identity(),
        );
        messages::Accept {
            swap_id: id,
            alpha_ledger_redeem_identity: self.alpha_ledger_redeem_identity,
            beta_ledger_refund_identity,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct OnlyRefund<L: Ledger> {
    pub beta_ledger_refund_identity: L::Identity,
}

impl ListRequiredFields for Accept<bitcoin::Mainnet, Ethereum> {
    fn list_required_fields() -> Vec<siren::Field> {
        bitcoin_ethereum_accept_required_fields()
    }
}

impl ListRequiredFields for Accept<bitcoin::Testnet, Ethereum> {
    fn list_required_fields() -> Vec<siren::Field> {
        bitcoin_ethereum_accept_required_fields()
    }
}

impl ListRequiredFields for Accept<bitcoin::Regtest, Ethereum> {
    fn list_required_fields() -> Vec<siren::Field> {
        bitcoin_ethereum_accept_required_fields()
    }
}

fn bitcoin_ethereum_accept_required_fields() -> Vec<siren::Field> {
    vec![siren::Field {
        name: "beta_ledger_refund_identity".to_owned(),
        class: vec!["ethereum".to_owned(), "address".to_owned()],
        _type: Some("text".to_owned()),
        value: None,
        title: Some("Beta ledger refund identity".to_owned()),
    }]
}

impl IntoAcceptMessage<bitcoin::Regtest, Ethereum> for OnlyRefund<Ethereum> {
    fn into_accept_message(
        self,
        id: SwapId,
        secret_source: &dyn DeriveIdentities,
    ) -> messages::Accept<bitcoin::Regtest, Ethereum> {
        let alpha_ledger_redeem_identity = crate::bitcoin::PublicKey::from_secret_key(
            &*crate::SECP,
            &secret_source.derive_redeem_identity(),
        );
        messages::Accept {
            swap_id: id,
            beta_ledger_refund_identity: self.beta_ledger_refund_identity,
            alpha_ledger_redeem_identity,
        }
    }
}
