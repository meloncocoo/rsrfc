use std::collections::HashMap;

use rsrfc::{error::*, *};

fn main() {
    println!("main函数开始执行");

    test();
    println!("main函数执行完毕");
}

#[derive(Debug, RfcTable)]
struct Field {
    #[sap(alias = "EBELN")]
    order_code: String,
    #[sap(alias = "TYPE")]
    r#type: String,
    #[sap(alias = "MSG")]
    msg: String,
}

#[derive(Debug, RfcResult)]
struct SimpleResult {
    #[sap(alias = "EV_EBELN")]
    ev_ebeln: String,
    #[sap(alias = "EV_TYPE")]
    ev_type: String,
    #[sap(alias = "EV_MSG")]
    ev_msg: String,
    #[sap(alias = "CT_DATA")]
    et_data: Vec<Field>,
}

fn test() {
    println!("Testing RfcClient creation and destruction");
    {
        // Try to create a new RfcClient instance
        let client_result = RfcClient::new();
        match client_result {
            Ok(client) => {
                println!("RfcClient created successfully");
                let mut params: HashMap<&str, ParamType<'_>> = HashMap::new();
                params.insert("IV_ZSQDH", ParamType::Value(ParamValue::Str("********")));
                params.insert(
                    "CT_DATA",
                    ParamType::Table(vec![vec![
                        ("BANFN", ParamValue::Str("********")),
                        ("BUKRS", ParamValue::Str("****")),
                        ("WERKS", ParamValue::Str("****")),
                        ("MENGE", ParamValue::Dec(1.0)),
                        ("MEINS", ParamValue::Str("**")),
                    ]]),
                );
                match client.execute::<SimpleResult>("********", params) {
                    Ok(result) => {
                        println!("RFC call successful");
                        println!("RFC return result: {:?}", result);
                    }
                    Err(e) => {
                        println!("RFC call failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("RfcClient creation failed: {}", e);
            }
        }
    }
}
