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
        $(, deprecated:$([$($dep:pat ,)*])?$({$($dep_s:pat => $dep_d:expr,)*})?)?
        $(, specialize:[$( $sp_pat : pat => $sp_exp : expr )+ ])? ) => {{
            #[allow(unused_mut)]
            let mut key;
            convert_case_to!($key, key);

            #[allow(unused_labels)]
            'parse_value_jmp_loop: loop {
                break 'parse_value_jmp_loop match key {
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
                            const N_KEY : &str = stringify!($dep_d);
                            convert_case_to!(N_KEY, key);
                            continue 'parse_value_jmp_loop;
                        },
                        #[cfg(not(feature="allow-deprecated"))]
                        $dep_s => {
                            return Err(Error::custom(format_args!("deprecated key: {}, use {} insted", stringify!($depr_s), $dep_d)))
                        },
                    )*)*)*
                    unknown => return Err(Error::unknown_field(unknown, &[
                        $( $( stringify!($register),)*
                            $( stringify!($register_r),)*  )*
                        $($(stringify!($sp_pat),)*)* ]))
                }
            }
        }
    }
}

macro_rules! parse_value_array {
    ($app : ident, $target_type:ty, $seq : ident,
        { $(
            $( ( $register : ident, $value_type:ty), ),+
            $( ref ( $register_r : ident, $value_type_r:ty) ),* $(,)?
        )* }
        $(, tuple2:{$(( $register_t : ident, ( $value_type_t0:ty,  $value_type_t1:ty)),)*})?
        $(, tuple3:{$(( $register_3t : ident, ( $value_type_3t0:ty,  $value_type_3t1:ty,  $value_type_3t2:ty)),)*})?
        $(, deprecated:$([$($dep:pat ,)*])?$({$($dep_s:pat => $dep_d:expr,)*})?)?
        $(, specialize:[$( $sp_pat : ident => $sp_exp : expr )+ ])? ) => {{

            #[derive(Deserialize)]
            #[allow(non_camel_case_types)]
            pub enum PvValue<'pv> {
                $(
                    $($register($value_type),)*
                    $($register_r($value_type_r),)*
                )*
                $($(
                    $register_t($value_type_t0, $value_type_t1),
                )*)*
                $($(
                    $register_3t($value_type_3t0, $value_type_3t1, $value_type_3t2),
                )*)*
            }

            while let Some(el) = $seq.next_element::<PvValue>()? {
                match el {
                    $($(
                        PvValue::$register(v) => {
                            $app = <$target_type>::$register($app, v);
                        }
                    )*
                    $(
                        PvValue::$register_r(v) => {
                            $app = <$target_type>::$register_r($app, &v);
                        }
                        )*
                    )*
                    $($(
                        PvValue::$register_t(v0, v1) => {
                            $app = <$target_type>::$register_t($app, v0, v1);
                        }                        
                    )*)*
                    $($(
                        PvValue::$register_3t(v0, v1, v2) => {
                            $app = <$target_type>::$register_3t($app, v0, v1, v2);
                        }
                    )*)*
                };
            }
            // #[allow(unused_labels)]
            // 'parse_value_jmp_loop: loop {
            //     break 'parse_value_jmp_loop match key {
            //         $(
            //             $( stringify!($register) => parse_value_inner!($app, $map, $target_type, $value_type, $register), )*
            //             $( stringify!($register_r) => parse_value_inner!($app, $map, $target_type, ref $value_type_r, $register_r), )*
            //         )*
            //         $($(
            //             stringify!($register_t) => {
            //                 let (v0, v1) = $map.next_value::<($value_type_t0, $value_type_t1)>()?;
            //                 <$target_type>::$register_t($app, v0, v1)
            //             }
            //         )*)*
            //         $($(
            //             stringify!($register_3t) => {
            //                 let (v0, v1, v2) = $map.next_value::<($value_type_3t0, $value_type_3t1, $value_type_3t2)>()?;
            //                 <$target_type>::$register_3t($app, v0, v1, v2)
            //             }
            //         )*)*
            //         $($($sp_pat => {$sp_exp})*)*
            //         $($(depr @ ($($dep )|* ) => {return Err(Error::custom(format_args!("deprecated key: {}", depr)))})*)*
            //         $($($(
            //             #[cfg(feature="allow-deprecated")]
            //             $dep_s => {
            //                 const N_KEY : &str = stringify!($dep_d);
            //                 convert_case_to!(N_KEY, key);
            //                 continue 'parse_value_jmp_loop;
            //             },
            //             #[cfg(not(feature="allow-deprecated"))]
            //             $dep_s => {
            //                 return Err(Error::custom(format_args!("deprecated key: {}, use {} insted", stringify!($depr_s), $dep_d)))
            //             },
            //         )*)*)*
            //         unknown => return Err(Error::unknown_field(unknown, &[
            //             $( $( stringify!($register),)*
            //                 $( stringify!($register_r),)*  )*
            //             $($(stringify!($sp_pat),)*)* ]))
            //     }
            // }
        }
    }
}

#[cfg(feature = "snake-case-key")]
macro_rules! convert_case_to {
    ($key:ident, $target: ident) => {{
        $target = $key;
    }};
}

// if const heap is stabilized, should convert the target keys instead.

#[cfg(not(feature = "snake-case-key"))]
macro_rules! convert_case_to {
    ($key:ident, $target: ident) => {
        let cc_xk =
            (<&str as convert_case::Casing<&str>>::to_case(&$key, convert_case::Case::Snake));
        $target = cc_xk.as_str();
    };
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
