use std::collections::HashMap;

use rsrfc::{
    error::RfcErrorInfo, FromMethod, FromTable, ParamType, ParamValue, RfcClient, RfcResult,
    RfcTable,
};

#[derive(RfcTable)]
struct User {
    #[sap(alias = "WA")]
    _user_name: String,
}

#[derive(RfcResult)]
struct Response {
    #[sap(alias = "DATA")]
    _data: Vec<User>,
}

fn main() -> Result<(), RfcErrorInfo> {
    let client = RfcClient::new();
    let mut params: HashMap<&str, ParamType<'_>> = HashMap::new();
    params.insert("QUERY_TABLE", ParamType::Value(ParamValue::Str("USR02")));
    params.insert(
        "FIELDS",
        ParamType::Table(vec![vec![("FIELDNAME", ParamValue::Str("BNAME"))]]),
    );

    match client.execute::<Response>("RFC_READ_TABLE", params) {
        Ok(_response) => {
            // 处理 response 的逻辑
            println!("OK");
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    Ok(())
}
