use std::cell::RefCell;
use serde::Serialize;

macro_rules! ser_value {
    ( $command: ident, $ser: ident, $config: ident, [
            $( $(#[$m:meta])? $([$r:tt])? $({$ex:expr})? ($field: ident, $getter: ident),)*
            is [ $($(#[$m_ref:meta])? ($field_ref: ident, $getter_ref: ident)),+ $(,)? ],
            opt [ $( $(#[$m_opt:meta])? $([$r_opt:tt]$({$ex_r:expr})?)?($field_opt: ident, $getter_opt: ident)),+ $(,)? ]
            $(iter [ $(($field_iter: ident, $expr_iter:expr )),+ ])?
            $( speciallize [ $(($field_sp: ident, $expr_sp: expr)),* ])?
        ]) => {{ 
        let mut map = $ser.serialize_map(None)?;
        let serialize_all = $config.serialize_all();
        $(
        $(#[$m])*
        {
            map.serialize_entry(stringify!($field), $($r)* $($ex)*($command.$getter()))?;
        })*
        $(
        $(#[$m_ref])*
        {
            let flag = $command.$getter_ref();
            if serialize_all || flag {
                map.serialize_entry(stringify!($field_ref), &flag)?;
            }
        })*
        $(
            $(#[$m_opt])*
            {
                if serialize_all
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
            {
                let iter = ($expr_iter)(&$command);
                if serialize_all {
                    map.serialize_entry(stringify!($field_iter), &IterSer::new(iter))?;
                } else {
                    let mut peekable = iter.peekable();
                    if peekable.peek().is_some() {
                        map.serialize_entry(stringify!($field_iter), &IterSer::new(peekable))?;
                    }
                }
            }
        )*)*
        $($(
            map.serialize_entry(stringify!($field_sp), &$expr_sp($command))?;
        )*)*

        map.end() }
    };
}

pub struct IterSer<I>(RefCell<Option<I>>);

impl<I> IterSer<I> {
    pub fn new(iter: I) -> Self { Self (RefCell::new(Some(iter))) }
}


impl<I > Serialize for IterSer<I> where I: IntoIterator, <I as IntoIterator>::Item: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        use serde::ser::Error;
        // should be unreachable unchecked?
        let iter = self.0.borrow_mut().take().ok_or_else(|| S::Error::custom("logic error in IterSer"))?;
        serializer.collect_seq(iter)
    }
}
