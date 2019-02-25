mod socket_addr;
#[macro_use]
mod with_swap_types;
pub mod handlers;
pub mod routes;

mod htlc_location_impls {
    use crate::http_api::Http;
    use serde::{ser::Serialize, Serializer};

    impl Serialize for Http<bitcoin_support::OutPoint> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl Serialize for Http<ethereum_support::Address> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.serialize(serializer)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http_api::Http;
    use bitcoin_support::{OutPoint, Sha256dHash};
    use ethereum_support::{Address, H160};

    #[test]
    fn http_htlc_location_serializes_correctly_to_json() {
        let bitcoin_htlc_location = OutPoint {
            txid: Sha256dHash::from_hex(
                "ad067ee417ee5518122374307d1fa494c67e30c75d38c7061d944b59e56fe024",
            )
            .unwrap(),
            vout: 1u32,
        };
        let ethereum_htlc_location: Address = H160::from(7);

        let bitcoin_htlc_location = Http(bitcoin_htlc_location);
        let ethereum_htlc_location = Http(ethereum_htlc_location);

        let bitcoin_htlc_location_serialized =
            serde_json::to_string(&bitcoin_htlc_location).unwrap();
        let ethereum_htlc_location_serialized =
            serde_json::to_string(&ethereum_htlc_location).unwrap();

        assert_eq!(
            &bitcoin_htlc_location_serialized,
            r#"{"txid":"ad067ee417ee5518122374307d1fa494c67e30c75d38c7061d944b59e56fe024","vout":1}"#
        );
        assert_eq!(
            &ethereum_htlc_location_serialized,
            r#""0x0000000000000000000000000000000000000007""#
        );
    }
}
