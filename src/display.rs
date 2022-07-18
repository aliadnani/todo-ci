use chrono::Utc;

use crate::{
    cli::DisplayMode,
    core::{SearchResult, Todo, TodoState},
};

fn print_single(todo: &Todo) {
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
                .date()
                .naive_utc()
                .signed_duration_since(
                    todo.date
                        .expect("Date should be set if TODO is valid/overdue"),
                )
                .num_days();

            bunt::println!("  {$red}Due:{/$} {} ({} days)", date_str, day_difference);
            bunt::println!("  {$cyan}Description:{/$} {}", &todo.description);
        }
        TodoState::Malformed => {
            bunt::println!(
                "{$bold+yellow}TODO@ {$underline} {}:{}{/$} is malformed!{/$}",
                &todo.file.as_path().display().to_string(),
                &todo.line_number
            );
            bunt::println!("  {$red}Description:{/$} {}", &todo.description);
        }
    }

    bunt::println!();
}

pub fn print(mode: DisplayMode, results: &SearchResult) {
    // Total stats
    bunt::println!(
        "{$blue+intense}Searched {} file(s):{/$}",
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

    // Individual TODO details
    bunt::println!();
    if !matches!(mode, DisplayMode::Concise) {
        results.todos.iter().for_each(|todo| {
            if !matches!(mode, DisplayMode::OverdueOnly) {
                print_single(todo)
            }
        })
    }

}
