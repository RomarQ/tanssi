// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

use {
    container_chain_template_simple_runtime::{
        AccountId, MaintenanceModeConfig, MigrationsConfig, PolkadotXcmConfig, Signature, Balance,
        constants::currency::DOLLARS,
    },
    cumulus_primitives_core::ParaId,
    sc_chain_spec::{ChainSpecExtension, ChainSpecGroup},
    sc_network::config::MultiaddrWithPeerId,
    sc_service::ChainType,
    serde::{Deserialize, Serialize},
    sp_core::{sr25519, Pair, Public},
    sp_runtime::traits::{IdentifyAccount, Verify},
};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<
    container_chain_template_simple_runtime::RuntimeGenesisConfig,
    Extensions,
>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Orcherstrator's parachain id
const ORCHESTRATOR: ParaId = ParaId::new(3000);

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config(para_id: ParaId, boot_nodes: Vec<String>) -> ChainSpec {
    // Give your base currency a XCAV name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XCAV".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());

    let mut default_funded_accounts = get_endowed_accounts_with_balance();
    default_funded_accounts.sort();
    default_funded_accounts.dedup();
    let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
        .into_iter()
        .map(|x| {
            x.parse::<MultiaddrWithPeerId>()
                .unwrap_or_else(|e| panic!("invalid bootnode address format {:?}: {:?}", x, e))
        })
        .collect();

    ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                default_funded_accounts.clone(),
                para_id,
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        boot_nodes,
        None,
        None,
        None,
        Some(properties),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
}

pub fn local_testnet_config(para_id: ParaId, boot_nodes: Vec<String>) -> ChainSpec {
    // Give your base currency a XCAV name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XCAV".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("isEthereum".into(), false.into());
    let protocol_id = Some(format!("container-chain-{}", para_id));

    let mut default_funded_accounts = get_endowed_accounts_with_balance();
    default_funded_accounts.sort();
    default_funded_accounts.dedup();
    let boot_nodes: Vec<MultiaddrWithPeerId> = boot_nodes
        .into_iter()
        .map(|x| {
            x.parse::<MultiaddrWithPeerId>()
                .unwrap_or_else(|e| panic!("invalid bootnode address format {:?}: {:?}", x, e))
        })
        .collect();

    ChainSpec::from_genesis(
        // Name
        &format!("Simple Container {}", para_id),
        // ID
        &format!("simple_container_{}", para_id),
        ChainType::Local,
        move || {
            testnet_genesis(
                default_funded_accounts.clone(),
                para_id,
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        // Bootnodes
        boot_nodes,
        // Telemetry
        None,
        // Protocol ID
        protocol_id.as_deref(),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: para_id.into(),
        },
    )
}

pub fn get_endowed_accounts_with_balance() -> Vec<(AccountId, u128)> {
	let accounts: Vec<AccountId> = vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
	];

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	let accounts_with_balance: Vec<(AccountId, u128)> =
		accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect();
	let json_data = &include_bytes!("../../../../../seed/balances.json")[..];
	let additional_accounts_with_balance: Vec<(AccountId, u128)> =
		serde_json::from_slice(json_data).unwrap();

	let mut accounts = additional_accounts_with_balance.clone();

	accounts_with_balance.iter().for_each(|tup1| {
		for tup2 in additional_accounts_with_balance.iter() {
			if tup1.0 == tup2.0 {
				return;
			}
		}
		accounts.push(tup1.to_owned());
	});

	accounts
}

fn testnet_genesis(
    endowed_accounts: Vec<(AccountId, u128)>,
    id: ParaId,
    root_key: AccountId,
) -> container_chain_template_simple_runtime::RuntimeGenesisConfig {
    container_chain_template_simple_runtime::RuntimeGenesisConfig {
        system: container_chain_template_simple_runtime::SystemConfig {
            code: container_chain_template_simple_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
            ..Default::default()
        },
        balances: container_chain_template_simple_runtime::BalancesConfig {
            balances: endowed_accounts.iter().cloned().map(|x| (x.0.clone(), x.1)).collect(),
        },
        assets: container_chain_template_simple_runtime::AssetsConfig {
            assets: vec![(9, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1)],
			..Default::default()
        },
        parachain_info: container_chain_template_simple_runtime::ParachainInfoConfig {
            parachain_id: id,
            ..Default::default()
        },
        parachain_system: Default::default(),
        sudo: container_chain_template_simple_runtime::SudoConfig {
            key: Some(root_key),
        },
        authorities_noting: container_chain_template_simple_runtime::AuthoritiesNotingConfig {
            orchestrator_para_id: ORCHESTRATOR,
            ..Default::default()
        },
        migrations: MigrationsConfig::default(),
        maintenance_mode: MaintenanceModeConfig {
            start_in_maintenance_mode: false,
            ..Default::default()
        },
        // This should initialize it to whatever we have set in the pallet
        polkadot_xcm: PolkadotXcmConfig::default(),
        transaction_payment: Default::default(),
        community_loan_pool: Default::default(),
        alliance_motion: Default::default(),
        council: container_chain_template_simple_runtime::CouncilConfig { members: vec![], phantom: Default::default() },
        technical_committee: container_chain_template_simple_runtime::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
        treasury: Default::default(),
    }
}

