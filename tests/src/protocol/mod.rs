
mod kabletop;
use molecule::prelude::{
	Byte, Builder, Entity
};
use kabletop::{
	Round, Operations
};

fn uint8_t(v: u8) -> kabletop::Uint8T {
    kabletop::Uint8TBuilder::default().set([Byte::from(v); 1]).build()
}

fn bytes_t(v: &[u8]) -> kabletop::Bytes {
    let bytes = v
        .to_vec()
        .iter()
        .map(|byte| Byte::new(byte.clone()))
        .collect::<Vec<Byte>>();
    kabletop::Bytes::new_builder()
        .set(bytes)
        .build()
}

#[allow(dead_code)]
pub fn to_vec<T: Entity>(t: &T) -> Vec<u8> {
    t.as_bytes().to_vec()
}

#[allow(dead_code)]
pub fn round(user_type: u8, operations: Vec<&str>) -> Round {
    let operations = operations
        .iter()
        .map(|bytes| {
			println!("[contract debug] RUST = \"{}\"", hex::encode(bytes.as_bytes()));
			bytes_t(bytes.as_bytes())
		})
        .collect::<Vec<kabletop::Bytes>>();
    let operations = Operations::new_builder()
        .set(operations)
        .build();
    Round::new_builder()
        .user_type(uint8_t(user_type))
        .operations(operations)
        .build()
}
