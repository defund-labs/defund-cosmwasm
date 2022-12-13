use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use defund_cosmwasm::{
    GetFundResponse, GetFundsResponse, GetFundPriceResponse, GetBrokerResponse, GetBrokersResponse, DefundMsg, DefundQueryWrapper, DefundQuery, DefundRoute, 
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(DefundMsg), &out_dir);
    export_schema(&schema_for!(DefundQueryWrapper), &out_dir);
    export_schema(&schema_for!(DefundQuery), &out_dir);
    export_schema(&schema_for!(DefundRoute), &out_dir);
    export_schema(&schema_for!(GetFundResponse), &out_dir);
    export_schema(&schema_for!(GetFundsResponse), &out_dir);
    export_schema(&schema_for!(GetFundPriceResponse), &out_dir);
    export_schema(&schema_for!(GetBrokerResponse), &out_dir);
    export_schema(&schema_for!(GetBrokersResponse), &out_dir);
}