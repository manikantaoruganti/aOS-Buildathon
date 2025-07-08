
use cosmwasm_std::Binary;
use cw721_base::ContractError;
use cw721_base::entry::{execute, instantiate, query};
use cw721_base::Cw721Contract;
use cw721::Cw721ExecuteMsg;

// Define our contract wrapper
pub type Extension = Option<Metadata>;
pub type OptiVaultContract<'a> = Cw721Contract<'a, Extension, Metadata>;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Metadata {
    pub description: Option<String>,
    pub value_usd: Option<u128>,
    pub asset_type: Option<String>
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, ContractError> {
    let contract = OptiVaultContract::default();
    contract.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ExecuteMsg<Extension>
) -> Result<Response, ContractError> {
    let contract = OptiVaultContract::default();
    contract.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    let contract = OptiVaultContract::default();
    contract.query(deps, env, msg)
}
