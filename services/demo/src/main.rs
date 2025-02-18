use std::collections::HashMap;

use rsrfc::{
    error::RfcErrorInfo, FromMethod, FromTable, ParamType, ParamValue, RfcClient, RfcResult,
    RfcTable,
};

#[derive(RfcTable)]
struct User {
    // #[sap(alias = "WA")]
    // user_name: String,
    #[sap(alias = "WA")]
    real_name: String,
}

// impl FromTable for User {
//     fn from_table(param: &mut rsrfc::RfcParameter) -> Result<Self, RfcErrorInfo> {
//         let idx = param.get_field_index_by_name("WA")?;
//         let row = param.get_field_by_index(idx)?.get_chars()?;

//         Ok(User {
//             user_name: row.trim_end().to_string(),
//         })
//     }
// }

#[derive(RfcResult)]
struct Response {
    #[sap(alias = "DATA")]
    data: Vec<User>,
    // #[sap(alias = "EV_TYPE")]
    // ev_type: Option<String>,
}

// #[derive(Debug, RfcResult)]
// pub struct Response {
//     #[sap(alias = "EV_TYPE")]
//     pub ev_type: String,
// }

// impl FromMethod for Response {
//     fn from_method(method: &mut rsrfc::RfcFunction) -> Result<Self, RfcErrorInfo> {
//         let data = method
//             .get_mut_parameter("DATA")
//             .ok_or(RfcErrorInfo::custom("unknown field DATA"))?;
//         // let idx_wa = data.get_field_index_by_name("WA")?;
//         let num_users = data.get_row_count()?;
//         let mut users: Vec<User> = vec![];

//         for i in 0..num_users {
//             data.set_row(i)?;
//             // let row_content = data.get_field_by_index(idx_wa)?.get_chars()?;
//             // let user = User {
//             //     user_name: row_content.trim_end().to_string(),
//             // };
//             let u = User::from_table(data)?;
//             users.push(u);
//         }
//         Ok(Response { data: users })
//     }
// }

fn main() -> Result<(), RfcErrorInfo> {
    let client = RfcClient::new();
    let mut params: HashMap<&str, ParamType<'_>> = HashMap::new();
    params.insert("QUERY_TABLE", ParamType::Value(ParamValue::Str("USR02")));
    params.insert(
        "FIELDS",
        ParamType::Table(vec![vec![
            // ("FIELDNAME", ParamValue::Str("BNAME")),
            ("FIELDNAME", ParamValue::Str("ERDAT")),
        ]]),
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
