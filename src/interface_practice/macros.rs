// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub value: String,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
}

macro_rules! implement_data_source {
    ($name:ident, $item_type:ty, $error_type:ty) => {
        pub struct $name {
            connection: String,
            config: std::collections::HashMap<String, String>,
        }

        impl $name {
            pub fn new(connection: String) -> Self {
                unimplemented!("Create new {} instance", stringify!($name))
            }
        }

        impl DataSource for $name {
            type Item = $item_type;
            
            fn fetch_data(&self) -> Result<Vec<Self::Item>, String> {
                unimplemented!("Fetch data for {}", stringify!($name))
            }
            
            fn is_available(&self) -> bool {
                unimplemented!("Check availability for {}", stringify!($name))
            }
        }
    };
}

macro_rules! create_processor {
    ($name:ident, $input:ty => $output:ty, {$($method:ident: $impl:expr),*}) => {
        pub struct $name;
        
        impl $name {
            pub fn new() -> Self {
                Self
            }
            
            $(
                pub fn $method(&self) -> impl Fn() -> String {
                    move || $impl.to_string()
                }
            )*
        }
        
        impl DataProcessor for $name {
            type Input = $input;
            type Output = $output;
            
            fn process(&self, _data: Self::Input) -> Result<Self::Output, String> {
                unimplemented!("Process data in {}", stringify!($name))
            }
            
            fn validate_input(&self, _data: &Self::Input) -> bool {
                unimplemented!("Validate input in {}", stringify!($name))
            }
        }
    };
}

macro_rules! config_builder {
    ($(
        $field:ident: $type:ty = $default:expr
    ),* $(,)?) => {
        #[derive(Debug, Clone)]
        pub struct ConfigBuilder {
            $(
                $field: Option<$type>,
            )*
        }
        
        impl ConfigBuilder {
            pub fn new() -> Self {
                Self {
                    $(
                        $field: None,
                    )*
                }
            }
            
            $(
                pub fn $field(mut self, value: $type) -> Self {
                    self.$field = Some(value);
                    self
                }
            )*
            
            pub fn build(self) -> ConfigResult {
                ConfigResult {
                    $(
                        $field: self.$field.unwrap_or($default),
                    )*
                }
            }
        }
        
        #[derive(Debug, Clone)]
        pub struct ConfigResult {
            $(
                pub $field: $type,
            )*
        }
    };
}

macro_rules! generate_error_types {
    ($($error_name:ident {
        $($variant:ident($variant_type:ty)),* $(,)?
    })*) => {
        $(
            #[derive(Debug, Clone)]
            pub enum $error_name {
                $(
                    $variant($variant_type),
                )*
            }
            
            impl std::fmt::Display for $error_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            $error_name::$variant(inner) => write!(f, "{}: {:?}", stringify!($variant), inner),
                        )*
                    }
                }
            }
            
            impl std::error::Error for $error_name {}
        )*
    };
}

implement_data_source!(PostgresSource, String, std::io::Error);
implement_data_source!(RedisSource, Vec<u8>, String);
implement_data_source!(HttpSource, serde_json::Value, reqwest::Error);

create_processor!(
    JsonTransformer, 
    Vec<String> => Vec<serde_json::Value>,
    {
        transform_method: "JSON transformation",
        validation_method: "JSON validation"
    }
);

config_builder! {
    host: String = "localhost".to_string(),
    port: u16 = 8080,
    timeout: u64 = 30,
    max_connections: u32 = 100,
    ssl_enabled: bool = false,
}

generate_error_types! {
    DatabaseError {
        Connection(String),
        Query(String),
        Timeout(u64),
    }
    
    ProcessingError {
        InvalidInput(String),
        Transformation(String),
        Validation(String),
    }
}

pub use crate::interface_practice::traits::*;