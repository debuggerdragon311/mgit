use std::fs;
use std::fs::File;

/// responsible for creating the .mgit/ dir structure
pub fn run(){

    // root
    match fs::create_dir(".mgit") {
        Ok(_) => {
            if cfg!(debug_assertions) {
                println!("[DEBUG] initializing repository...");
            }
        }
        Err(_) => {
        // Err(error) => {
            // eprintln!("Could not initialize repository. Details: {}", error);
            eprintln!("repository already exists...");
            std::process::exit(1);
        }
    };

    // refs
    match fs::create_dir(".mgit/refs"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: refs/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create refs. Details: {}", error);
            std::process::exit(1);
        }
    };
    // refs/heads/
    match fs::create_dir(".mgit/refs/heads"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: heads/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create heads/. Details: {}", error);
            std::process::exit(1);
        }
    };
    // refs/tags
    match fs::create_dir(".mgit/refs/tags"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: tags/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create tags/. Details: {}", error);
            std::process::exit(1);
        }
    };

    // objects
    match fs::create_dir(".mgit/objects"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: objects/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create objects. Details: {}", error);
            std::process::exit(1);
        }
    };
    // objects/info
    match fs::create_dir(".mgit/objects/info"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: info/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create info/. Details: {}", error);
            std::process::exit(1);
        }
    };
    // objects/pak
    match fs::create_dir(".mgit/objects/pak"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: pak/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create pak/. Details: {}", error);
            std::process::exit(1);
        }
    };

    // hooks
    match fs::create_dir(".mgit/hooks"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: hooks/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create hooks. Details: {}", error);
            std::process::exit(1);
        }
    };

    // HEAD
    match File::create(".mgit/HEAD"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: HEAD created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create HEAD. Details: {}", error);
            std::process::exit(1);
        }
    };
    // info
    match fs::create_dir(".mgit/info"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: info/ created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create info. Details: {}", error);
            std::process::exit(1);
        }
    };
    // description
    match File::create(".mgit/description"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: description created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create description. Details: {}", error);
            std::process::exit(1);
        }
    };
    // config
    match File::create(".mgit/config"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: config created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create config. Details: {}", error);
            std::process::exit(1);
        }
    };
    // info/exclude
    match File::create(".mgit/info/exclude"){
        Ok(_) => {
            if cfg!(debug_assertions){
                println!("[DEBUG]: info/exclude created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create info/exclude. Details: {}", error);
            std::process::exit(1);
        }
    };
}
