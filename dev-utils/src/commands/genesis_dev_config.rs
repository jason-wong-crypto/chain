use std::{collections::BTreeMap, str::FromStr};

use serde::{Deserialize, Serialize};

use chain_core::init::{
    address::RedeemAddress,
    coin::Coin,
    config::{JailingParameters, RewardsParameters, SlashRatio, SlashingParameters},
};
use chain_core::state::account::{ValidatorName, ValidatorSecurityContact};
use chain_core::state::tendermint::TendermintValidatorPubKey;

#[derive(Deserialize, Debug)]
pub struct GenesisDevConfig {
    pub distribution: BTreeMap<RedeemAddress, Coin>,
    pub unbonding_period: u32,
    pub required_council_node_stake: Coin,
    pub jailing_config: JailingParameters,
    pub slashing_config: SlashingParameters,
    pub rewards_config: RewardsParameters,
    pub initial_fee_policy: InitialFeePolicy,
    pub council_nodes: BTreeMap<
        RedeemAddress,
        (
            ValidatorName,
            ValidatorSecurityContact,
            TendermintValidatorPubKey,
        ),
    >,
}

impl GenesisDevConfig {
    pub fn new(expansion_cap: Coin) -> Self {
        GenesisDevConfig {
            distribution: BTreeMap::new(),
            unbonding_period: 60,
            required_council_node_stake: Coin::new(1_250_000_000_000_000_000).unwrap(),
            jailing_config: JailingParameters {
                jail_duration: 86400,
                block_signing_window: 100,
                missed_block_threshold: 50,
            },
            slashing_config: SlashingParameters {
                liveness_slash_percent: SlashRatio::from_str("0.1").unwrap(),
                byzantine_slash_percent: SlashRatio::from_str("0.2").unwrap(),
                slash_wait_period: 10800,
            },
            rewards_config: RewardsParameters {
                monetary_expansion_cap: expansion_cap,
                distribution_period: 24 * 60 * 60,
                monetary_expansion_r0: "0.45".parse().unwrap(),
                monetary_expansion_tau: 1_4500_0000_0000_0000,
                monetary_expansion_decay: 999_860,
            },
            initial_fee_policy: InitialFeePolicy {
                base_fee: "1.1".to_string(),
                per_byte_fee: "1.25".to_string(),
            },
            council_nodes: BTreeMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialFeePolicy {
    pub base_fee: String,
    pub per_byte_fee: String,
}
