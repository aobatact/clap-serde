

macro_rules! serialize_keys {
    ($app : ident, $map : ident, get : { $( $getter:ident ),* $(,)? }) => {
        $(
            if let Some(x_key) = $app.$getter(){
                // "get_*" -> "*" 
                let key_name = stringify!($getter).split_at(4).1;
                $map.serialize_entry(key_name, &x_key)?;
            }
        )*
    };
}