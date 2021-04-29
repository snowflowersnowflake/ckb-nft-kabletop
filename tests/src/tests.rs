use super::{
    helper::{sign_tx, blake160, MAX_CYCLES},
    *,
};
use ckb_testtool::context::Context;
use ckb_tool::{
    ckb_crypto::secp::{Generator, Privkey},
    ckb_hash::blake2b_256,
    ckb_types::{
        bytes::Bytes,
        core::{TransactionBuilder, TransactionView},
        packed::{CellDep, CellOutput, CellInput},
        prelude::*,
    },
};

fn get_keypair() -> (Privkey, Bytes) {
    let keypair = Generator::random_keypair();
    let compressed_pubkey = keypair.1.serialize();
    let script_args = Bytes::from(helper::blake160(compressed_pubkey.to_vec().as_slice()).to_vec());
    let privkey = keypair.0;
    (privkey, script_args)
}

fn get_nfts(count: u8) -> Vec<u8> {
    let mut nfts: Vec<u8> = vec![];
    for i in 0..count {
        nfts.append(&mut blake160(&i.to_be_bytes()).to_vec());
    }
    return nfts;
}

#[test]
fn test_success_origin_to_challenge() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("kabletop");
    let out_point = context.deploy_cell(contract_bin);

    // generate two users' privkey and pubkhash
    let (user1_privkey, user1_pkhash) = get_keypair();
    let (user2_privkey, user2_pkhash) = get_keypair();

    // prepare scripts
    let mut lock_args: Vec<u8> = vec![];
    lock_args.append(&mut 500u64.to_be_bytes().to_vec());   // staking_ckb
    lock_args.push(5u8);                                    // deck_size
    lock_args.append(&mut 1024u64.to_be_bytes().to_vec());  // block_number
    lock_args.append(&mut blake2b_256([1]).to_vec());       // lock_code_hash
    lock_args.append(&mut user1_pkhash.to_vec());           // user1_pkhash
    lock_args.append(&mut get_nfts(5));                     // user1_nft_collection
    lock_args.append(&mut user2_pkhash.to_vec());           // user1_pkhash
    lock_args.append(&mut get_nfts(5));                     // user1_nft_collection

    let lock_script = context
        .build_script(&out_point, Bytes::from(lock_args))
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(2000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let output = CellOutput::new_builder()
        .capacity(2000u64.pack())
        .lock(lock_script.clone())
        .build();

    // prepare witnesses
    

    let outputs_data = vec![Bytes::new(); 1];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx(tx, &user1_privkey);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass test_success_origin_to_challenge");
    println!("consume cycles: {}", cycles);
}
