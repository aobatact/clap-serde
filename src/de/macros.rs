/// Deserialize the value and call the builder method.
/// `null` (unit) value is ignored and the target is returned as it is.
macro_rules! parse_value_inner {
    ( $app : ident, $map : ident, $target_type:ty, $value_type:ty, $register : ident) => {
        match $map.next_value::<Option<$value_type>>()? {
            Some(v) => <$target_type>::$register($app, v),
            None => $app,
        }
    };
    ( $app : ident, $map : ident, $target_type:ty, ref $value_type:ty, $register : ident) => {
        match $map.next_value::<Option<$value_type>>()? {
            Some(v) => <$target_type>::$register($app, &v),
            None => $app,
        }
    };
}

/// Deserialize `Option<$value_type>` and apply `$f` if it is `Some`.
macro_rules! parse_opt {
    ( $app : ident, $map : ident, $value_type:ty, |$v:ident| $f:expr) => {
        match $map.next_value::<Option<$value_type>>()? {
            Some($v) => $f,
            None => $app,
        }
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
        $(, deprecated:$([$($dep:pat ,)*])?$({$($dep_s:pat => $dep_d:expr,)*})?)?
        $(, not_supported:{$($ns:pat => $ns_r:expr ,)*})?
        $(, specialize:[$( $sp_pat : pat => $sp_exp : expr )+ ])? ) => {{
            #[allow(unused_mut)]
            let mut key: String = convert_case_to!($key);

            #[allow(unused_labels)]
            'parse_value_jmp_loop: loop {
                break 'parse_value_jmp_loop match key.as_str() {
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
                    $($(depr @ ($($dep )|* ) => {return Err(Error::custom(format_args!("deprecated key: {}", depr)))})*)*
                    $($($(
                        #[cfg(feature="allow-deprecated")]
                        $dep_s => {
                            key = String::from($dep_d);
                            continue 'parse_value_jmp_loop;
                        },
                        #[cfg(not(feature="allow-deprecated"))]
                        $dep_s => {
                            return Err(Error::custom(format_args!("deprecated key: {}, use {} instead", key, $dep_d)))
                        },
                    )*)*)*
                    $($(
                        $ns => {
                            return Err(Error::custom(format_args!("not supported key : {}, {} ", key, $ns_r)))
                        }
                    )*)*
                    unknown => return Err(Error::unknown_field(unknown, &[
                        $( $( stringify!($register),)*
                            $( stringify!($register_r),)*  )*
                        $($(stringify!($sp_pat),)*)* ]))
                }
            }
        }
    }
}

#[cfg(feature = "snake-case-key")]
macro_rules! convert_case_to {
    ($key:expr) => {
        String::from(AsRef::<str>::as_ref(&$key))
    };
}

// if const heap is stabilized, should convert the target keys instead.

#[cfg(not(feature = "snake-case-key"))]
macro_rules! convert_case_to {
    ($key:expr) => {
        convert_case::Casing::to_case(&AsRef::<str>::as_ref(&$key), convert_case::Case::Snake)
    };
}

macro_rules! enum_de {
    ($basety : ident, $newty :ident,
        $(#[$derive_meta:meta])* {
            $(
                $( #[ $cfg_meta:meta ] )?
                $var: ident
            ,)*
        }
        $({
            $( $(
                #[ $cfg_meta_ex:meta ] )?
                $var_ex: ident $( { $( $(#[$cfg_v:meta])* $vx: ident : $vt: ty ),* } )?
                    => $to_ex: expr
            ,)*
        } )?
    ) => {

        $(#[$derive_meta])*
        pub(crate) enum $newty {
            $(  $(#[$cfg_meta])* $var , )*
            $($(  $(#[$cfg_meta_ex])* $var_ex $( { $( $( #[$cfg_v] )* $vx :  $vt ,)* } )* , )*)*
        }

        impl From<$newty> for $basety {
            fn from(s : $newty) -> $basety {
                match s {
                    $(
                        $(#[$cfg_meta])*
                        $newty::$var => $basety::$var,
                    )*
                    $($(
                        $(#[$cfg_meta_ex])*
                        $newty::$var_ex$({$( $vx ,)*})* => { $to_ex },
                    )*)*
                }
            }
        }

        impl $newty {
            #[allow(dead_code)]
            pub fn from_clap_type(c : $basety) -> Option<Self> {
                #[allow(unreachable_patterns)]
                match c {
                    $(
                        $(#[$cfg_meta])*
                        $basety::$var => Some($newty::$var),
                    )*
                    _ => None,
                }
            }
        }
    };
}
