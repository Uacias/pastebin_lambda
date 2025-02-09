use crate::models::Note;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

pub static NOTES: Lazy<Mutex<HashMap<String, Note>>> = Lazy::new(|| Mutex::new(HashMap::new()));
