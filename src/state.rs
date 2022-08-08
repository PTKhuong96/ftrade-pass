use cosmwasm_std::Storage;
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static STORE_KEY: &[u8] = b"store";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pass {
    pub id: String,
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

pub fn store(storage: &mut dyn Storage) -> Bucket<Pass> {
    bucket(storage, STORE_KEY)
}

pub fn store_query(storage: &dyn Storage) -> ReadonlyBucket<Pass> {
    bucket_read(storage, STORE_KEY)
}
