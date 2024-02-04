// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::*;
use serde::{Serialize, Deserialize};

fn main() {

    tauri::Builder::default()
        .manage(app::Settings::builder(String::from("settings.json")).into())
        .invoke_handler(tauri::generate_handerl![get_profile, get_profiles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_profile () -> ProfileAccounts {
    let p = app::Profile {
        id: 1,
        name: String::from("test"),
        location: String::from("test.json")
    };

    let as = match p.get_accounts() {
        Err(why) => panic!("Error when getting accounts: {}", why),
        Ok(as) => as,
    };
    as
}

#[tauri::command]
fn get_profiles(settings: tauri::State<Settings>) -> Vec<Profile> {
    settings.profiles
}


[tauri::command]
fn new_profile(mut settings: tauri::State<Settings>, p: Profile) -> Result<()> {
    settings.profiles.push(p);
    Ok(())
}
