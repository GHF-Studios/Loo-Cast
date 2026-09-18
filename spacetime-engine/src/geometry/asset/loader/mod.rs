//! Bevy asset-loader adapter for `.spacemap` authored maps.

use std::io;

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    reflect::TypePath,
};

use super::AuthoredMap;

#[derive(Default, TypePath)]
pub struct AuthoredMapLoader;

impl AssetLoader for AuthoredMapLoader {
    type Asset = AuthoredMap;
    type Settings = ();
    type Error = io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let map: AuthoredMap = ron::de::from_bytes(&bytes)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;

        map.validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

        Ok(map)
    }

    fn extensions(&self) -> &[&str] {
        &["spacemap"]
    }
}
