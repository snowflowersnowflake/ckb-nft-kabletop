use super::{
    helper::{sign_tx, blake160, MAX_CYCLES, gen_witnesses_and_signature},
    protocol,
    *,
};
use ckb_system_scripts::BUNDLED_CELL;
use ckb_testtool::{
    builtin::ALWAYS_SUCCESS,
    context::Context
};
use ckb_tool::{
    ckb_crypto::secp::{Generator, Privkey},
    ckb_hash::blake2b_256,
    ckb_types::{
        bytes::Bytes,
        core::{TransactionBuilder},
        packed::{CellDep, CellOutput, CellInput},
        prelude::*,
    },
};

fn get_keypair() -> (Privkey, [u8; 20]) {
    let keypair = Generator::random_keypair();
    let compressed_pubkey = keypair.1.serialize();
    let script_args = blake160(compressed_pubkey.to_vec().as_slice());
    let privkey = keypair.0;
    (privkey, script_args)
}

fn get_nfts(count: u8) -> Vec<[u8; 20]> {
    let mut nfts = vec![];
    for i in 0..count {
        nfts.push(blake160(&i.to_be_bytes()));
    }
    return nfts;
}

fn get_round(user_type: u8, lua_code: Vec<&str>) -> Bytes {
    let user_round = protocol::round(user_type, lua_code);
    Bytes::from(protocol::to_vec(&user_round))
}

#[test]
fn test_success_origin_to_challenge() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("kabletop");
    let out_point = context.deploy_cell(contract_bin);
    let secp256k1_data_bin = BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap();
    let secp256k1_data_out_point = context.deploy_cell(secp256k1_data_bin.to_vec().into());
    let secp256k1_data_dep = CellDep::new_builder()
        .out_point(secp256k1_data_out_point)
        .build();

    // generate two users' privkey and pubkhash
    let (user1_privkey, user1_pkhash) = get_keypair();
    let (user2_privkey, user2_pkhash) = get_keypair();

    // println!("user1_pkhash = {}", hex::encode(user1_pkhash));
    // println!("user2_pkhash = {}", hex::encode(user2_pkhash));

    // prepare scripts
    let lock_args_molecule = (500u64, 5u8, 1024u64, blake2b_256([1]), user1_pkhash, get_nfts(5), user2_pkhash, get_nfts(5));
    let lock_args = protocol::lock_args(lock_args_molecule);

    let lock_script = context
        .build_script(&out_point, Bytes::from(protocol::to_vec(&lock_args)))
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
    let end_round = protocol::round(1u8, vec![
        "print('user2 draw one card, and skip current round.')",
        // "_winner = 1"
    ]);
    let end_round_bytes = Bytes::from(protocol::to_vec(&end_round));
    let witnesses = vec![
        (&user2_privkey, get_round(1u8, vec!["
			print('用户1的回合：')
			print('1.抽牌')
			print('2.回合结束')
		"])),
        (&user1_privkey, get_round(2u8, vec!["
			print('用户2的回合：')
			print('1.抽牌')
			print('2.放置一张牌，跳过回合')
			print('3.回合结束')
		"])),
        (&user2_privkey, get_round(1u8, vec!["
			print('用户1的回合：')
			spell('用户1', '用户2', '36248218d2808d668ae3c0d35990c12712f6b9d2')
			print('abc123abc123abc123abc123abc123abc123abc123abc123')
			print('2.回合结束')
		"])),
        (&user1_privkey, get_round(2u8, vec!["
			print('用户2的回合：')
			print('1.认输')
			print('2.回合结束')
		"])),
        (&user2_privkey, end_round_bytes),
    ];
    let (witnesses, signature) = gen_witnesses_and_signature(&lock_script, 2000u64, witnesses);
    let challenge = protocol::challenge((witnesses.len() - 1) as u8, signature, end_round);
    let outputs_data = vec![Bytes::from(protocol::to_vec(&challenge))];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(secp256k1_data_dep)
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx(tx, &user1_privkey, witnesses);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass test_success_origin_to_challenge");
    println!("consume cycles: {}", cycles);
}

#[test]
fn test_success_origin_to_settlement() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("kabletop");
    let out_point = context.deploy_cell(contract_bin);
    let secp256k1_data_bin = BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap();
    let secp256k1_data_out_point = context.deploy_cell(secp256k1_data_bin.to_vec().into());
    let secp256k1_data_dep = CellDep::new_builder()
        .out_point(secp256k1_data_out_point)
        .build();
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    let always_success_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();

    // generate two users' privkey and pubkhash
    let (user1_privkey, user1_pkhash) = get_keypair();
    let (user2_privkey, user2_pkhash) = get_keypair();

    // prepare scripts
    let code_hash: [u8; 32] = blake2b_256(ALWAYS_SUCCESS.to_vec());
    let lock_args_molecule = (500u64, 5u8, 1024u64, code_hash, user1_pkhash, get_nfts(5), user2_pkhash, get_nfts(5));
    let lock_args = protocol::lock_args(lock_args_molecule);

    let lock_script = context
        .build_script(&out_point, Bytes::from(protocol::to_vec(&lock_args)))
        .expect("lock_script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(out_point)
        .build();
    let user1_always_success_script = context
        .build_script(&always_success_out_point, Bytes::from(user1_pkhash.to_vec()))
        .expect("user1 always_success_script");
    let user2_always_success_script = context
        .build_script(&always_success_out_point, Bytes::from(user2_pkhash.to_vec()))
        .expect("user2 always_success_script");

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
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(1500.pack())
            .lock(user1_always_success_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500.pack())
            .lock(user2_always_success_script.clone())
            .build()
    ];

    // prepare witnesses
    let end_round = protocol::round(2u8, vec![
        "ckb.debug('user2 draw one card, and surrender the game.')",
        "_winner = 1"
    ]);
    let end_round_bytes = Bytes::from(protocol::to_vec(&end_round));
    let witnesses = vec![
        (&user2_privkey, get_round(1u8, vec!["ckb.debug('user1 draw one card, and spell it adding HP.')"])),
        (&user1_privkey, get_round(2u8, vec!["ckb.debug('user2 draw one card, and spell it to damage user1.')"])),
        (&user2_privkey, get_round(1u8, vec!["ckb.debug('user1 draw one card, and use it to kill user2.')"])),
        (&user1_privkey, get_round(2u8, vec!["ckb.debug('user2 draw one card, and put it onto battleground.')"])),
        (&user2_privkey, get_round(1u8, vec!["ckb.debug('user1 draw one card, and use it to kill user2.')"])),
        (&user1_privkey, end_round_bytes),
    ];
    let (witnesses, _) = gen_witnesses_and_signature(&lock_script, 2000u64, witnesses);
    let outputs_data = vec![Bytes::new(), Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(secp256k1_data_dep)
        .cell_dep(always_success_script_dep)
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx(tx, &user1_privkey, witnesses);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass test_success_origin_to_settlement");
    println!("consume cycles: {}", cycles);
}

#[test]
fn test_success_timeout_to_settlement() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("kabletop");
    let out_point = context.deploy_cell(contract_bin);
    let secp256k1_data_bin = BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap();
    let secp256k1_data_out_point = context.deploy_cell(secp256k1_data_bin.to_vec().into());
    let secp256k1_data_dep = CellDep::new_builder()
        .out_point(secp256k1_data_out_point)
        .build();
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    let always_success_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();

    // generate two users' privkey and pubkhash
    let (user1_privkey, user1_pkhash) = get_keypair();
    let (user2_privkey, user2_pkhash) = get_keypair();

    // prepare scripts
    let code_hash: [u8; 32] = blake2b_256(ALWAYS_SUCCESS.to_vec());
    let lock_args_molecule = (500u64, 5u8, 10000u64, code_hash, user1_pkhash, get_nfts(5), user2_pkhash, get_nfts(5));
    let lock_args = protocol::lock_args(lock_args_molecule);

    let lock_script = context
        .build_script(&out_point, Bytes::from(protocol::to_vec(&lock_args)))
        .expect("lock_script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(out_point)
        .build();
    let user1_always_success_script = context
        .build_script(&always_success_out_point, Bytes::from(user1_pkhash.to_vec()))
        .expect("user1 always_success_script");
    let user2_always_success_script = context
        .build_script(&always_success_out_point, Bytes::from(user2_pkhash.to_vec()))
        .expect("user2 always_success_script");

    // prepare witnesses
    let end_round = protocol::round(2u8, vec![
        "ckb.debug('user2 draw one card, and quit the game without responding.')",
        // "_winner = 1"
    ]);
    let end_round_bytes = Bytes::from(protocol::to_vec(&end_round));
    let witnesses = vec![
        (&user2_privkey, get_round(1u8, vec!["ckb.debug('user1 draw one card, and spell it adding HP.')"])),
        (&user1_privkey, get_round(2u8, vec!["ckb.debug('user2 draw one card, and spell it to damage user1.')"])),
        (&user2_privkey, get_round(1u8, vec!["ckb.debug('user1 draw one card, and use it to kill user2.')"])),
        (&user1_privkey, get_round(2u8, vec!["ckb.debug('user2 draw one card, and put it onto battleground.')"])),
        (&user2_privkey, get_round(1u8, vec!["ckb.debug('user1 draw one card, and use it to kill user2.')"])),
        (&user1_privkey, end_round_bytes),
    ];
    let (witnesses, signature) = gen_witnesses_and_signature(&lock_script, 2000u64, witnesses);
    let challenge = protocol::challenge((witnesses.len() - 1) as u8, signature, end_round);

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(2000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::from(protocol::to_vec(&challenge)),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .since(10036u64.pack())
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(1500.pack())
            .lock(user1_always_success_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500.pack())
            .lock(user2_always_success_script.clone())
            .build()
    ];
    let outputs_data = vec![Bytes::new(), Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(secp256k1_data_dep)
        .cell_dep(always_success_script_dep)
        .build();
    let tx = context.complete_tx(tx);
    let tx = sign_tx(tx, &user1_privkey, witnesses);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass test_success_timeout_to_settlement");
    println!("consume cycles: {}", cycles);
}
