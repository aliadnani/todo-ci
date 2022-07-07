use chrono::Utc;

use crate::{
    cli::DisplayMode,
    core::{Todo, TotalTodoSearchResult},
};

pub struct Printer {
    mode: DisplayMode,
}

impl Printer {
    pub fn new(display_mode: DisplayMode) -> Printer {
        Printer { mode: display_mode }
    }

    fn print_single(&self, todo: &Todo, overdue: bool) {
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

    pub fn print(&self, results: &TotalTodoSearchResult) {
        // TODO: Tidy up the `println()`s
        bunt::println!();
        bunt::println!("{$blue}Searched {} file(s):{/$}", results.files_searched);
        bunt::print!(
            "{$bold}{} todo(s) found{/$}",
            results.valid_todos.len() + results.overdue_todos.len()
        );

        if results.overdue_todos.len() > 0 {
            bunt::println!(
                "{$bold+red} of which {} is/are overdue{/$}",
                results.overdue_todos.len(),
            );
        } else {
            bunt::println!();
        }

        bunt::println!();

        if !matches!(self.mode, DisplayMode::Concise) {
            if !matches!(self.mode, DisplayMode::OverdueOnly) {
                results
                    .valid_todos
                    .iter()
                    .for_each(|todo| self.print_single(todo, false));
            }
            results
                .overdue_todos
                .iter()
                .for_each(|todo| self.print_single(todo, true));
        }
    }
}
