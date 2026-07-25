/**************************************************************************/
/*                                init.rs                                 */
/**************************************************************************/
/*                         This file is part of:                          */
/*                                  mgit                                  */
/**************************************************************************/
/*                  Copyright (C) 2026  Soumyajit Bala                    */
/*                                                                        */
/* This program is free software: you can redistribute it and/or modify   */
/* it under the terms of the GNU Affero General Public License as         */
/* published by the Free Software Foundation, either version 3 of the     */
/* License, or (at your option) any later version.                        */
/*                                                                        */
/* This program is distributed in the hope that it will be useful,        */
/* but WITHOUT ANY WARRANTY; without even the implied warranty of         */
/* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          */
/* GNU Affero General Public License for more details.                    */
/*                                                                        */
/* You should have received a copy of the GNU Affero General Public       */
/* License along with this program.  If not, see                          */
/* <https://www.gnu.org/licenses/>.                                       */
/**************************************************************************/

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
    // objects/info (future-use)
    // match fs::create_dir(".mgit/objects/info"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: info/ created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create info/. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };

    // objects/pak (future-use)
    // match fs::create_dir(".mgit/objects/pak"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: pak/ created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create pak/. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };

    // hooks (future-use)
    // match fs::create_dir(".mgit/hooks"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: hooks/ created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create hooks. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };

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
    // info (future-use)
    // match fs::create_dir(".mgit/info"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: info/ created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create info. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };

    // description (future-use)
    // match File::create(".mgit/description"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: description created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create description. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };

    // config (future-use)
    // match File::create(".mgit/config"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: config created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create config. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };

    // info/exclude (future-use)
    // match File::create(".mgit/info/exclude"){
    //     Ok(_) => {
    //         if cfg!(debug_assertions){
    //             println!("[DEBUG]: info/exclude created...");
    //         }
    //     }
    //     Err(error) => {
    //         eprintln!("Could not create info/exclude. Details: {}", error);
    //         std::process::exit(1);
    //     }
    // };
}
