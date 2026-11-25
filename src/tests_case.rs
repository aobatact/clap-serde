//! Tests which run with every case-key feature.
use crate::{CommandWrap, CommandWrapRef, NoSkip};
use clap::{Arg, ArgAction, Command, ValueHint};

fn command() -> Command {
    Command::new("x")
        .arg(
            Arg::new("path")
                .long("path")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("os")
                .long("os")
                .value_parser(clap::value_parser!(std::ffi::OsString)),
        )
        .arg(
            Arg::new("num")
                .long("num")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(Arg::new("flag").long("flag").action(ArgAction::SetTrue))
}

#[test]
fn roundtrip_with_case_feature() {
    let c = command();
    for json in [
        serde_json::to_value(CommandWrapRef::new(&c)).unwrap(),
        serde_json::to_value(CommandWrapRef::new(&c).with_config(NoSkip)).unwrap(),
    ] {
        let c2: Command = serde_json::from_value::<CommandWrap>(json.clone())
            .unwrap()
            .into();
        assert_eq!(serde_json::to_value(CommandWrapRef::new(&c2)).unwrap(), {
            serde_json::to_value(CommandWrapRef::new(&c)).unwrap()
        });
        let m = c2
            .try_get_matches_from(["x", "--path", "a", "--os", "b", "--num", "3", "--flag"])
            .unwrap();
        assert_eq!(m.get_one::<u32>("num"), Some(&3));
        assert!(m.get_flag("flag"));
    }
}
