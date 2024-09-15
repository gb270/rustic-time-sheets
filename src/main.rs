// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use std::cell::RefCell;
use std::rc::Rc;
use csv::Writer;
use chrono::{DateTime, Utc, TimeZone};
use slint::SharedString;

slint::include_modules!();


// TODO: add functionality to enable custom project names
// TODO: add functionality to enable custom price codes
// TODO: might need to add functionality to run multiple tasks at once?

// TODO: change the price codes so that this can all be edited in the app
fn calculate_task_cost(price_code: &str, duration: f64) -> f64 {
    let hourly_rate = match price_code {
        "Price code 1" => 50.0,
        "Price code 2" => 75.0,
        "Price code 3" => 100.0,
        _ => 0.0,
    };
    let hours = duration / 3600.0;
    hours * hourly_rate
}

fn system_time_to_datetime(system_time: SystemTime) -> DateTime<Utc> {
    let datetime: DateTime<Utc> = system_time.into();
    datetime
}

fn format_datetime(datetime: DateTime<Utc>) -> SharedString {
    SharedString::from(datetime.format("%d/%m/%Y %H:%M:%S").to_string())
}


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();

    // you need to do this for ownership reasons
    let ui_weak = ui.as_weak();
    
    // Rc<Refcell> is used because it's mutable and needs to be used across closures
    let start_time: Rc<RefCell<Option<Instant>>> = Rc::new(RefCell::new(None));
    
    // TODO: change this so we add to existing file if it exists
    let file = File::create("timesheet.csv")?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&[
        "Task Name",
        "Project Name",
        "Price Code",
        "Start Time",
        "End Time",
        "Time Taken (seconds)",
        "Cost"
    ])?;

    let application_start_time = SystemTime::now();

    {
        let ui_weak_clone = ui_weak.clone();
        let start_time_clone = start_time.clone();

        ui.on_parent_task_running_changed(move || {
            let ui = ui_weak_clone.unwrap();
            let task_running = ui.get_task_running();

            if task_running {
                *start_time_clone.borrow_mut() = Some(Instant::now());
                println!("Task started at: {}", format_datetime(system_time_to_datetime(application_start_time + start_time_clone.borrow().unwrap().elapsed())));
            } else {
                if let Some(start_time) = *start_time_clone.borrow() {
                    let elapsed_time: Duration = start_time.elapsed();
                    let elapsed_secs = elapsed_time.as_secs_f64();
                    let end_time = Instant::now();
                    
                    println!(
                        "Task stopped. Total time taken: {:.2?} seconds",
                        elapsed_secs
                    );



                    // TODO: fix this, if only default values are used and value isn't actually
                    // selected then nothing will be returned for task, and price code
                    wtr.write_record(&[
                        &ui.get_task_name(),
                        &ui.get_project_name(),
                        &ui.get_price_code(),
                        &format_datetime(system_time_to_datetime(application_start_time + start_time.elapsed())),
                        &format_datetime(system_time_to_datetime(application_start_time + end_time.elapsed())),
                        &format!("{:.2?} seconds", elapsed_secs).into(),
                        &format!("£{:.2?}", calculate_task_cost(&ui.get_price_code(), elapsed_secs)).into(),
                        
                    ]).expect("Failed to write to CSV file");
                } else {
                    println!("Task hasn't been started yet.");
                }
                *start_time_clone.borrow_mut() = None;
            }
        });
    }

    ui.run().unwrap();
    Ok(())
}
