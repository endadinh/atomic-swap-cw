use cosmwasm_schema::write_api;
use dm_cw::msg::ExecuteMsg;
use dm_cw::msg::InstantiateMsg;
use dm_cw::msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
