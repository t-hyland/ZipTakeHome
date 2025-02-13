// Thomas Hyland Backend Take Home Assignment Submission
// This file is new. I wrote the bulk of the assignment in this file including 
//  the functions for the assigned POST route and GET route. 
use axum::{
    extract::Json,
    response::{IntoResponse, Json as ResponseJson},
    http::StatusCode,
};
use serde::{
    Serialize, 
    Deserialize,
};
use serde_json; 
use reqwest; 
use dotenv::dotenv;
use std::env; 

//Part 1: 
// Credentials struct for username, password, and url 
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String, 
    pub url: String, 

}

//Takes Credentials struct as input and returns same Credentials struct to client. 
//  POST call as detailed in part 1 of instructions and referenced from line 12 of main.rs. 
pub async fn credentials(Json(payload): Json<Credentials>) -> ResponseJson<Credentials>{
    ResponseJson(payload)
}


//Part 2: 

//Struct for response from token request
#[derive(Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires: String,
}

//Struct for response from computer-inventory request
#[derive(Deserialize)]
pub struct InventoryResponse {
    pub totalCount: u32,
    pub results: Vec<Computer>, 
}

#[derive(Deserialize)]
pub struct Computer {
    pub id: String,
    pub general: General,
    pub hardware: Hardware, 
    pub operatingSystem: OperatingSystem
}

#[derive(Deserialize)]
pub struct General {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Hardware {
    pub model: String,
}

#[derive(Deserialize)]
pub struct OperatingSystem {
    pub name: String,
    pub version: String, 
    pub build: String, 
}

//Struct for response from the the available updates request
#[derive(Deserialize)]
pub struct Updates {
    pub availableUpdates: AvailableUpdates,
}

#[derive(Deserialize)]
pub struct AvailableUpdates {
    pub macOS: Vec<String>,
    pub iOS: Vec<String>,
}

// Struct for final delivery of devices to user
#[derive(Serialize, Deserialize)]
pub struct FinalDevices {
    pub devices: Vec<FinalDevice>,
}

#[derive(Serialize, Deserialize)]
pub struct FinalDevice {
    pub id: u64,
    pub name: String,
    pub model: String, 
    pub os: String, 
    pub os_is_latest: bool, 
}

//Returns a list of devices from the jamf API
pub async fn devices() -> impl IntoResponse{
    // Loads USERNAME and PASSWORD from .env file 
    dotenv().ok();
    let username = env::var("USERNAME").expect("USERNAME not set");;
    let password = env::var("PASSWORD").expect("PASSWORD not set");;
    
    // URLs for API calls
    let auth_url = "https://zipziptest.jamfcloud.com/api/v1/auth/token"; 
    let inv_url = "https://zipziptest.jamfcloud.com/api/v1/computers-inventory?section=GENERAL&section=HARDWARE&section=OPERATING_SYSTEM"; 
    let updates_url = "https://zipziptest.jamfcloud.com/api/v1/managed-software-updates/available-updates";


    let client = reqwest::Client::new();
    
    // Requests bearer token from Jamf using username and password
    let res = match client.post(auth_url)
        .basic_auth(username, Some(password))
        .send()
        .await 
        {
            Ok(res) => res, 
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to make token request").into_response(),
        };

    // Parses Auth Response to get the bearer token for the rest of the API calls
    let auth_response: AuthResponse = match res.json().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(), 
    }; 

    let bearer_token = auth_response.token; 

    // Requests computer inventory from Jamf
    let res_inv = match client.get(inv_url)
        .header("AUTHORIZATION", format!("Bearer {}", bearer_token))
        .send()
        .await {
            Ok(res_inv) => res_inv,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to make inventory request").into_response(), 
        };
    
    // Parses res_inv to JSON format
    let inventory_response: InventoryResponse = match res_inv.json().await {
        Ok(body) => body, 
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read inventory body").into_response(), 
    }; 

    // Requests available updates from Jamf
    let res_updates = match client.get(updates_url)
        .header("AUTHORIZATION", format!("Bearer {}", bearer_token))
        .send()
        .await {
            Ok(res_updates) => res_updates,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to make updates request").into_response(), 
        }; 

    // Parses res_updates to a JSON format
    let updates_response: Updates = match res_updates.json().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read updates body").into_response(), 
    };

    // Getting latest os update for both iOS and macOS
    let latest_mac_os = updates_response.availableUpdates.macOS[0].clone();
    let latest_ios = updates_response.availableUpdates.iOS[0].clone();

    // list of devices in FinalDevice form
    let mut devices: Vec<FinalDevice> = Vec::new(); 

    for device in inventory_response.results {
        // Changing from string type to u64
        let id = device.id.parse::<u64>().unwrap();
        let name = device.general.name; 
        let model = device.hardware.model; 
        let os = device.operatingSystem.name; 
        let version = device.operatingSystem.version;
        let mut os_is_latest = true; 
        // Checking if os is latest by comparing to latest available update 
        if os == "macOS"{
            if version != latest_mac_os {
                os_is_latest = false; 
            }
        } else {
            if version != latest_ios {
                os_is_latest = false; 
            }
        }

        // Creating the device with only the information needed 
        let device = FinalDevice {
            id: id,
            name: name, 
            model: model, 
            os: os,
            os_is_latest: os_is_latest,
        };

        // Adding device to list of FinalDevices
        devices.push(device);
    } 

    // Creating final FinalDevices struct 
    let final_devices = FinalDevices{
        devices: devices, 
    };
    
    // Transforming FinalDevices to string
    let result = serde_json::to_string_pretty(&final_devices).unwrap();

    // Returning FinalDevices
    result.into_response()
}