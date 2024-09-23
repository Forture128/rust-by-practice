// src/db.rs

use std::collections::HashMap;
use crate::models::UserProfile;

pub struct Database {
    user_profiles: HashMap<String, UserProfile>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            user_profiles: HashMap::new(),
        }
    }

    pub fn insert_user_profile(&mut self, user: UserProfile) {
        self.user_profiles.insert(user.user_id.clone(), user);
    }

    pub fn get_user_profile(&self, user_id: &str) -> Option<&UserProfile> {
        self.user_profiles.get(user_id)
    }
}
