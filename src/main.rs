use std::io;

#[derive(Debug)]
struct VFT {
    display_name: String,
    id: String,
    image_path: String,
    crypto_cost: f32
}

impl Default for VFT {
    fn default() -> VFT {
        VFT {
            display_name: String::from("Boring default name"),
            id: String::from("default_id"),
            image_path: String::from("boring_default_image_path.png"),
            crypto_cost: 100.1
        }
    }
}

#[derive(Debug)]
struct Folder { // TODO: Think of better name for this
    name: String,
    vfts: Vec<VFT>
}

#[derive(Debug)]
struct Blockchain { // TODO: Think of better name for this
    folders: Vec<Folder>
}

fn get_blockchain() -> Blockchain {
    // TODO: Change to Blockchain new name
    let blockchain : Blockchain = Blockchain {
        folders: vec![
            Folder {
                name: String::from("wa"),
                vfts: vec![
                    VFT {
                        display_name: String::from("Test VFT"),
                        id: String::from("test_vft"),
                        image_path: String::from("test.png"),
                        crypto_cost: 20.1
                    },
                    VFT {
                        ..Default::default()
                    }
                ]
            },
            Folder {
                name: String::from("woo"),
                vfts: vec![
                    VFT {
                        ..Default::default()
                    }
                ]
            }
        ]
    };

    return blockchain
}

fn query_data() {
    println!("\nWhat data would you like to query?");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {},
    }
    let input = input.trim();

    let user_query: Vec<&str> = input.split(" ").collect();

    //println!("{}", input)
    match user_query[0] {
        "folder" => {
            println!("Querying folder: {}", user_query[1]);
            
            let mut success = false;
            for folder in get_blockchain().folders {
                if folder.name == user_query[1] {
                    success = true;
                    println!("\n-------------------------------------------------------------");
                    println!("{:#?}", folder);
                }
            }

            if success == false {
                println!("Folder Query failed")
            } else {
                println!("-------------------------------------------------------------\n");
                println!("Query successful: folder/{}", user_query[1]);
            }
        }
        "vft" => {
            println!("Querying VFT: {}", user_query[1]);

            let mut success = false;
            for folder in get_blockchain().folders {
                for vft in folder.vfts {
                    if vft.id == user_query[1] {
                        success = true;
                        println!("\n-------------------------------------------------------------");
                        println!("RootFolder:   \"{}\" \nDisplayName:  \"{}\" \nID:           \"{}\" \nImagePath:    \"{}\" \nCryptoCost:   \"{}\"", folder.name, vft.display_name, vft.id, vft.image_path, vft.crypto_cost);
                    }
                }
            }

            if success == false {
                println!("VFT Query failed")
            } else {
                println!("-------------------------------------------------------------\n");
                println!("Query successful: VFT/{}", user_query[1]);
            }
        }
        "all" => {
            println!("Querying all Blockchain data");
            println!("-------------------------------------------------------------");
            println!("{:#?}", get_blockchain());
            println!("-------------------------------------------------------------");
            println!("Query successful");
        }
        _ => println!("Invalid query")
    }
}

fn main() {
    //println!("Printing entire Blockchain for debugging purposes:\n{:#?}", get_blockchain())

    loop {
        query_data()
    }
}