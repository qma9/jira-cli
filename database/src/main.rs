mod db;
mod io_utils;
mod navigator;
mod ui;

use clearscreen::clear;
use db::JiraDatabase;
use io_utils::{get_user_input, wait_for_key_press};
use navigator::Navigator;
use std::rc::Rc;

fn main() {
    // TODO: create database and navigator
    let database = JiraDatabase::new("database/src/data/db.json".to_string());
    let mut navigator = Navigator::new(Rc::new(database));

    loop {
        clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!("Error rendering page: {error}\nPress any key to continue...");
                wait_for_key_press()
            }
            let user_input = get_user_input();

            match page.handle_input(user_input.trim()) {
                Err(error) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
