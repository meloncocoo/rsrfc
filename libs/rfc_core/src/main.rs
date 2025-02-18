extern crate rsrfc;

use std::collections::HashMap;

use rsrfc::error::RfcErrorInfo;
use rsrfc::*;

fn main() -> Result<(), RfcErrorInfo> {
    let client = RfcClient::new();

    eprintln!("Fetching user names...");
    {
        let mut params = HashMap::new();
        params.insert("QUERY_TABLE", ParamType::Value(ParamValue::Str("USR02")));
        params.insert(
            "FIELDS",
            ParamType::Table(vec![vec![("FIELDNAME", ParamValue::Str("BNAME"))]]),
        );
        // client.excute("RFC_READ_TABLE", params)?;

        // Call the function
        // rfc_read_table.call()?;

        // Now the local data structures are filled with the response of the
        // remote function: retrieve the data
        // let data = rfc_read_table
        //     .get_mut_parameter("DATA")
        //     .ok_or(RfcErrorInfo::custom("unknown field DATA"))?;
        // // Get the intger index of the field to allow quicker access later
        // let idx_wa = data.get_field_index_by_name("WA")?;
        // let num_users = data.get_row_count()?;
        // eprintln!("Response from SAP has arrived: {} users.", num_users);
        // for i in 0..num_users {
        //     data.set_row(i)?;
        //     let row_content = data.get_field_by_index(idx_wa)?.get_chars()?;
        //     println!("Username: {}", row_content.trim_end());
        // }
    }

    Ok(())
}
