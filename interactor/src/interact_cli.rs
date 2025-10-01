use clap::{Args, Parser, Subcommand};

/// Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy")]
    Deploy,

    #[command(name = "upgrade", about = "Upgrade")]
    Upgrade,

    #[command(name = "setRootHash", about = "Set root hash")]
    SetRootHash(RootHashArg),

    #[command(name = "newPoll", about = "Create new poll")]
    NewPoll(NewPollArgs),

    #[command(name = "endPoll", about = "End poll")]
    EndPoll(PollArgs),

    #[command(name = "votePoll", about = "Vote in poll")]
    VotePoll(VotePollArgs),
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct RootHashArg {
    #[arg(short = 'r', long = "hash")]
    pub root_hash: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct NewPollArgs {
    #[arg(short = 'c', long = "address")]
    pub caller: String,

    #[arg(short = 'q', long = "question")]
    pub question: String,

    #[arg(short = 'o', long = "options")]
    pub options: Vec<String>,

    #[arg(short = 'd', long = "duration")]
    pub duration: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PollArgs {
    #[arg(short = 'c', long = "address")]
    pub caller: String,

    #[arg(short = 'i', long = "index")]
    pub index: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PollAndOptionArgs {
    #[arg(short = 'i', long = "index")]
    pub index: u32,

    #[arg(short = 'o', long = "option")]
    pub option: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct VotePollArgs {
    #[arg(short = 'c', long = "address")]
    pub caller: String,

    #[arg(short = 'i', long = "index")]
    pub index: u32,

    #[arg(short = 'o', long = "option")]
    pub option: u32,

    #[arg(short = 'v', long = "votingPower")]
    pub voting_power: u128,

    #[arg(short = 'p', long = "proof", num_args = 0..)]
    pub proof: String,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct ConfirmVotingPowerArgs {
    #[arg(short = 'c', long = "address")]
    pub caller: String,
    #[arg(short = 'v', long = "votingPower")]
    pub voting_power: u128,
    #[arg(short = 'p', long = "proof", num_args = 0..)]
    pub proof: String,
}
