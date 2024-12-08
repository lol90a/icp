use ic_cdk_macros::{init, update, query};
use ic_cdk::println;
use candid::CandidType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, CandidType, Deserialize)] // Added required traits
struct Asset {
    id: u64,
    name: String,
    price: u64,
    seller: String,
}

type Assets = HashMap<u64, Asset>;

static mut ASSETS: Option<Assets> = None;

#[init]
fn init() {
    println!("Marketplace initialized!");
    unsafe {
        ASSETS = Some(HashMap::new());
    }
}

#[update]
fn list_asset(name: String, price: u64, seller: String) -> String {
    let asset = Asset {
        id: generate_id(),
        name,
        price,
        seller,
    };

    unsafe {
        let assets = ASSETS.as_mut().unwrap();
        assets.insert(asset.id, asset.clone());
    }

    println!("Asset listed: {:?}", asset);
    format!("Asset {} listed successfully!", asset.id)
}

#[query]
fn get_assets() -> Vec<Asset> {
    unsafe {
        ASSETS
            .as_ref()
            .unwrap()
            .values()
            .cloned() // Clone each Asset
            .collect::<Vec<Asset>>() // Collect into a Vec<Asset>
    }
}

// Helper function to generate unique IDs
fn generate_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_millis() as u64
}
