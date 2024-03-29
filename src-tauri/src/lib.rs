use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;
use rand::prelude::*;

fn read(p: &String) -> Result<String> {
    // add sanitization to the path argument
    let path = Path::new(&p);
    let mut file = File::options().read(true).write(true)
        .create(true).open(&path)?;
        
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn write(p: &String, s: &String) -> Result<()> {
    // add sanitization to the path argument
    let path = Path::new(&p);
    let mut file = File::options().write(true).create(true).open(&path)?;

    file.write_all(s.as_bytes())?;
    Ok(())
}

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
                    let str = serde_json::to_string(&new_setting);
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

    pub fn add_profile(&mut self, p: Profile) -> Result<()> {
        let str_settings = read(&self.location).unwrap();
        let mut settings: Settings = serde_json::from_str(&str_settings)?;
        settings.profiles.push(p);
        self.profiles.push(p);
        let str: String = serde_json::to_string(&settings)?;
        let _ = write(&self.location, &str)?;
        Ok(())
    }
    
    pub fn delete_profile(&mut self, id:u32) -> Result<()> {
        println!("{}", id);
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub location: String,
}

impl Profile {
    pub fn builder(name: &String, location: &String) -> Profile {
        let mut rnd = rand::thread_rng();
        let str = String::from("{\n\r\"ProfileAccounts\": []\n\r}");

        match read(&location) {
            Err(why) => panic!("Issue with file location: {}", why), 
            Ok(s) => {
                if s.is_empty() {
                    match write(&location, &str){
                        Err(why) => panic!("Could not create file: {}", why),
                        Ok(_) => (),
                    };
                } else {
                   panic!("File already exists"); 
                }
            },
        };

        Profile {
            id: rnd.gen_range(0..=10),
            name: name.to_string(),
            location: location.to_string(),
        }
    }

    pub fn update_location(&mut self, location: &String) {
        let old_loc = self.location.to_string();

        let f = read(&old_loc).unwrap();

        match read(&location) {
            Err(why) => panic!("Issue with file location: {}", why), 
            Ok(s) => {
                if s.is_empty() {
                    match write(&location, &f){
                        Err(why) => panic!("Could not create file: {}", why),
                        Ok(_) =>  {
                            let _ = std::fs::remove_file(&old_loc); 
                            ()
                        }, 
                    };
                } else {
                   panic!("File already exists"); 
                }
            },
        };

        self.location = location.to_string();
    }

    pub fn get_accounts(&self) -> Result<ProfileAccounts> {
        let s = read(&self.location)?;
        let accounts: ProfileAccounts = serde_json::from_str(&s)?;
        Ok(accounts)
    }

    pub fn add_account(&self, a: Account) -> Result<()> {
        let mut accounts = self.get_accounts()?;
        accounts.accounts.push(a);
        let s = serde_json::to_string(&accounts)?;
        write(&self.location, &s)?;
        Ok(())
    }

    pub fn update_account(&self, a: &Account) -> Result<()> {
        println!("{},", a.id);
        Ok(())
    }

    pub fn delete_account(&self, id: u32) -> Result<()> {
        println!("{}", id);
        Ok(())
    }
    pub fn delete_all(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileAccounts {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountType;
impl AccountType {
    pub const BASIC: &'static str = "BASIC";
    //pub const ACCOUNT: &'static str = "ACCOUNT";
    //pub const SYSTEM: &'static str = "SYSTEM";
    //pub const SECRET: &'static str = "SECRET";
    //pub const RECOVERY: &'static str = "RECOVERY";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicAccount {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub r#type: AccountType,
    pub account: String,
}

#[cfg(test)]
mod io {
    use std::fs::File;
    use super::*;

    #[test]
    fn file_write() {
        let path = String::from("tests/files/test.txt");
        let _ = std::fs::remove_file(&path); 
        let test = String::from("this is a test");
        write(&path, &test).unwrap();

        // Read in file to chek.
        let mut file = match File::open(&path){
            Err(why) => panic!("could not write to file: {}", why),
            Ok(file) => file,
        };
        let mut s = String::new();
        match file.read_to_string(&mut s){
            Err(why) => panic!("could not read file: {}", why),
            Ok(_) => assert_eq!(s, test), 
        };
    }

    #[test]
    fn file_read() {
        let path = String::from("tests/files/test2.txt");
        let test = String::from("this is a test");
        let file = read(&path).unwrap();
        assert_eq!(file, test);
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
    fn create() {}
    #[test]
    fn delete() {}
    */
}

#[cfg(test)]
mod profile {
    use super::*;

    #[test]
    fn builder() {
        let name = String::from("Test Profile");
        let location = String::from("tests/files/test_profile");
        let str = String::from("{\n\r\"ProfileAccounts\": []\n\r}");

        let _ = std::fs::remove_file(&location);
        let p = Profile::builder(&name, &location);
        let file = read(&location).unwrap();

        assert_eq!(name, p.name);
        assert_eq!(location, p.location);
        assert_ne!(0, p.id);
        assert_eq!(str, file);
    }
    /*
    #[test]
    fn get() {}
    #[test]
    fn add() {}
    #[test]
    fn update() {}
    #[test]
    fn delete() {}
    #[test]
    fn delete_all() {}
    */
}
