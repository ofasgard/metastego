use std::fs;
use std::env;
use std::io::Write;
use std::collections::HashMap;

// Create an metasteganographic oracle from an array of bytes.
// If it fails to find a corresponding value for a byte, it will return an error with the byte that failed.
fn create_oracle(buf : &[u8]) -> Result<HashMap<u8, u32>,u8> {
	let mut oracle : HashMap<u8, u32> = HashMap::new();
	
	for i in 0..256 {
		let byte = i as u8;
		let buflen : u32 = buf.len() as u32;
		
		for offset in 0..buflen {
			let current_value = buf[offset as usize];
			if current_value == byte {
				oracle.insert(byte, offset);
				break;
			}
		}
		if !oracle.contains_key(&byte) {
			return Err(byte);
		}
	}
	
	Ok(oracle)
}

// Use an oracle to encode a payload metasteganographically.
// If it fails to translate a byte from the payload, it will return an error with the byte that failed.
fn metasteg_encode(payload: &[u8], oracle: &HashMap<u8, u32>) -> Result<Vec<u32>,u8> {
	let mut encoded : Vec<u32> = Vec::new();
	for byte in payload {
		let encoded_offset = match oracle.get(byte) {
			Some(b) => *b,
			None => return Err(*byte)
		};
		encoded.push(encoded_offset);
	}
	Ok(encoded)
}

// Use the original buffer to decode a payload metasteganographically.
// If it fails to translate an offset from the payload, it will return an error with the offset that failed.
fn metasteg_decode(payload: &[u32], buf: &[u8]) -> Result<Vec<u8>,u32> {
	let mut decoded : Vec<u8> = Vec::new();
	for offset in payload {
		if (*offset as usize) > buf.len() {
			return Err(*offset);
		}
		let decoded_byte = buf[*offset as usize];
		decoded.push(decoded_byte);
	}
	Ok(decoded)
}

fn encode_file(input_path: &str, output_path: &str, image_path: &str) -> Result<(),String> {
	// Read in the payload and the image used to encode it.
	let payload : Vec<u8> = match fs::read(input_path) {
		Ok(x) => x,
		Err(e) => return Err(e.to_string())
	};
	let image : Vec<u8> = match fs::read(image_path) {
		Ok(x) => x,
		Err(e) => return Err(e.to_string())
	};
	// Create an oracle from the image.
	let oracle = match create_oracle(&image) {
		Ok(x) => x,
		Err(e) => return Err(format!("Failed to create oracle; could not produce an offset for value 0x{:02x}", e))
	};
	// Encode the payload with the oracle.
	let encoded_payload = match metasteg_encode(&payload, &oracle) {
		Ok(x) => x,
		Err(e) => return Err(format!("Failed to encode payload with oracle; failed on byte {}", e))
	};
	// Write the encoded payload to a file.
	let mut output = match fs::File::create(output_path) {
		Ok(x) => x,
		Err(e) => return Err(e.to_string())
	};
	for offset in encoded_payload {
		let offset_bytes = offset.to_be_bytes();
		match output.write_all(&offset_bytes) {
			Ok(_) => (),
			Err(e) => return Err(e.to_string())
		};
	}
	
	Ok(())
}

fn decode_file(input_path: &str, output_path: &str, image_path: &str) -> Result<(),String> {
	// Read in the encoded/serialized payload and the image used to encode it.
	let serialized_payload : Vec<u8> = match fs::read(input_path) {
		Ok(x) => x,
		Err(e) => return Err(e.to_string())
	};
	let image : Vec<u8> = match fs::read(image_path) {
		Ok(x) => x,
		Err(e) => return Err(e.to_string())
	};
	// Check and deserialize the payload.
	if serialized_payload.len() % 4 != 0 {
		return Err(format!("Serialized payload has an invalid length: {}", serialized_payload.len()));
	}
	
	let mut payload : Vec<u32> = Vec::new();
	let mut i = 0;
	loop {
		if i >= serialized_payload.len() { break; }
		let current_offset_serialized : [u8;4] = serialized_payload[i..i+4].try_into().unwrap();
		let current_offset = u32::from_be_bytes(current_offset_serialized);
		payload.push(current_offset);
		i += 4;
	}
	// Decode the payload with the image.
	let decoded_payload = match metasteg_decode(&payload, &image) {
		Ok(x) => x,
		Err(e) => return Err(format!("Failed to decode payload with image; failure on offset {}", e))
	};
	// Write the decoded payload to a file.
	match fs::write(output_path, decoded_payload) {
		Ok(_) => (),
		Err(e) => return Err(e.to_string())
	};
	
	Ok(())
}

fn usage() {
	let args : Vec<String> = env::args().collect();
	println!("USAGE: {} [encode|decode]", args[0]);
	println!("\tencode <path to plaintext payload> <output path> <image to use>");
	println!("\tdecode <path to encoded payload> <output path> <image to use>");
}

fn main() {
	let args : Vec<String> = env::args().collect();
	
	if args.len() < 5 { return usage(); }
	
	let input_path = args[2].to_string();
	let output_path = args[3].to_string();
	let image_path = args[4].to_string();
	
	match args[1].as_str() {
		"encode" => {
			match encode_file(&input_path, &output_path, &image_path) {
				Ok(_) => println!("Successfully encoded '{}' with '{}', result stored in '{}'", input_path, image_path, output_path),
				Err(e) => println!("Failed to encode '{}' with '{}': {}", input_path, image_path, e)
			}
		},
		"decode" => {
			match decode_file(&input_path, &output_path, &image_path) {
				Ok(_) => println!("Successfully decoded '{}' with '{}', result stored in '{}'", input_path, image_path, output_path),
				Err(e) => println!("Failed to decode '{}' with '{}': {}", input_path, image_path, e)
			}
		},
		_ => return usage()
	};
}
