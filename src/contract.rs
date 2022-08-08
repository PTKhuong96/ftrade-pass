#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{PassInfoResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{store, store_query, Pass};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let pass = Pass {
        id: "0".to_string(),
        name: msg.name,
        image_url: msg.image_url,
        desc: msg.desc,
        quantity_issued: msg.quantity_issued,
        expired_date: msg.expired_date,
        duration: msg.duration,
        nft_type: msg.nft_type,
        career: msg.career,
        course_type: msg.course_type,
        creator: msg.creator,
        owner: msg.owner,
    };
    let key = pass.id.as_bytes();
    store(deps.storage).save(key, &pass)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddNew {
            id,
            name,
            image_url,
            desc,
            quantity_issued,
            expired_date,
            duration,
            nft_type,
            career,
            course_type,
            creator,
            owner,
        } => add_new(
            deps,
            id,
            name,
            image_url,
            desc,
            quantity_issued,
            expired_date,
            duration,
            nft_type,
            career,
            course_type,
            creator,
            owner,
        ),
    }
}

pub fn add_new(
    deps: DepsMut,
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
) -> Result<Response, ContractError> {
    let pass = Pass {
            id,
            name,
            image_url,
            desc,
            quantity_issued,
            expired_date,
            duration,
            nft_type,
            career,
            course_type,
            creator,
            owner,
    };
    let key = pass.id.as_bytes();
    if (store(deps.storage).may_load(key)?).is_some() {
        // id is already taken
        return Err(ContractError::IdTaken { id: pass.id });
    }
    store(deps.storage).save(key, &pass)?;
    Ok(Response::new()
        .add_attribute("method", "add_new")
        .add_attribute("id", pass.id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPass { id } => query_pass(deps, id),
    }
}

fn query_pass(deps: Deps, id: String) -> StdResult<Binary> {
    let key = id.as_bytes();
    let pass = match store_query(deps.storage).may_load(key)? {
        Some(pass) => Some(pass),
        None => return Err(StdError::generic_err("Pass does not exist")),
    };

    let resp = PassInfoResponse { pass };
    to_binary(&resp)
}
