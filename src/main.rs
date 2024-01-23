use std::{env, fs::read_to_string};
use sha1::{Sha1, Digest};
use rs_sha256::Sha256Hasher;
use std::hash::Hasher;
use md5;

fn main() {
    // get command line arguments
    // command line arguments should include the hash algorithm method the user prefers and a file path;
    let cmd_args: Vec<String> = env::args().collect();
    let cmd_args_len = cmd_args.len();

    let algos: [String; 3] = [String::from("MD5"), String::from("SHA-1"), String::from("SHA-256")];

    //given that there should be two commands in addition to the standard command, throw an error if the arguments length isnt three

    if cmd_args_len != 3 as usize {
        panic!("Please include the hash algorithm method and a file path");
    }

    // get properties from command
    let method_type = &cmd_args[1];
    let file_path = &cmd_args[2];

    println!("{} {}", method_type, file_path);

    // check if the chosen method isnt
    if !algos.contains(&method_type.to_uppercase()) {
        panic!("Please include a valid hash algorithm method. Available options include: MD5, SHA-1, SHA-256");
    }

    println!("now let's see if we can find the file");

    let file = read_to_string(file_path);

    match file {
        Ok(val) => {
            println!("File found: {}", val);

            match method_type.to_uppercase().as_str() {
                "MD5" => println!("{}", hash_with_md5(file_path)),
                "SHA-1" => println!("{}", hash_with_sha1(file_path)),
                "SHA-256" => println!("{}", hash_with_sha256(file_path)),
                _ => panic!("Please include a valid hash algorithm method. Available options include: MD5, SHA-1, SHA-256"),
            }
        },
        Err(_) => panic!("File not found"),
    }

}




fn hash_with_md5(file_path: &String) -> String {
    let hash_val = md5::compute(file_path);
    format!("{:x}", hash_val)
}

fn hash_with_sha256(input: &String) -> String {
    let mut sha256hasher = Sha256Hasher::default();
    sha256hasher.write(input.as_bytes());
    let hash = sha256hasher.finish();
    format!("{:x}", hash)
}

fn hash_with_sha1(input:&String) -> String {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(input.as_bytes());

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();

    // create String from hash digest in the form of hex
    format!("{:x}", result)

}