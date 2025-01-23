use std::{collections::HashMap, time};

use chrono::{DateTime, Local};

use crate::{RfcConnection, RfcConnectionParameters, RfcLib, RfcParameter};

#[derive(Debug, Clone)]
enum ParamValue<'a> {
    Str(&'a str),
    Dec(f32),
    Date(DateTime<Local>),
    Int(i64),
}

trait RfcParameterTrait {
    fn set_value(&mut self, value: ParamValue);
    fn set_struct(&mut self, value: &HashMap<String, ParamValue>);
    fn set_table(&mut self, value: &Vec<HashMap<String, ParamValue>>);
}

impl RfcParameterTrait for RfcParameter<'_, '_> {
    fn set_value(&mut self, value: ParamValue) {
        match match value {
            ParamValue::Str(v) => self.set_string(v),
            ParamValue::Dec(v) => self.set_float(v),
            ParamValue::Date(v) => self.set_date(v.format("%Y%m%d").to_string().as_str()),
            ParamValue::Int(v) => self.set_int(v),
        } {
            Err(err) => eprintln!("error while set value for params, error: {}", err),
            _ => {}
        };
    }

    fn set_struct(&mut self, value: &HashMap<String, ParamValue>) {
        value
            .iter()
            .for_each(|(name, value)| match self.get_field_index_by_name(name) {
                Ok(index) => match self.get_field_by_index(index) {
                    Ok(param) => param.set_value(value.clone()), // 使用 clone() 以避免移动
                    Err(err) => eprintln!("error while get field by index, error: {}", err),
                },
                Err(err) => eprintln!(
                    "error while get field index by name: {}, error: {}",
                    name, err
                ),
            });
    }

    fn set_table(&mut self, value: &Vec<HashMap<String, ParamValue>>) {
        value.iter().for_each(|item| match self.append_rows(1) {
            Ok(_) => match self.first_row() {
                Ok(_) => self.set_struct(item),
                Err(_) => todo!(),
            },
            Err(_) => todo!(),
        });
    }
}

pub struct RfcClient<'client> {
    conn_params: RfcConnectionParameters<'client>,
    rfc_dll: RfcLib,
}

fn header<'a>() -> HashMap<String, ParamValue<'a>> {
    let mut header_map = HashMap::new();
    header_map.insert(String::from("BUKRS"), ParamValue::Str("8000"));
    header_map.insert(String::from("BSART"), ParamValue::Str("D"));
    header_map.insert(String::from("ERNAM"), ParamValue::Str(""));
    header_map.insert(String::from("LIFNR"), ParamValue::Str("125197"));
    header_map.insert(String::from("ZTERM"), ParamValue::Str(""));
    header_map.insert(String::from("EKORG"), ParamValue::Str("P100"));
    header_map.insert(String::from("EKGRP"), ParamValue::Str("101"));
    header_map.insert(String::from("WAERS"), ParamValue::Str("CNY"));
    header_map.insert(String::from("WKURS"), ParamValue::Dec(1.0));
    header_map.insert(String::from("BEDAT"), ParamValue::Date(Local::now()));
    header_map
}
fn table<'a>() -> Vec<HashMap<String, ParamValue<'a>>> {
    let mut result: Vec<HashMap<String, ParamValue<'a>>> = vec![];
    (1..10).into_iter().for_each(|i| {
        let mut header_map = HashMap::new();
        header_map.insert(String::from("EBELP"), ParamValue::Int(10 * i));
        header_map.insert(String::from("TXZ01"), ParamValue::Str(""));
        header_map.insert(String::from("MATNR"), ParamValue::Str("1000001"));
        header_map.insert(String::from("WERKS"), ParamValue::Str("8000"));
        header_map.insert(String::from("LGORT"), ParamValue::Str(""));
        header_map.insert(String::from("MENGE"), ParamValue::Dec(12.0));
        header_map.insert(String::from("MEINS"), ParamValue::Str("PCS"));
        header_map.insert(String::from("BPRME"), ParamValue::Str(""));
        header_map.insert(String::from("NETPR"), ParamValue::Dec(2.2));
        header_map.insert(String::from("PEINH"), ParamValue::Dec(1.0));
        header_map.insert(String::from("MWSKZ"), ParamValue::Str(""));
        header_map.insert(String::from("PSTYP"), ParamValue::Str(""));
        header_map.insert(String::from("KNTTP"), ParamValue::Str(""));
        header_map.insert(String::from("REPOS"), ParamValue::Str(""));
        header_map.insert(String::from("BANFN"), ParamValue::Str(""));
        header_map.insert(String::from("BNFPO"), ParamValue::Int(10 * i));
        header_map.insert(String::from("RETPO"), ParamValue::Str(""));
        header_map.insert(String::from("EINDT"), ParamValue::Date(Local::now()));
        header_map.insert(String::from("SAKTO"), ParamValue::Str(""));
        header_map.insert(String::from("KOSTL"), ParamValue::Str(""));
        header_map.insert(String::from("ANLN1"), ParamValue::Str(""));
        header_map.insert(String::from("ANLN2"), ParamValue::Str(""));
        header_map.insert(String::from("PRCTR"), ParamValue::Str(""));
        header_map.insert(String::from("POSID"), ParamValue::Str(""));
        result.push(header_map);
    });
    result
}

impl<'client> RfcClient<'client> {
    pub fn new() -> Self {
        let rfc_dll = RfcLib::new().expect("Unable to open the rfc lib");
        let conn_params = RfcConnectionParameters {
            ashost: "192.168.89.51",
            sysnr: "00",
            client: "300",
            user: "OA",
            passwd: "Aa12345678",
            lang: "ZH",
        };

        Self {
            conn_params,
            rfc_dll,
        }
    }

    pub fn excute(self, method: &str) {
        match RfcConnection::new(&self.conn_params, &self.rfc_dll) {
            Ok(conn) => match conn.get_function(method) {
                Ok(mut method) => {
                    match method.get_mut_parameter("IV_ZSQDH") {
                        Some(param) => param.set_value(ParamValue::Str("8000")),
                        None => eprintln!("input param {} not exists.", "IV_ZSQDH"),
                    }

                    match method.get_mut_parameter("IW_HEADER") {
                        Some(param) => param.set_struct(&header()),
                        None => todo!(),
                    }

                    match method.get_mut_parameter("IT_ITEM") {
                        Some(param) => param.set_table(&table()),
                        None => todo!(),
                    }

                    match method.call() {
                        Ok(()) => {
                            eprintln!("call method successfully.");
                            match method.get_parameter("EV_TYPE") {
                                Some(ev_type) => {
                                    eprintln!("ev_type: {}", ev_type.get_chars().unwrap())
                                }
                                None => eprintln!("EV_TYPE is empty"),
                            }
                            match method.get_parameter("EV_MSG") {
                                Some(ev_message) => {
                                    eprintln!("ev_msg: {}", ev_message.get_chars().unwrap())
                                }
                                None => eprintln!("ev_msg is empty"),
                            }
                        }
                        Err(err) => eprintln!("error while call method: {}", err),
                    }
                }
                Err(err) => eprintln!(
                    "error while get method with name: {}, error: {}",
                    method, err
                ),
            },
            Err(err) => eprintln!("error while connect to sap rfc: {}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = RfcClient::new();
        client.excute("ZFM_WECOM_CREATE_PO");
    }
}
