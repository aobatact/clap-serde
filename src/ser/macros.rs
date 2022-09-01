macro_rules! ser_value {
    ( $command: ident, $ser: ident, [
            $( $(#[$m:meta])? $([$r:tt])? $({$ex:expr})? ($field: ident, $getter: ident),)+
            ref [ $($(#[$m_ref:meta])? $({$ex_ref:expr})? ($field_ref: ident, $getter_ref: ident)),+ $(,)? ],
            opt [ $($([$r_opt:tt]$({$ex_r:expr})?)?($field_opt: ident, $getter_opt: ident)),+ $(,)? ]

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
            map.serialize_entry(stringify!($field_ref), &$($ex_ref)*($command.$getter_ref()))?;
        })*
        $(
            if let Some(value) = $command.$getter_opt() 
            {
                map.serialize_entry( stringify!($field_opt), $($r_opt)* $($($ex_r)*)*(value))?;
            }
        )*
        
        map.end() }
    };
}
