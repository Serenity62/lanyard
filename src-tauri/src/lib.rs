use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write, Result};
use std::path::Path;

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
}

impl Settings {
    pub fn builder(filename: String) -> Settings {
        let s = match read(&filename) {
            Err(why) => panic!("Could not read file: {}", why), 
            Ok(s) => {
                if s.is_empty() {
                    let str = String::from("{\n\r\"profiles\": []\n\r}");
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

    pub fn create_profile(&mut self, p: &Profile) -> Result<()> {
        Ok(())
    }
    pub fn delete_profile(&mut self, id:i32) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub location: String,
}

impl Profile {
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
        Ok(())
    }

    pub fn delete_account(&self, id: i32) -> Result<()> {
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
    pub id: i32,
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
