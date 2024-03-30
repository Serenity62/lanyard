// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_lib;
//use app::*;
use crate::app_lib::account::*;
use crate::app_lib::setting::*;

use std::sync::Mutex;
//use serde::{Serialize, Deserialize};

struct Note(Mutex<Settings>);

fn main() {
    let settings = Settings::builder(String::from("settings.json"));
    tauri::Builder::default()
        .manage(Note(settings.into()))
        .invoke_handler(tauri::generate_handler![get_profile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_profile () -> ProfileAccounts {
    let p = Profile {
        id: 1,
        name: String::from("test"),
        location: String::from("test.json")
    };

    let accounts = match p.get_accounts() {
        Err(why) => panic!("Error when getting accounts: {}", why),
        Ok(accounts) => accounts,
    };
    accounts
}

/*
#[tauri::command]
fn get_profiles(state: tauri::State<Note>) -> Vec<Profile> {
    let settings = state.0.lock().unwrap();
    let profs = settings.profiles.clone();
    profs
}
*/
/*
#[tauri::command]
fn new_profile(state: tauri::State<Note>, p: Profile) -> Result<(), String> {
    let mut settings = state.0.lock().unwrap();
    settings.profiles.push(p);
    Ok(())
}
*/
