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
            $( ( $value_type:ty, $register : ident), ),+
            $( ( ref $value_type_r:ty, $register_r : ident) ),* $(,)?
        )* }
        $(, deprecated:[$($dep:pat,)*])?
        $([$( $sp_pat : pat => {$sp_exp : expr} )+ ])? ) => {
        match $key {
            $(
                $( stringify!($register) => parse_value_inner!($app, $map, $target_type, $value_type, $register), )*
                $( stringify!($register_r) => parse_value_inner!($app, $map, $target_type, ref $value_type_r, $register_r), )*
            )*
            $($($sp_pat => {$sp_exp})*)*
            $(depr @ ($($dep )|* ) => {return Err(Error::custom(format_args!("deprecated :{}", depr)))})*
            unknown => return Err(Error::unknown_field(unknown, &[
                $( $( stringify!($register),)*
                    $( stringify!($register_r),)*  )*
                $($(stringify!($sp_pat),)*)* ]))
        }
    }
}
