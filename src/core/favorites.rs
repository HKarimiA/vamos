use leptos::prelude::*;
use std::collections::HashSet;

/// Global context for managing favorites across the app
#[derive(Clone, Copy)]
pub struct FavoritesContext {
    pub favorites: RwSignal<HashSet<(u32, u32)>>, // (stage, card_id)
}

impl FavoritesContext {
    pub fn new() -> Self {
        Self {
            favorites: RwSignal::new(HashSet::new()),
        }
    }

    pub fn toggle(&self, stage: u32, card_id: u32) {
        self.favorites.update(|favs| {
            let key = (stage, card_id);
            if favs.contains(&key) {
                favs.remove(&key);
            } else {
                favs.insert(key);
            }
        });
    }

    pub fn is_favorite(&self, stage: u32, card_id: u32) -> bool {
        self.favorites.read().contains(&(stage, card_id))
    }

    pub fn get_all(&self) -> Vec<(u32, u32)> {
        self.favorites.read().iter().copied().collect()
    }

    pub fn remove(&self, stage: u32, card_id: u32) {
        self.favorites.update(|favs| {
            favs.remove(&(stage, card_id));
        });
    }

    pub fn count(&self) -> usize {
        self.favorites.read().len()
    }
}
