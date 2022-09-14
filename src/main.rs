use codec::Encode;
use subxt::{tx::PairSigner, OnlineClient, SubstrateConfig, PolkadotConfig};
use polkadex_primitives::snapshot::EnclaveSnapshot;
use polkadex_primitives::{AccountId, AssetsLimit, SnapshotAccLimit, WithdrawalLimit};
use sp_core::{H256, Pair, sr25519};
//use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, PairSigner};
use sp_runtime::{BoundedVec,BoundedBTreeMap};
use crate::polkadex::runtime_types::sp_runtime::MultiSignature;

#[subxt::subxt(runtime_metadata_path = "src/metadata.scale"
)]
pub mod polkadex {
    // #[subxt(substitute_type = "sp_core::crypto::AccountId32")]
    // use crate::AccountId;
    //
    // #[subxt(substitute_type = "runtime_typesx::polkadex_primitives::snapshot::EnclaveSnapshot")]
    // use crate::EnclaveSnapshot;
    // #[subxt(substitute_type = "runtime_types::sp_runtime::bounded::bounded_btree_map::BoundedBTreeMap")]
    // use crate::BoundedBTreeMap;
    // #[subxt(substitute_type = "BoundedVec")]
    // use crate::BoundedVec;
}

#[tokio::main]
async fn main() {
    let (pair, _) = sr25519::Pair::generate();
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
    // let snapshot: EnclaveSnapshot<AccountId,WithdrawalLimit,AssetsLimit,SnapshotAccLimit> = crate::EnclaveSnapshot {
    //     snapshot_number: 10,
    //     merkle_root: H256::random(),
    //     withdrawals: BoundedBTreeMap::default(),
    //     fees: BoundedVec::default(),
    // };
    //let withdrwal = crate::polkadex::runtime_types::sp_runtime::bounded::bounded_btree_map::BoundedBTreeMap::try_from(vec![]);
    //let fee = crate::polkadex::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec::try_from(vec![]);

    // let snapshott = crate::polkadex::runtime_types::polkadex_primitives::snapshot::EnclaveSnapshot::<AccountId,WithdrawalLimit,AssetsLimit,SnapshotAccLimit>{
    //     snapshot_number: 10,
    //         merkle_root: H256::random(),
    //         withdrawals: BoundedBTreeMap::default(),
    //         fees: BoundedVec::default(),
    // };
    //
    // let signature = pair.sign(&snapshot.encode()).0;
    // let sig = crate::polkadex::runtime_types::sp_core::sr25519::Signature(signature);
    // let mutli = crate::polkadex::runtime_types::sp_runtime::MultiSignature::Sr25519(sig);

    let txn = polkadex::tx().ocex().shutdown();

    let result = api
        .tx()
        .sign_and_submit_then_watch_default(&txn, & PairSigner::new(pair))
        .await.unwrap()
        .wait_for_finalized_success()
        .await.unwrap();
}