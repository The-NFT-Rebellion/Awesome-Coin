use std::io;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct VFT {
    display_name: String,
    id: String,
    file_path: String,
    crypto_cost: f32
}

impl Default for VFT {
    fn default() -> VFT {
        VFT {
            display_name: String::from("Boring default name"),
            id: String::from("default_id"),
            file_path: String::from("boring_default_image_path.png"),
            crypto_cost: 100.1
        }
    }
}

#[derive(Debug)]
struct Drawer { // TODO: Think of better name for this
    name: String,
    vfts: Vec<VFT>
}

#[derive(Debug)]
struct Cubelink { // TODO: Think of better name for this
    drawers: Vec<Drawer>
}

fn get_cubelink() -> Cubelink {
    // TODO: Change to Cubelink new name and Drawer new name
    let cubelink : Cubelink = Cubelink {
        drawers: vec![
            Drawer {
                name: String::from("images"),
                vfts: vec![
                    VFT {
                        display_name: String::from("Shrek my beloved"),
                        id: String::from("shrek"),
                        file_path: String::from("shrek.jpg"),
                        crypto_cost: 20000.
                    },
                    VFT {
                        display_name: String::from("Kirbo"),
                        id: String::from("kirbo"),
                        file_path: String::from("kirbo.png"),
                        crypto_cost: 999999999.
                    },
                    VFT {
                        display_name: String::from("Ordinary Door"),
                        id: String::from("door"),
                        file_path: String::from("door.jpg"),
                        crypto_cost: 50.
                    },
                    VFT {
                        display_name: String::from("Living Door"),
                        id: String::from("alive_door"),
                        file_path: String::from("alive_door.jpg"),
                        crypto_cost: 5000.
                    }
                ]
            }
        ]
    };

    return cubelink
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
        "drawer" => {
            println!("Querying Drawer: {}", user_query[1]);
            
            let mut success = false;
            for drawer in get_cubelink().drawers {
                if drawer.name == user_query[1] {
                    success = true;
                    println!("\n-------------------------------------------------------------");
                    println!("{:#?}", drawer);
                }
            }

            if success == false {
                println!("Drawer Query failed")
            } else {
                println!("-------------------------------------------------------------\n");
                println!("Query successful: Drawer/{}", user_query[1]);
            }
        }
        "vft" => {
            println!("Querying VFT: {}", user_query[1]);

            let mut success = false;
            for drawer in get_cubelink().drawers {
                for vft in drawer.vfts {
                    if vft.id == user_query[1] {
                        success = true;
                        println!("\n-------------------------------------------------------------");
                        println!("RootDrawer:   \"{}\" \nDisplayName:  \"{}\" \nID:           \"{}\" \nFilePath:     \"{}\" \nCryptoCost:   \"{}\"", drawer.name, vft.display_name, vft.id, vft.file_path, vft.crypto_cost);
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
            println!("Querying all Cubelink data");
            println!("-------------------------------------------------------------");
            println!("{:#?}", get_cubelink());
            println!("-------------------------------------------------------------");
            println!("Query successful");
        }
        "export" => {
            let _dir = std::fs::create_dir("./vfts");

            for drawer in get_cubelink().drawers {
                for vft in drawer.vfts {
                    println!("Exporting VFT \"{}\" from Drawer \"{}\"", vft.display_name, drawer.name);

                    let write_data = format!("display_name:{},id:{},drawer:{},file_path:{},crypto_cost:{}", vft.display_name, vft.id, drawer.name, vft.file_path, vft.crypto_cost);
                    let mut file = File::create(format!("./vfts/{}.vft", vft.id)).expect("Unable to create file");
                    file.write_all(write_data.as_bytes()).expect("Unable to write data");
                }
            }
        }
        _ => println!("Invalid query")
    }
}

fn main() {
    //println!("Printing entire Cubelink for debugging purposes:\n{:#?}", get_Cubelink())

    loop {
        query_data()
    }
}