pub mod collection;
pub mod datalake;
pub mod output;
pub mod rlp_fields;

// Export all types
pub use collection::*;
pub use datalake::*;
pub use rlp_fields::*;

#[cfg(test)]
mod tests {
    use crate::datalake::{Datalake, DatalakeCollection};

    use super::*;

    #[test]
    fn test_transactions_datalake() {
        let encoded_datalake= "0x000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000020100000000000000000000000000000000000000000000000000000000000000";

        let transaction_datalake =
            TransactionsInBlockDatalake::new(1000000, "tx.nonce".to_string(), 1).unwrap();

        let encoded = transaction_datalake.encode().unwrap();

        assert_eq!(encoded, encoded_datalake);

        assert_eq!(
            transaction_datalake.commit(),
            "0xd1cc3bc248fcb1b57b249b27b79eb317cf9f38e3b4ec157f99f116d22d0d0b6a"
        );

        assert_eq!(
            transaction_datalake.sampled_property,
            TransactionsCollection::Transactions(TransactionField::Nonce)
        );

        let decoded = TransactionsInBlockDatalake::decode(&encoded).unwrap();
        assert_eq!(decoded, transaction_datalake);
    }

    #[test]
    fn test_transactions_datalake_receipt() {
        let encoded_datalake = "0x000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000020200000000000000000000000000000000000000000000000000000000000000";
        let transaction_datalake =
            TransactionsInBlockDatalake::new(1000000, "tx_receipt.success".to_string(), 1).unwrap();

        let encoded = transaction_datalake.encode().unwrap();

        assert_eq!(encoded, encoded_datalake);

        assert_eq!(
            transaction_datalake.commit(),
            "0x1d0ff6dcd6eba2d1b638c469bd1d68a479173bace1f13922be20defa74cbacfb"
        );

        assert_eq!(
            transaction_datalake.sampled_property,
            TransactionsCollection::TranasactionReceipts(TransactionReceiptField::Success)
        );

        let decoded = TransactionsInBlockDatalake::decode(&encoded).unwrap();
        assert_eq!(decoded, transaction_datalake);
    }

    #[test]
    fn test_tx_collection_serialize() {
        let tx_collection = TransactionsCollection::Transactions(TransactionField::Nonce);
        let serialized = tx_collection.serialize().unwrap();
        assert_eq!(serialized, [1, 0]);

        let tx_collection =
            TransactionsCollection::TranasactionReceipts(TransactionReceiptField::Logs);
        let serialized = tx_collection.serialize().unwrap();
        assert_eq!(serialized, [2, 2]);

        let tx_collection = TransactionsCollection::Transactions(TransactionField::AccessList);
        let serialized = tx_collection.serialize().unwrap();
        assert_eq!(serialized, [1, 10]);
    }

    #[test]
    fn test_tx_collection_deserialize() {
        let serialized = [1, 1];
        let tx_collection = TransactionsCollection::deserialize(&serialized).unwrap();
        assert_eq!(
            tx_collection,
            TransactionsCollection::Transactions(TransactionField::GasPrice)
        );

        let serialized = [2, 3];
        let tx_collection = TransactionsCollection::deserialize(&serialized).unwrap();
        assert_eq!(
            tx_collection,
            TransactionsCollection::TranasactionReceipts(TransactionReceiptField::Bloom)
        );

        let serialized = [1, 10];
        let tx_collection = TransactionsCollection::deserialize(&serialized).unwrap();
        assert_eq!(
            tx_collection,
            TransactionsCollection::Transactions(TransactionField::AccessList)
        );
    }
}
