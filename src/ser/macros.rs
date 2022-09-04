macro_rules! ser_value {
    ( $command: ident, $ser: ident, $config: ident, [
            $( $(#[$m:meta])? $([$r:tt])? $({$ex:expr})? ($field: ident, $getter: ident),)*
            is [ $($(#[$m_ref:meta])? ($field_ref: ident, $getter_ref: ident)),+ $(,)? ],
            opt [ $( $(#[$m_opt:meta])? $([$r_opt:tt]$({$ex_r:expr})?)?($field_opt: ident, $getter_opt: ident)),+ $(,)? ]
            $( speciallize [ $(($field_sp: ident, $expr_sp: expr)),* ])?
        ]) => {
        { let mut map = $ser.serialize_map(None)?;
        $(
        $(#[$m])*
        {
            map.serialize_entry(stringify!($field), $($r)* $($ex)*($command.$getter()))?;
        })*
        $(
        $(#[$m_ref])*
        {
            let flag = $command.$getter_ref();
            if $config || flag {
                map.serialize_entry(stringify!($field_ref), &flag)?;
            }
        })*
        $(
            $(#[$m_opt])*
            {
                if $config
                {
                    map.serialize_entry( stringify!($field_opt), &($command.$getter_opt())$(.map(|v|$($ex_r)*(v)))*)?;
                }
                else {
                    if let Some(value) = $command.$getter_opt()
                    {
                        map.serialize_entry( stringify!($field_opt), $($r_opt)* $($($ex_r)*)*(value))?;
                    }
                }
            }
        )*
        $($(
            map.serialize_entry(stringify!($field_sp), &$expr_sp($command))?;
        )*)*

        map.end() }
    };
}
