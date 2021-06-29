use super::{
    protocol, *,
};
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{
	bytes::Bytes, core::TransactionBuilder, prelude::*,
	packed::{
		CellDep, CellOutput, CellInput, WitnessArgs
	}
};

const MAX_CYCLES: u64 = 100_000_000;

fn get_round(user_type: u8, lua_code: Vec<&str>) -> Bytes {
    let user_round = protocol::round(user_type, lua_code);
    Bytes::from(protocol::to_vec(&user_round))
}

#[test]
fn test_molecule() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("kabletop");
    let out_point = context.deploy_cell(contract_bin);

    let lock_script = context
        .build_script(&out_point, Bytes::new())
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
    let witnesses = vec![
        get_round(1u8, vec![
			"print('用户1的回合：')",	// 这里注释掉就没有问题
			"spell('用户1', '用户2', '36248218d2808d668ae3c0d35990c12712f6b9d2')",
			"print('abc123abc123abc123abc123abc123abc123abc123abc123')"
		]),
        get_round(2u8, vec![
			"print('user2 draw one card, and skip current round.')"
		]),
    ].iter().map(|code| {
		WitnessArgs::new_builder()
            .lock(Some(Bytes::from(vec![0u8; 65])).pack())	// 这里注释掉就没有问题
            .input_type(Some(code.clone()).pack())
            .build()
			.as_bytes()
			.pack()
	}).collect::<Vec<_>>();
    let outputs_data = vec![Bytes::new()];

	println!(" ");
	println!("// ========================================================================================");
	println!(" ");

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output)
        .outputs_data(outputs_data.pack())
		.witnesses(witnesses)
        .cell_dep(lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass molecule");

	println!(" ");
	println!("// 对比RUST端和C端的第三排，两边数据不一致，但神奇的是稍微改动一些东西就没问题了，比如：1.减少一排数据；2.WitnessArgs不填写lock字段");
	println!(" ");
}
