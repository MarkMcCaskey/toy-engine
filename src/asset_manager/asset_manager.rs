//! Manages all content in the game
use mesh::*;
use texture::*;
use scripting::*;
use asset_manager::text::*;
use find_folder;

pub struct AssetManager {
    asset_packs: Vec<AssetPack>,
}

pub enum AssetPack {
    AssetFolder { contents: Vec<AssetPack> },
    Asset { contents: Asset },
}

pub struct Asset {
    id: u64,
    content_ref: Box<AssetContent>,
}

pub enum AssetContent {
    String(String),
    Mesh(Mesh),
    Texture(Texture),
    Script(Script),
}

impl AssetPack {
    /// brute force asset search
    /// to be updated later
    pub fn get_asset(&self, id: u64) -> Option<&Asset> {
        match *self {
            AssetPack::AssetFolder { contents: ref content } => {
                content.iter().filter_map(|a| a.get_asset(id)).next()
            }
            AssetPack::Asset { contents: ref asset } => {
                if asset.id == id { Some(asset) } else { None }
            }
        }
    }
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager { asset_packs: vec![] }
    }

    /// Currently brute force asset search
    /// To be improved later
    pub fn get_asset(&self, id: u64) -> Option<&Asset> {
        self.asset_packs
            .iter()
            .filter_map(|ap| ap.get_asset(id))
            .next()
    }

    /// loads all assets
    pub fn load_all(&mut self) {}

    pub fn lookup_text(&self, text_id: Text) -> &str {
        "hello"
    }

    /// loads the text for the given language
    pub fn load_language(&mut self, language: &str) {}
}
