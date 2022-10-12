use chrono::{FixedOffset, Utc};

use crate::{
    cli::DisplayMode,
    core::{SearchResult, Todo, TodoState},
};

fn print_single(todo: &Todo, fixed_offset: &FixedOffset) {
    match todo.state {
        TodoState::Valid | TodoState::Overdue => {
            if matches!(todo.state, TodoState::Overdue) {
                bunt::println!(
                    "{$bold+red}TODO@ {$underline} {}:{}{/$} [overdue]{/$}",
                    &todo.file.as_path().display().to_string(),
                    &todo.line_number
                )
            } else {
                bunt::println!(
                    "{$bold}TODO@{/$} {$underline} {}:{}{/$}",
                    &todo.file.as_path().display().to_string(),
                    &todo.line_number
                )
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

            bunt::println!(
                "  {$magenta+dimmed}Due:        {/$} {} ({} days)",
                date_str,
                day_difference
            );
            bunt::println!("  {$blue+dimmed}Description:{/$} {}", &todo.description);
        }
        TodoState::Malformed => {
            bunt::println!(
                "{$bold+yellow}TODO@ {$underline} {}:{}{/$} is malformed!{/$}",
                &todo.file.as_path().display().to_string(),
                &todo.line_number
            );
            bunt::println!("  {$yellow+dimmed}Description:{/$} {}", &todo.description);
        }
    }

    bunt::println!();
}

pub fn print(mode: DisplayMode, results: &SearchResult, fixed_offset: &FixedOffset) {
    // Individual TODO details
    if !matches!(mode, DisplayMode::Concise) {
        results.todos.iter().for_each(|todo| {
            if !matches!(mode, DisplayMode::OverdueOnly) {
                print_single(todo, fixed_offset)
            }
        })
    }

    // Total stats
    bunt::println!(
        "{$green+intense}Searched {} file(s):{/$}",
        results.statistics.files_searched
    );
    bunt::print!(
        "{$bold}{} todo(s) found{/$}",
        results.statistics.valid_todo_count + results.statistics.overdue_todo_count
    );

    if results.statistics.overdue_todo_count > 0 {
        bunt::println!(
            "{$bold+red} of which {} is/are overdue{/$}",
            results.statistics.overdue_todo_count,
        );
    } else {
        bunt::println!();
    }
}
