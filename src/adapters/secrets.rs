use std::str::FromStr;

use rusoto_core::Region;
use rusoto_secretsmanager::{
    SecretsManagerClient,
    SecretsManager,
    GetSecretValueRequest
};

pub async fn get_secret(
    region: String,
    id: String,
    version_id: Option<String>,
    version_stage: Option<String>
) -> Option<String> {
    let region = Region::from_str(&region).unwrap();
    let secrets = SecretsManagerClient::new(region);
    secrets.get_secret_value(GetSecretValueRequest {
        secret_id: id,
        version_id,
        version_stage,
    }).await.unwrap().secret_string
}