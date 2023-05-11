mod errors;
mod models;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Receiver};

use may::go;
use serde_json::json;
use reqwest::blocking::Client;
use rand::{Rng, distributions::Alphanumeric};

use crate::models::*;
use crate::errors::*;


pub fn get_domains() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let json_data: serde_json::Value = reqwest::blocking::get("https://api.mail.tm/domains")?
        .json()?;

    let mut domains = vec![];

    if let Some(members) = json_data["hydra:member"].as_array() {
        for member in members {
            if let Some(domain) = member["domain"].as_str() {
                domains.push(domain.to_string());
            }
        }
    }
    return Ok(domains);
}

pub struct Account {
    address: String,
    password: String,
    token: String,
    id: String,
    client: reqwest::blocking::Client,
}

impl Account {
    pub fn address(&self) -> &String { return &self.address; }
    pub fn password(&self) -> &String { return &self.password; }

    pub fn create(address: &str, password: &str) -> Result<Account, Box<dyn std::error::Error>> {
        let client = Client::new();

        let data = json!({
            "address": address,
            "password": password,
        });

        let response_data: serde_json::Value = client.post("https://api.mail.tm/accounts")
            .json(&data)
            .send()?
            .error_for_status()?
            .json()?;

        let id = response_data["id"].as_str().unwrap().to_string();
        let created_at = response_data["createdAt"].as_str().unwrap();
        let updated_at = response_data["updatedAt"].as_str().unwrap();

        let data = json!({"@context":"/contexts/Account","@id":format!("/accounts/{}", id),"@type":"Account","id":id,"address":address,"quota":40000000,"used":0,"isDisabled":false,"isDeleted":false,"createdAt":created_at,"updatedAt":updated_at,"password":password});
        let response_data: serde_json::Value = client.post("https://api.mail.tm/token")
            .json(&data)
            .send()?
            .error_for_status()?
            .json()?;

        let address = address.to_string();
        let password = password.to_string();
        let token = response_data["token"].as_str().unwrap().to_string();

        return Ok(Account {
            address,
            password,
            token,
            id,
            client
        });

    }

    pub fn create_random() -> Result<Account, Box<dyn std::error::Error>> {

        let client = Client::new();
        let mut rng = rand::thread_rng();

        let domains = get_domains()?;
        if domains.is_empty() {
            return Err(Box::new(NoDomainsError));
        }

        let address_name = || -> String {
            rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect()
        }();

        let password = || -> String {
            rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
        }();

        
        let index: usize = rng.gen_range(0..=domains.len()-1);
        let address = format!("{}@{}", address_name, domains[index]);

        let data = json!({
            "address": address,
            "password": password,
        });

        let response_data: serde_json::Value = client.post("https://api.mail.tm/accounts")
            .json(&data)
            .send()?
            .error_for_status()?
            .json()?;

        let id = response_data["id"].as_str().unwrap().to_string();
        let created_at = response_data["createdAt"].as_str().unwrap();
        let updated_at = response_data["updatedAt"].as_str().unwrap();

        let data = json!({"@context":"/contexts/Account","@id":format!("/accounts/{}", id),"@type":"Account","id":id,"address":address,"quota":40000000,"used":0,"isDisabled":false,"isDeleted":false,"createdAt":created_at,"updatedAt":updated_at,"password":password});
        let response_data: serde_json::Value = client.post("https://api.mail.tm/token")
            .json(&data)
            .send()?
            .error_for_status()?
            .json()?;

        let address = address.to_string();
        let password = password.to_string();
        let token = response_data["token"].as_str().unwrap().to_string();

        return Ok(Account {
            address,
            password,
            token,
            id,
            client
        });
    }

    pub fn get_messages(&self) -> Result<Vec<PreviewMessage>, Box<dyn std::error::Error>> {

        let json_object: serde_json::Value = self.client.get("https://api.mail.tm/messages")
            .header("Authorization", &self.token)
            .send()?
            .error_for_status()?
            .json()?;

        let messages: Vec<PreviewMessage> = serde_json::from_str(&json_object["hydra_member"].to_string())
            .unwrap_or(vec![]);


        return Ok(messages);
    }

    pub fn rx_messages(&self) -> Receiver<(Option<PreviewMessage>, Box<dyn std::error::Error + Send + Sync>)> {
        let (tx, rx) = mpsc::channel::<(Option<PreviewMessage>, Box<dyn std::error::Error + Send + Sync>)>();
        let account = Account {
            address: self.address.clone(),
            password: self.password.clone(),
            token: self.token.clone(),
            id: self.id.clone(),
            client: self.client.clone()
        };
        go!(move || {
            let mut received: Vec<PreviewMessage> = vec![];
            'main: loop {
                let msgs = match account.get_messages() {
                    Ok(msgs) => msgs,
                    Err(_) => {
                        if tx.send((None, Box::new(MessageRecvError))).is_err() {
                            break 'main;
                        };
                        vec![]
                    }
                };

                #[allow(unused_must_use)]
                if !msgs.is_empty() {
                    for msg in msgs.iter() {
                        if !received.contains(msg) {
                            received.push(msg.clone());
                            
                            if tx.send((Some(msg.clone()), Box::new(Nil))).is_err() {
                                break 'main;
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_secs(5));
            }
        });
        
        return rx;
    }

    pub fn get_message(&self, preview_msg: &PreviewMessage) -> Result<Message, Box<dyn std::error::Error>> {
        return Ok(self.client.get(format!("https://api.mail.tm/messages/{}", preview_msg.id))
            .header("Authorization", &self.token)
            .send()?
            .error_for_status()?
            .json()?);
    }

    pub fn delete(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.delete(format!("https://api.mail.tm/accounts/{}", self.id))
            .header("Authorization", &self.token)
            .send()?
            .error_for_status()?;

        return Ok(());
    }
}

impl Drop for Account {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.delete();
    }
}
