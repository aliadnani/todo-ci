use chrono::{FixedOffset, Utc};
use termcolor::WriteColor;

use crate::{
    cli::DisplayMode,
    core::{SearchResult, Todo, TodoState},
};

fn print_single(output_target: &mut dyn WriteColor, todo: &Todo, fixed_offset: &FixedOffset) {
    let mut output_target = output_target;
    match todo.state {
        TodoState::Valid | TodoState::Overdue => {
            if matches!(todo.state, TodoState::Overdue) {
                bunt::writeln!(
                    output_target,
                    "{$bold+red}TODO@ {$underline} {}:{}{/$} [overdue]{/$}",
                    &todo.file.as_path().display().to_string(),
                    &todo.line_number
                )
                .expect("Could not write to output.");
            } else {
                bunt::writeln!(
                    output_target,
                    "{$bold}TODO@{/$} {$underline} {}:{}{/$}",
                    &todo.file.as_path().display().to_string(),
                    &todo.line_number
                )
                .expect("Could not write to output.");
            }

            let date_str = &todo
                .date
                .expect("Date should be set if TODO is valid/overdue")
                .format("%Y-%m-%d")
                .to_string();

            let day_difference = -Utc::now()
                .with_timezone(fixed_offset)
                .date()
                .naive_local()
                .signed_duration_since(
                    todo.date
                        .expect("Date should be set if TODO is valid/overdue"),
                )
                .num_days();

            bunt::writeln!(
                output_target,
                "  {$magenta+dimmed}Due:        {/$} {} ({} days)",
                date_str,
                day_difference
            )
            .expect("Could not write to output.");
            bunt::writeln!(
                output_target,
                "  {$blue+dimmed}Description:{/$} {}",
                &todo.description
            )
            .expect("Could not write to output.");
        }
        TodoState::Malformed => {
            bunt::writeln!(
                output_target,
                "{$bold+yellow}TODO@ {$underline} {}:{}{/$} is malformed!{/$}",
                &todo.file.as_path().display().to_string(),
                &todo.line_number
            )
            .expect("Could not write to output.");
            bunt::writeln!(
                output_target,
                "  {$yellow+dimmed}Description:{/$} {}",
                &todo.description
            )
            .expect("Could not write to output.");
        }
    }

    bunt::writeln!(output_target).expect("Could not write to output.");
}

pub fn print(
    output_target: &mut dyn WriteColor,
    mode: DisplayMode,
    results: &SearchResult,
    fixed_offset: &FixedOffset,
) {
    let mut output_target = output_target;
    // Individual TODO details
    if !matches!(mode, DisplayMode::Concise) {
        results.todos.iter().for_each(|todo| {
            if !matches!(mode, DisplayMode::OverdueOnly) || matches!(todo.state, TodoState::Overdue)
            {
                print_single(&mut output_target, todo, fixed_offset)
            }
        })
    }

    // Total stats
    bunt::writeln!(
        output_target,
        "{$green+intense}Searched {} file(s):{/$}",
        results.statistics.files_searched
    )
    .expect("Could not write to output.");
    bunt::write!(
        output_target,
        "{$bold}{} todo(s) found{/$}",
        results.statistics.valid_todo_count + results.statistics.overdue_todo_count
    )
    .expect("Could not write to output.");

    if results.statistics.overdue_todo_count > 0 {
        bunt::writeln!(
            output_target,
            "{$bold+red} of which {} is/are overdue{/$}",
            results.statistics.overdue_todo_count,
        )
        .expect("Could not write to output.");
    } else {
        bunt::writeln!(output_target).expect("Could not write to output.");
    }
}
