use clap::{
    error::ErrorKind, Arg, ArgAction, ArgGroup, ArgMatches, Args, Command, Error, FromArgMatches,
    Parser, Subcommand,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CaruArgs {
    #[command(subcommand)]
    pub command: BasicCommand,
}

#[derive(Debug, Subcommand)]
pub enum BasicCommand {
    /// Initializes a new game at <User>/.caru
    Init,
    /// Delete all data relating to the current game instance
    Delete,
    /// Gets the current status of the game
    Status,
    /// Upgrades
    Upgrade(UpgradeEntityArgs),
}

#[derive(Debug, Args)]
pub struct UpgradeEntityArgs {
    /// Choose what entity to upgarde
    #[command(subcommand)]
    pub entity: IdleEntityArg,
}

#[derive(Debug, Subcommand)]
pub enum IdleEntityArg {
    /// Upgrade all possible entities
    All,
    /// Reference the lumberjack entity
    Lumberjack(EntityCountArg),
}

#[derive(Debug)]
pub enum EntityCountArg {
    One,
    All,
    Amount(u32),
}
impl Args for EntityCountArg {
    fn augment_args(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("one")
                .short('o')
                .long("one")
                .help("Sets the amount to 1")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Sets the amount to the maximum possible value")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("amount")
                .short('n')
                .long("amount")
                .value_parser(clap::value_parser!(u32).range(1..))
                .help("Sets the amount to a user defined value")
                .action(ArgAction::Set),
        )
        .group(
            clap::ArgGroup::new("entity_amount")
                .args(["one", "all", "amount"])
                .required(true),
        )
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        cmd.arg(
            Arg::new("one")
                .short('o')
                .long("one")
                .help("Sets the amount to 1")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Sets the amount to the maximum possible value")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("amount")
                .short('n')
                .long("amount")
                .value_parser(clap::value_parser!(u32).range(1..))
                .help("Sets the amount to a user defined value")
                .action(ArgAction::Set),
        )
        .group(
            clap::ArgGroup::new("entity_amount")
                .args(["one", "all", "amount"])
                .required(true),
        )
    }
}

impl FromArgMatches for EntityCountArg {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        if matches.get_flag("one") {
            return Ok(Self::One);
        } else if matches.get_flag("all") {
            return Ok(Self::All);
        }

        if let Some(n) = matches.get_one::<u32>("amount") {
            return Ok(Self::Amount(*n));
        };
        Err(Error::new(ErrorKind::Io))
    }

    fn update_from_arg_matches(&mut self, _matches: &ArgMatches) -> Result<(), Error> {
        Ok(())
    }
}

// #[derive(Debug)]
// pub struct EntityCountArg {
//     pub one: bool,
//     pub all: bool,
//     pub amount: Option<i32>,
// }

// impl Args for EntityCountArg {
//     fn augment_args(cmd: Command) -> Command {
//         cmd.arg(
//             Arg::new("one")
//                 .short('o')
//                 .long("one")
//                 .help("Sets the amount to 1")
//                 .action(ArgAction::SetTrue),
//         )
//         .arg(
//             Arg::new("all")
//                 .short('a')
//                 .long("all")
//                 .help("Sets the amount to the maximum possible value")
//                 .action(ArgAction::SetTrue),
//         )
//         .arg(
//             Arg::new("amount")
//                 .short('n')
//                 .long("amount")
//                 .value_parser(clap::value_parser!(i32).range(1..))
//                 .help("Sets the amount to a user defined value")
//                 .action(ArgAction::Set),
//         )
//         .group(
//             clap::ArgGroup::new("entity_amount")
//                 .args(["one", "all", "amount"])
//                 .required(true),
//         )
//     }

//     fn augment_args_for_update(cmd: Command) -> Command {
//         cmd.arg(
//             Arg::new("one")
//                 .short('o')
//                 .long("one")
//                 .help("Sets the amount to 1")
//                 .action(ArgAction::SetTrue),
//         )
//         .arg(
//             Arg::new("all")
//                 .short('a')
//                 .long("all")
//                 .help("Sets the amount to the maximum possible value")
//                 .action(ArgAction::SetTrue),
//         )
//         .arg(
//             Arg::new("amount")
//                 .short('n')
//                 .long("amount")
//                 .value_parser(clap::value_parser!(i32).range(1..))
//                 .help("Sets the amount to a user defined value")
//                 .action(ArgAction::Set),
//         )
//         .group(
//             clap::ArgGroup::new("entity_amount")
//                 .args(["one", "all", "amount"])
//                 .required(true),
//         )
//     }
// }

// impl FromArgMatches for EntityCountArg {
//     fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
//         Ok(EntityCountArg {
//             one: matches.get_flag("one"),
//             all: matches.get_flag("all"),
//             amount: match matches.get_one("amount") {
//                 Some(n) => *n,
//                 None => None,
//             },
//         })
//     }

//     fn update_from_arg_matches(&mut self, _matches: &ArgMatches) -> Result<(), Error> {
//         Ok(())
//     }
// }
