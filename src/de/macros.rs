macro_rules! parse_value_inner {
    ( $app : ident, $map : ident, $target_type:ty, $value_type:ty, $register : ident) => {
        <$target_type>::$register($app, $map.next_value::<$value_type>()?)
    };
    ( $app : ident, $map : ident, $target_type:ty, ref $value_type:ty, $register : ident) => {
        <$target_type>::$register($app, &$map.next_value::<$value_type>()?)
    };
}

macro_rules! parse_value {
    ($key : ident, $app : ident, $map : ident, $target_type:ty,
        { $(
            $( ( $register : ident, $value_type:ty), ),+
            $( ref ( $register_r : ident, $value_type_r:ty) ),* $(,)?
        )* }
        $(, tuple2:{$(( $register_t : ident, ( $value_type_t0:ty,  $value_type_t1:ty)),)*})?
        $(, tuple3:{$(( $register_3t : ident, ( $value_type_3t0:ty,  $value_type_3t1:ty,  $value_type_3t2:ty)),)*})?
        $(, deprecated:[$($dep:pat,)*])?
        $(specialize:[$( $sp_pat : pat => $sp_exp : expr )+ ])? ) => {
        match $key {
            $(
                $( stringify!($register) => parse_value_inner!($app, $map, $target_type, $value_type, $register), )*
                $( stringify!($register_r) => parse_value_inner!($app, $map, $target_type, ref $value_type_r, $register_r), )*
            )*
            $($(
                stringify!($register_t) => {
                    let (v0, v1) = $map.next_value::<($value_type_t0, $value_type_t1)>()?;
                    <$target_type>::$register_t($app, v0, v1)
                }
            )*)*            
            $($(
                stringify!($register_3t) => {
                    let (v0, v1, v2) = $map.next_value::<($value_type_3t0, $value_type_3t1, $value_type_3t2)>()?;
                    <$target_type>::$register_3t($app, v0, v1, v2)
                }
            )*)*
            $($($sp_pat => {$sp_exp})*)*
            $(depr @ ($($dep )|* ) => {return Err(Error::custom(format_args!("deprecated :{}", depr)))})*
            unknown => return Err(Error::unknown_field(unknown, &[
                $( $( stringify!($register),)*
                    $( stringify!($register_r),)*  )*
                $($(stringify!($sp_pat),)*)* ]))
        }
    }
}

macro_rules! enum_de {
    ($basety : ident, $newty :ident, $(#[$derive_meta:meta])* { $( $( #[ $cfg_meta:meta ] )? $var: ident ,)* } ) => {
        $(#[$derive_meta])*
        pub(crate) enum $newty {
            $(  $(#[$cfg_meta])* $var , )*
        }

        impl From<$newty> for $basety {
            fn from(s : $newty) -> $basety {
                match s {
                    $(
                        $(#[$cfg_meta])*
                        $newty::$var => $basety::$var,
                    )*
                }
            }
        }
    };
}
