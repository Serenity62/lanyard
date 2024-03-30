use serde::{Serialize, Deserialize};
use std::io::Result;
//use rand::prelude::*;

//mod lanyard_utils;
//mod profile;
use crate::app_lib::utils::{read, write};
use crate::app_lib::account::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub profiles: Vec<Profile>,
    pub location: String,
}

impl Settings {
    pub fn builder(filename: String) -> Settings {
        let s = match read(&filename) {
            Err(why) => panic!("Could not read file: {}", why), 
            Ok(s) => {
                if s.is_empty() {
                    let new_setting = Settings {
                        profiles: Vec::new(),
                        location: filename.to_string(),
                    };
                    let str = serde_json::to_string(&new_setting).unwrap();
                    match write(&filename, &str){
                        Err(why) => panic!("Could not create file: {}", why),
                        Ok(_) => (),
                    };
                    read(&filename).unwrap()
                } else {
                    s
                }
            },
        };

        let settings: Settings = match serde_json::from_str(&s){
            Err(why) => panic!("Issue processing settings: {}", why),
            Ok(settings) => settings,
        };
        settings
    }

    pub fn get_profile(location: &String) -> Settings {
        let str_settings = read(&location).unwrap();
        let settings: Settings = match serde_json::from_str(&str_settings){
            Err(why) => panic!("Could not crate file: {}", why),
            Ok(settings) => settings,
        };
        settings
    }

    pub fn add_profile(&mut self, p: Profile) -> Result<()> {
        let str_settings = read(&self.location).unwrap();
        let mut settings: Settings = serde_json::from_str(&str_settings)?;
        settings.profiles.push(p);
        //self.profiles.push(p);
        let str: String = serde_json::to_string(&settings)?;
        let _ = write(&self.location, &str)?;
        self.profiles = settings.profiles;
        Ok(())
    }

    pub fn update_profile(&mut self, p: Profile) -> Result<()> {
        let str_settings = read(&self.location).unwrap();
        let mut settings: Settings = serde_json::from_str(&str_settings)?;

        for i in 0..settings.profiles.len() {
            if p.id == settings.profiles[i].id {
                settings.profiles[i] = p;
                break;
            }
        }
        let str: String = serde_json::to_string(&settings)?;
        let _ = write(&self.location, &str);
        Ok(())
    }
    
    pub fn delete_profile(&mut self, id:u32) -> Result<()> {
        let str_settings = read(&self.location).unwrap();
        let mut settings: Settings = serde_json::from_str(&str_settings)?;
        
        let mut idx = settings.profiles.len() + 1;
        for i in 0..settings.profiles.len() {
            if id == settings.profiles[i].id {
                idx = i;
            }
        }
        if idx != settings.profiles.len() + 1 {
            let _ = settings.profiles.remove(idx);
        }

        let str: String = serde_json::to_string(&settings)?;
        let _ = write(&self.location, &str)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod settings {
    use super::*;

    #[test]
    fn builder() {
        let settings = Settings::builder(String::from("tests/files/settings.json"));
        assert_eq!(0, settings.profiles.len());
    }

    #[test]
    fn builder_create() {
        let path = String::from("tests/files/settings_new.json");
        let _ = std::fs::remove_file(&path);

        let settings = Settings::builder(path);
        assert_eq!(0, settings.profiles.len());
    }

    /*
    #[test]
    fn update() {}
    #[test]
    fn create() {}
    #[test]
    fn delete() {}
    */
}

