use std::collections::HashMap;
use std::env;

use chrono::{DateTime, Local};
use dotenv::dotenv;
use figment::providers::{Env, Format, Toml};
use figment::Figment;

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
        dotenv().ok();

        // 首先尝试从 config.toml 文件中读取配置
        let figment = Figment::from(Toml::file("config.toml").nested())
            .merge(Env::prefixed("SAP_").global())
            .select("SAP");

        // 尝试获取各个参数，优先使用环境变量
        let ashost = get_param_value(&figment, "ashost", "SAP_ASHOST")?;
        let sysnr = get_param_value(&figment, "sysnr", "SAP_SYSNR")?;
        let client = get_param_value(&figment, "client", "SAP_CLIENT")?;
        let user = get_param_value(&figment, "user", "SAP_USER")?;
        let passwd = get_param_value(&figment, "passwd", "SAP_PASSWD")?;
        let lang = get_param_value(&figment, "lang", "SAP_LANG")?;

        let conn_params = RfcConnectionParameters {
            ashost,
            sysnr,
            client,
            user,
            passwd,
            lang,
        };

        let conn = RfcConnection::new(&conn_params, &self)?;
        Ok(conn)
    }
}

// 辅助函数，从 figment 或环境变量中获取参数值
fn get_param_value(figment: &Figment, key: &str, env_key: &str) -> Result<String, RfcErrorInfo> {
    // 首先尝试从环境变量中获取
    if let Ok(value) = env::var(env_key) {
        if !value.is_empty() {
            return Ok(value);
        }
    }

    // 如果环境变量不存在或为空，尝试从 figment 中获取
    if let Ok(value) = figment.extract_inner::<String>(key) {
        if !value.is_empty() {
            return Ok(value);
        }
    }

    // 如果都不存在，返回错误
    Err(RfcErrorInfo::custom(&format!(
        "{} not set in environment variables or config.toml",
        key
    )))
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
    pub fn new() -> Result<Self, RfcErrorInfo> {
        let rfc_lib = RfcLib::new().map_err(|e| RfcErrorInfo::custom(&e))?;

        Ok(Self {
            rfc_lib,
            params: HashMap::new(),
        })
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
        let conn = self.rfc_lib.connect()?;
        let mut method = conn.with_method(name)?;

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
        let result = T::from_method(&mut method);

        // 确保 conn 在整个方法执行期间保持有效
        // drop(method);
        // drop(conn);

        result
    }
}
