extern crate parity_hash as hash;
extern crate secp256k1;
extern crate rustc_hex as hex;
extern crate rand;
extern crate tiny_keccak;
extern crate num_cpus;

use rand::Rng;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread;

const START_MASK: &'static str = "00bad0";

pub fn keccak<T>(input: T) -> [u8; 32] where T: AsRef<[u8]> {
	let mut keccak = tiny_keccak::Keccak::new_keccak256();
	let mut res = [0u8; 32];
	keccak.update(input.as_ref());
	keccak.finalize(&mut res);
	res
}

enum Message {
	Address{ public: [u8; 64], secret: [u8; 32], address: [u8; 20] },
	Progress(usize),
}

fn main_loop(counter: Arc<AtomicUsize>, tx: Sender<Message>) {
	let start = hex::FromHex::from_hex(START_MASK).unwrap();
	let secp = secp256k1::Secp256k1::new();
	let mut rng = rand::os::OsRng::new().expect("Failed to generate");
	let mut address = [0u8; 20];
	let mut random_slice = [0u8; 32];

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
			let mut public = [0u8; 64];
			let mut secret = [0u8; 32];
			public.copy_from_slice(&serialized[1..65]);
			secret.copy_from_slice(&random_slice[..]);
			tx.send(
				Message::Address {
					public: public,
					secret: secret,
					address: address.clone()
				 }
			).expect("Failed to send");
		}

		let counter_val = counter.fetch_add(1, Ordering::SeqCst);
		if counter_val % 100000 == 0 {
			tx.send(Message::Progress(counter_val)).expect("Failed to send");
		}
	}
}

fn main() {

	let counter = Arc::new(AtomicUsize::new(0));
	let (sender, receiver) = mpsc::channel();

	for _ in 0..num_cpus::get() {
		let sender = sender.clone();
		let counter = counter.clone();
		thread::spawn(move || {
			main_loop(counter, sender);
		});
	}

	loop {
		match receiver.recv() {
			Ok(Message::Progress(progress)) => {
				println!("{} generated", progress);
			},
			Ok(Message::Address { public, secret, address }) => {
				println!("Found bad ass address: ");
				println!("secret:  {}", hex::ToHex::to_hex(&secret[..]));
				println!("public:  {}", hex::ToHex::to_hex(&public[..]));
				println!("address: {}", hex::ToHex::to_hex(&address[..]));
			},
			Err(e) => {
				println!("warn: some thread failed: {}", e)
			}
		};
		thread::park_timeout(::std::time::Duration::from_millis(50));
	}
}