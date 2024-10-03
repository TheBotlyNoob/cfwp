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
pub struct GetAccount200Response {
    #[serde(rename = "account_type")]
    pub account_type: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "license")]
    pub license: String,
    #[serde(rename = "premium_data")]
    pub premium_data: f64,
    #[serde(rename = "quota")]
    pub quota: f64,
    #[serde(rename = "referral_count")]
    pub referral_count: f64,
    #[serde(rename = "referral_renewal_countdown")]
    pub referral_renewal_countdown: f64,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "updated")]
    pub updated: String,
    #[serde(rename = "warp_plus")]
    pub warp_plus: bool,
}

impl GetAccount200Response {
    pub fn new(account_type: String, created: String, id: String, license: String, premium_data: f64, quota: f64, referral_count: f64, referral_renewal_countdown: f64, role: String, updated: String, warp_plus: bool) -> GetAccount200Response {
        GetAccount200Response {
            account_type,
            created,
            id,
            license,
            premium_data,
            quota,
            referral_count,
            referral_renewal_countdown,
            role,
            updated,
            warp_plus,
        }
    }
}

