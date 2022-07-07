use std::path::PathBuf;

use clap::{clap_derive::ArgEnum, Parser};

/// todo-ci: A simple ci tool to check overdue todos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Flag to disable ignored files by default (.gitignore, hidden files, etc.)
    #[clap(short = 'n', long = "no-ignore", parse(from_flag))]
    pub no_ignore: bool,

    /// Flag to disable returning system error code (1) if there are overdue todos
    #[clap(short = 'e', long = "no-error", parse(from_flag))]
    pub no_error: bool,

    /// Display mode:
    ///{n}
    ///- concise: total number of valid + overdue todos {n}
    ///- overdue-only: total number of valid + overdue todos + details of overdue todos {n}
    ///- default: total number of valid + overdue todos + details of all todos {n}
    ///- [PLANNED] detailed: total number of valid + overdue todos + details of all todos with inline code snippet {n}{n}
    #[clap(
        short = 'd',
        long = "display-mode",
        arg_enum,
        default_value = "default"
    )]
    pub display_mode: DisplayMode,

    /// Root directory to check `todos` for
    #[clap(value_parser, default_value = "./")]
    pub root_directory: PathBuf,
    // /// Timezone to use for date checking
    // #[clap(value_parser, default_value = "+00:00")]
    // timezone_offset: UtcOffset,
}

#[derive(ArgEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum DisplayMode {
    Concise,
    OverdueOnly,
    Default,
    // Verbose,
}

// #[derive(Clone)]
// struct UtcOffsetParser;

// impl TypedValueParser for UtcOffsetParser {
//     type Value = UtcOffset;

//     fn parse_ref(
//         &self,
//         cmd: &clap::Command,
//         arg: Option<&clap::Arg>,
//         value: &std::ffi::OsStr,
//     ) -> Result<Self::Value, clap::Error> {

//         let offset = match UtcOffset::from_hms(0, 0, 0) {
//             Ok(offset) => Ok(offset),
//             Err(_) => ,
//         }
//         todo!()
//     }

//     // fn parse(
//     //     &self,
//     //     cmd: &clap::Command,
//     //     arg: Option<&clap::Arg>,
//     //     value: std::ffi::OsString,
//     // ) -> Result<Self::Value, clap::Error> {
//     //     self.parse_ref(cmd, arg, &value)
//     // }

//     // fn possible_values(
//     //     &self,
//     // ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
//     //     None
//     // }
// }
