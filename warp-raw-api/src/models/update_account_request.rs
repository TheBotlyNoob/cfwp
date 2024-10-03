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
pub struct UpdateAccountRequest {
    #[serde(rename = "license")]
    pub license: String,
}

impl UpdateAccountRequest {
    pub fn new(license: String) -> UpdateAccountRequest {
        UpdateAccountRequest {
            license,
        }
    }
}

