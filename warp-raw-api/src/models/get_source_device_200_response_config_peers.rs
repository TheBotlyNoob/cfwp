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
pub struct GetSourceDevice200ResponseConfigPeers {
    #[serde(rename = "endpoint")]
    pub endpoint: Box<models::GetSourceDevice200ResponseConfigEndpoint>,
    #[serde(rename = "public_key")]
    pub public_key: String,
}

impl GetSourceDevice200ResponseConfigPeers {
    pub fn new(endpoint: models::GetSourceDevice200ResponseConfigEndpoint, public_key: String) -> GetSourceDevice200ResponseConfigPeers {
        GetSourceDevice200ResponseConfigPeers {
            endpoint: Box::new(endpoint),
            public_key,
        }
    }
}

