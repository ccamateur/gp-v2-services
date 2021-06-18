#[macro_export]
macro_rules! addr {
    ($val:literal) => {
        ::ethcontract::H160(::hex_literal::hex!($val))
    };
}

#[macro_export]
macro_rules! json_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut map = ::serde_json::Map::<String, ::serde_json::Value>::new();
        $(
            map.insert(($key).into(), ($value).into());
        )*
        map
    }}
}
