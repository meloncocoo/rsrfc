use std::collections::HashMap;

use chrono::{DateTime, Local};
use figment::providers::{Env, Format, Toml};

use crate::{
    RfcConnection, RfcConnectionParameters, RfcErrorInfo, RfcFunction, RfcLib, RfcParameter,
};

pub trait FromMethod {
    fn from_method(method: &mut RfcFunction) -> Result<Self, RfcErrorInfo>
    where
        Self: Sized;
}

pub trait FromTable {
    fn from_table(param: &mut RfcParameter) -> Result<Self, RfcErrorInfo>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub enum ParamValue<'a> {
    Str(&'a str),
    Dec(f32),
    Date(DateTime<Local>),
    Int(i64),
}

trait RfcLibTrait {
    fn connect<'t>(&self) -> Result<RfcConnection, RfcErrorInfo>;
}

impl RfcLibTrait for RfcLib {
    /// ### You must set the parameters required for RfcConnectionParameters
    /// ### either through the config.toml file or via environment variables before using RfcClient.
    ///
    /// Example of config.toml:
    /// ```toml
    /// [SAP]
    /// ashost = "127.0.0.1"
    /// sysnr = "00"
    /// client = "100"
    /// user = "username"
    /// passwd = "password"
    /// lang = "LANG"
    /// ```
    ///
    /// Example of environment variables:
    /// ```bash
    /// export SAP_ASHOST="127.0.0.1"
    /// export SAP_SYSNR="00"
    /// export SAP_CLIENT="100"
    /// export SAP_USER="username"
    /// export SAP_PASSWD="password"
    /// export SAP_LANG="LANG"
    /// ```
    fn connect<'t>(&self) -> Result<RfcConnection, RfcErrorInfo> {
        let figment = figment::Figment::from(Toml::file("config.toml").nested())
            .merge(Env::prefixed("SAP_").global())
            .select("SAP");
        let conn_params = figment
            .extract::<RfcConnectionParameters>()
            .expect("error while load sap connection parameters");
        let conn = RfcConnection::new(&conn_params, &self)?;
        Ok(conn)
    }
}

trait RfcConnectionTrait {
    fn with_method(&self, name: &str) -> Result<RfcFunction, RfcErrorInfo>;
}

impl<'conn> RfcConnectionTrait for RfcConnection<'conn> {
    fn with_method(&self, name: &str) -> Result<RfcFunction, RfcErrorInfo> {
        self.get_function(name)
    }
}

trait RfcParameterTrait<'a> {
    fn set_value(&mut self, value: ParamValue);
    fn set_struct(&mut self, value: Vec<(&'a str, ParamValue<'a>)>);
    fn set_table(&mut self, value: Vec<Vec<(&'a str, ParamValue<'a>)>>);
}

impl<'a> RfcParameterTrait<'a> for RfcParameter<'_, '_> {
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

    fn set_struct(&mut self, value: Vec<(&'a str, ParamValue<'a>)>) {
        value
            .into_iter()
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

    fn set_table(&mut self, value: Vec<Vec<(&'a str, ParamValue<'a>)>>) {
        value
            .into_iter()
            .for_each(|item| match self.append_rows(1) {
                Ok(_) => match self.first_row() {
                    Ok(_) => self.set_struct(item),
                    Err(_) => todo!(),
                },
                Err(_) => todo!(),
            });
    }
}

pub enum ParamType<'a> {
    Value(ParamValue<'a>),
    Struct(Vec<(&'a str, ParamValue<'a>)>),
    Table(Vec<Vec<(&'a str, ParamValue<'a>)>>),
}
pub struct RfcClient<'t> {
    params: HashMap<&'t str, ParamType<'t>>,
    rfc_lib: RfcLib,
}

impl<'client> RfcClient<'client> {
    pub fn new() -> Self {
        let rfc_lib = RfcLib::new().expect("Unable to open the rfc lib");

        Self {
            rfc_lib,
            params: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.params.clear();
    }

    pub fn with_param(&mut self, name: &'client str, value: ParamType<'client>) {
        self.params.insert(name, value);
    }

    pub fn execute<T: FromMethod>(
        self,
        name: &str,
        params: HashMap<&str, ParamType>,
    ) -> Result<T, RfcErrorInfo> {
        match self.rfc_lib.connect()?.with_method(name) {
            Ok(mut method) => {
                params
                    .into_iter()
                    .for_each(|(name, value)| match method.get_mut_parameter(name) {
                        Some(param) => match value {
                            ParamType::Value(v) => param.set_value(v),
                            ParamType::Struct(v) => param.set_struct(v),
                            ParamType::Table(v) => param.set_table(v),
                        },
                        None => eprintln!("input param {} not exists.", name),
                    });

                method.call()?;
                T::from_method(&mut method)
            }
            Err(err) => Err(err),
        }
    }
}
