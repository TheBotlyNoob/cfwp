/*
 * untitled API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 536
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBoundDevices200Response {
    #[serde(rename = "activated")]
    pub activated: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl GetBoundDevices200Response {
    pub fn new(activated: String, active: bool, created: String, id: String, model: String, role: String, r#type: String) -> GetBoundDevices200Response {
        GetBoundDevices200Response {
            activated,
            active,
            created,
            id,
            model,
            name: None,
            role,
            r#type,
        }
    }
}

