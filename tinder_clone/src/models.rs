// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub name: String,
    // Add more fields as needed (e.g., age, bio, photos, preferences, etc.).
}

// Define additional data models as required.
