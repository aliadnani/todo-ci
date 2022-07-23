use std::path::PathBuf;

use chrono::FixedOffset;
use clap::{builder::TypedValueParser, clap_derive::ArgEnum, Parser};
use grep::{
    matcher::{Captures, Matcher},
    regex::RegexMatcher,
};

/// todo-ci: A simple ci tool to check overdue todos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// For disabling ignored files by default (.gitignore, hidden files, etc.)
    #[clap(short = 'n', long = "no-ignore", parse(from_flag))]
    pub no_ignore: bool,

    /// For disabling returning system error code (1) if there are overdue todos
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

    /// Pattern to check `todos` for (i.e. `*.rs` , `main.*`, etc.)
    #[clap(short = 'p', long = "pattern", value_parser, default_value = "*")]
    pub ignore_pattern: String,
    /// Timezone to use for date checking
    #[clap(short = 't', long = "timezone-offset", value_parser = FixedOffsetParser, default_value = "+00:00", allow_hyphen_values = true)]
    pub timezone_offset: FixedOffset,
}

#[derive(ArgEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
pub enum DisplayMode {
    Concise,
    OverdueOnly,
    Default,
    // Verbose,
}

#[derive(Clone)]
struct FixedOffsetParser;

impl TypedValueParser for FixedOffsetParser {
    type Value = FixedOffset;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<FixedOffset, clap::Error> {
        let offset_string = value.to_str().expect("Should be string!");
        const OFFSET_PATTERN: &str = r"(-|\+)(0[0-9]|1[0-9]|2[0-3]):([0-5][0-9])";

        let matcher = RegexMatcher::new(OFFSET_PATTERN).expect("Regex should be valid");
        let mut captures = matcher.new_captures().expect("Regex should be valid");
        matcher
            .captures(offset_string.as_bytes(), &mut captures)
            .expect("Regex should be valid");

        // Regex group match validation
        if matcher.capture_count() != 4 {
            Err(clap::Error::raw(
                clap::ErrorKind::ValueValidation,
                "UTC offset does not follow the format [+|-][HH]:[SS]",
            ))
        } else {
            // Unwraps here are ok - we validated the dates are integers already in the regex
            let offset_seconds: i32 = (3600
                * offset_string[captures.get(2).unwrap()]
                    .parse::<i32>()
                    .unwrap())
                + (60
                    * offset_string[captures.get(3).unwrap()]
                        .parse::<i32>()
                        .unwrap());

            if &offset_string[captures.get(1).unwrap()] == "+" {
                Ok(FixedOffset::east(offset_seconds))
            } else {
                Ok(FixedOffset::west(offset_seconds))
            }
        }
    }

    fn parse(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: std::ffi::OsString,
    ) -> Result<Self::Value, clap::Error> {
        self.parse_ref(cmd, arg, &value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
        None
    }
}
