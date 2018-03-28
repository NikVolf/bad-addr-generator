extern crate parity_hash as hash;
extern crate secp256k1;
extern crate rustc_hex as hex;
extern crate rand;
extern crate tiny_keccak;

use rand::Rng;

const START_MASK: &'static str = "00bad0";

pub fn keccak<T>(input: T) -> [u8; 32] where T: AsRef<[u8]> {
	let mut keccak = tiny_keccak::Keccak::new_keccak256();
	let mut res = [0u8; 32];
	keccak.update(input.as_ref());
	keccak.finalize(&mut res);
	res
}

fn main() {
	let start = hex::FromHex::from_hex(START_MASK).unwrap();
	let secp = secp256k1::Secp256k1::new();
	let mut rng = rand::os::OsRng::new().expect("Failed to generate");
	let mut address = [0u8; 20];
	let mut random_slice = [0u8; 32];

	let mut counter = 0;
	loop {
		rng.fill_bytes(&mut random_slice[..]);
		let secret = match secp256k1::key::SecretKey::from_slice(&secp, &random_slice[..]) {
			Ok(s) => s,
			_ => { continue; },
		};
		let public = match secp256k1::key::PublicKey::from_secret_key(&secp, &secret) {
			Ok(p) => p,
			_ => { continue; },
		};

		let serialized = public.serialize_vec(&secp, false);
		let serialized_hash = keccak(&serialized[1..65]);
		address.copy_from_slice(&serialized_hash[12..]);
		if address.starts_with(&start[..]) {
			println!("public:  {}", hex::ToHex::to_hex(&serialized[1..65]));
			println!("secret:  {}", hex::ToHex::to_hex(&random_slice[..]));
			println!("address: {}", hex::ToHex::to_hex(&address[..]));
			break;
		} else if counter % 100000 == 0 {
			println!("{} generated", counter);
		}
		counter += 1;
	}
}