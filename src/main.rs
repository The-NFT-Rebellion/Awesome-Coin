#[derive(Debug)]
struct VFT {
    name: String,
    image_path: String,
    crypto_cost: f32
}

impl Default for VFT {
    fn default() -> VFT {
        VFT {
            name: String::from("Boring default name"),
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

fn main() {
    // TODO: Change to Blockchain new name
    let blockchain : Blockchain = Blockchain {
        folders: vec![
            Folder {
                name: String::from("wa"),
                vfts: vec![
                    VFT {
                        name: String::from("Test VFT"),
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
    
    //println!("Printing entire Blockchain for debugging purposes:\n{:#?}", blockchain)

    
}