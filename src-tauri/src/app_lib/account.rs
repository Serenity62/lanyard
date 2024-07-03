use serde::{Serialize, Deserialize};
use std::io::Result;
use rand::prelude::*;

//mod utils;
use crate::app_lib::utils::{read, write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub id: u32,
    pub name: String,
    pub location: String,
}

impl Profile {
    pub fn builder(name: &String, location: &String) -> Profile {
        let mut rnd = rand::thread_rng();
        let pa = ProfileAccounts::builder();
        let str: String = serde_json::to_string(&pa).unwrap();

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

    pub fn update_account(&self, a: Account) -> Result<()> {
        let mut accounts = self.get_accounts()?;
        for i in 0..accounts.accounts.len() {
            if a.id ==accounts.accounts[i].id {
                accounts.accounts[i] = a;
                break;
            }
        }
        let s = serde_json::to_string(&accounts)?;
        write(&self.location, &s)?;
        Ok(())
    }

    pub fn delete_account(&self, id: u32) -> Result<()> {
        let mut accounts = self.get_accounts()?;
        let mut idx = accounts.accounts.len() +1;
        for i in 0..accounts.accounts.len() {
            if id == accounts.accounts[i].id {
                idx = i;
            }
        }
        if idx != accounts.accounts.len() + 1 {
            accounts.accounts.remove(idx);
        }
        let s = serde_json::to_string(&accounts)?;
        write(&self.location, &s)?;
        Ok(())
    }
    pub fn delete_all(&self) -> Result<()> {
        let ap = ProfileAccounts::builder();
        let s = serde_json::to_string(&ap)?;
        write(&self.location, &s)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileAccounts {
    pub accounts: Vec<Account>,
}

impl ProfileAccounts {
    pub fn builder() -> ProfileAccounts {
        ProfileAccounts {
            accounts: Vec::new(),
        }
    }
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
mod profile {
    use super::*;

    #[test]
    fn builder() {
        let name = String::from("Test Profile");
        let location = String::from("tests/files/test_profile");
        let str = String::from("{\"accounts\":[]}");

        let _ = std::fs::remove_file(&location);
        let p = Profile::builder(&name, &location);
        let file = read(&location).unwrap();

        assert_eq!(name, p.name);
        assert_eq!(location, p.location);
        assert_ne!(0, p.id);
        assert_eq!(str, file);
    }
    
    #[test]
    fn get() {
        
    }
    /*
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
