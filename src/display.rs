use chrono::Utc;

use crate::{
    cli::DisplayMode,
    core::{Todo, TotalSearchResult},
};

fn print_single(todo: &Todo, overdue: bool) {
    if overdue {
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

    let date_str = &todo.date.format("%Y-%m-%d").to_string();
    let day_difference = -Utc::now()
        .date()
        .naive_utc()
        .signed_duration_since(todo.date)
        .num_days();

    bunt::println!("  {$red}Due:{/$} {} ({} days)", date_str, day_difference);
    bunt::println!("  {$cyan}Description:{/$} {}", &todo.description);

    bunt::println!();
}

pub fn print(mode: DisplayMode, results: &TotalSearchResult) {

    // Total stats
    bunt::println!("{$blue}Searched {} file(s):{/$}", results.files_searched);
    bunt::print!(
        "{$bold}{} todo(s) found{/$}",
        results.valid_todos.len() + results.overdue_todos.len()
    );

    if !results.overdue_todos.is_empty() {
        bunt::println!(
            "{$bold+red} of which {} is/are overdue{/$}",
            results.overdue_todos.len(),
        );
    } else {
        bunt::println!();
    }

    bunt::println!();

    if !matches!(mode, DisplayMode::Concise) {
        if !matches!(mode, DisplayMode::OverdueOnly) {
            results
                .valid_todos
                .iter()
                .for_each(|todo| print_single(todo, false));
        }
        results
            .overdue_todos
            .iter()
            .for_each(|todo| print_single(todo, true));
    }
}
