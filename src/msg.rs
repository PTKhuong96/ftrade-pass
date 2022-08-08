use crate::state::Pass;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub image_url: String,
    pub desc: String,
    pub quantity_issued: i32,
    pub expired_date: String,
    pub duration: i32,
    pub nft_type: String,
    pub career: String,
    pub course_type: String,
    pub creator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddNew {
        id: String,
        name: String,
        image_url: String,
        desc: String,
        quantity_issued: i32,
        expired_date: String,
        duration: i32,
        nft_type: String,
        career: String,
        course_type: String,
        creator: String,
        owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetPass returns the pass's information
    GetPass { id: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PassInfoResponse {
    pub pass: Option<Pass>,
}
