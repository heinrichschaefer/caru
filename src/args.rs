use clap::{
    error::ErrorKind, Arg, ArgAction, ArgMatches, Args, Command, Error, FromArgMatches, Parser,
    Subcommand,
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
    /// Reference the Lumberjack entity
    Lumberjack(IdleEntityAmendmentArg),
    /// Reference the Stonemason entity
    Stonemason(IdleEntityAmendmentArg),
    /// Reference the Bowmaker entity
    Bowmaker(IdleEntityAmendmentArg),
    /// Reference the Weaponsmith entity
    Weaponsmith(IdleEntityAmendmentArg),
    /// Reference the Academic entity
    Academic(IdleEntityAmendmentArg),
    /// Reference the Catapult entity
    Catapult(IdleEntityAmendmentArg),
    /// Reference the King entity
    King(IdleEntityAmendmentArg),
}

#[derive(Debug)]
pub struct IdleEntityAmendmentArg {
    pub count: Option<EntityCountArg>,
    pub info: bool,
}

#[derive(Debug)]
pub enum EntityCountArg {
    One,
    All,
    Amount(u32),
}

impl Args for IdleEntityAmendmentArg {
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
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .help("Displays the maximum possible quantity to upgrade and the total cost")
                .action(ArgAction::SetTrue),
        )
        .group(
            clap::ArgGroup::new("entity_amount")
                .args(["one", "all", "amount", "info"])
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
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .help("Displays the maximum possible quantity to upgrade and the total cost")
                .action(ArgAction::SetTrue),
        )
        .group(
            clap::ArgGroup::new("entity_amount")
                .args(["one", "all", "amount", "info"])
                .required(true),
        )
    }
}

impl FromArgMatches for IdleEntityAmendmentArg {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        if matches.get_flag("info") {
            return Ok(Self {
                info: true,
                count: None,
            });
        }

        if matches.get_flag("one") {
            return Ok(Self {
                info: false,
                count: Some(EntityCountArg::One),
            });
        } else if matches.get_flag("all") {
            return Ok(Self {
                info: false,
                count: Some(EntityCountArg::All),
            });
        } else if let Some(n) = matches.get_one::<u32>("amount") {
            return Ok(Self {
                info: false,
                count: Some(EntityCountArg::Amount(*n)),
            });
        };

        Err(Error::new(ErrorKind::ValueValidation))
    }

    fn update_from_arg_matches(&mut self, _matches: &ArgMatches) -> Result<(), Error> {
        Ok(())
    }
}
