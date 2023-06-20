use crate::types::utils::serialize_datetime_as_rfc3339;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VoterGroupId(pub String);

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct VoterInfo {
    pub voting_power: i64,
    pub voting_group: VoterGroupId,
    pub delegations_power: i64,
    pub delegations_count: i64,
    pub voting_power_saturation: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegator_addresses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Voter {
    pub voter_info: VoterInfo,
    #[serde(serialize_with = "serialize_datetime_as_rfc3339")]
    pub as_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime_as_rfc3339")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "final")]
    pub is_final: bool,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Delegation {
    pub voting_key: String,
    pub group: VoterGroupId,
    pub weight: i32,
    pub value: i64,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Delegator {
    pub delegations: Vec<Delegation>,
    pub reward_address: String,
    pub raw_power: i64,
    pub total_power: i64,
    #[serde(serialize_with = "serialize_datetime_as_rfc3339")]
    pub as_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_datetime_as_rfc3339")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "final")]
    pub is_final: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use serde_json::json;

    #[test]
    fn voter_json_test() {
        let voter = Voter {
            voter_info: VoterInfo {
                voting_power: 100,
                voting_group: VoterGroupId("rep".to_string()),
                delegations_power: 100,
                delegations_count: 1,
                voting_power_saturation: 0.4,
                delegator_addresses: Some(vec!["stake_public_key_1".to_string()]),
            },
            as_at: DateTime::from_utc(NaiveDateTime::default(), Utc),
            last_updated: DateTime::from_utc(NaiveDateTime::default(), Utc),
            is_final: true,
        };
        let json = serde_json::to_value(&voter).unwrap();
        assert_eq!(
            json,
            json!(
                {
                    "voter_info": {
                            "voting_power": 100,
                            "voting_group": "rep",
                            "delegations_power": 100,
                            "delegations_count": 1,
                            "voting_power_saturation": 0.4,
                            "delegator_addresses": ["stake_public_key_1"]
                        },
                    "as_at": "1970-01-01T00:00:00+00:00",
                    "last_updated": "1970-01-01T00:00:00+00:00",
                    "final": true
                }
            )
        );
    }

    #[test]
    fn delegator_json_test() {
        let delegator = Delegator {
            delegations: vec![Delegation {
                voting_key: "voter".to_string(),
                group: VoterGroupId("rep".to_string()),
                weight: 5,
                value: 100,
            }],
            reward_address: "reward address 1".to_string(),
            raw_power: 100,
            total_power: 1000,
            as_at: DateTime::from_utc(NaiveDateTime::default(), Utc),
            last_updated: DateTime::from_utc(NaiveDateTime::default(), Utc),
            is_final: true,
        };
        let json = serde_json::to_value(&delegator).unwrap();
        assert_eq!(
            json,
            json!(
                {
                    "delegations": [{"voting_key": "voter","group": "rep","weight": 5,"value": 100}],
                    "reward_address": "reward address 1",
                    "raw_power": 100,
                    "total_power": 1000,
                    "as_at": "1970-01-01T00:00:00+00:00",
                    "last_updated": "1970-01-01T00:00:00+00:00",
                    "final": true
                }
            )
        );
    }
}
