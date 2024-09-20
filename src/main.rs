// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs::File;
use std::time::{Instant, Duration, SystemTime};
use std::cell::RefCell;
use std::rc::Rc;
use csv::Writer;
use chrono::{DateTime, Utc};
use slint::SharedString;
use slint::{ModelRc, VecModel};

slint::include_modules!();


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


fn custom_project_name_dialog(ui_weak: slint::Weak<AppWindow>, 
    parent_project_names_model : Rc<VecModel<SharedString>>,
    open_window_count: Rc<RefCell<usize>>,

) -> Result<(), Box<dyn Error>> {
    let popup = CustomProjectNameDialog::new()?;
    let popup_weak = popup.as_weak();

    *open_window_count.borrow_mut() += 1;

    {
        let popup_weak_clone = popup_weak.clone();
        let ui_weak_clone = ui_weak.clone();
        let open_window_count_clone = open_window_count.clone();

        // dialog ok button pressed
        popup.on_confirm_close_dialog(move || {
            if let (Some(popup), Some(ui)) = (popup_weak_clone.upgrade(), ui_weak_clone.upgrade()) {
                let new_custom_project_name = popup.get_new_custom_project_name();
                parent_project_names_model.push(new_custom_project_name.into());

                popup.window().hide().unwrap();

                // window closed byitself so we decrement open window counter
                *open_window_count_clone.borrow_mut() -= 1;
            }
        });
    }

    {
        let popup_weak_clone = popup_weak.clone();
        let open_window_count_clone = open_window_count.clone();
        // dialog cancel button pressed
        popup.on_cancel_close_dialog(move || {
            if let Some(popup) = popup_weak_clone.upgrade() {
                popup.window().hide().unwrap();

                *open_window_count_clone.borrow_mut() -= 1;
            }
        });
    }

    {
        let open_window_count_clone = open_window_count.clone();
        popup.window().on_close_requested(move || {
            *open_window_count_clone.borrow_mut() -= 1;
            slint::CloseRequestResponse::HideWindow
            
        });
    }


    popup.run()?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();

    // you need to do this for ownership reasons
    let ui_weak = ui.as_weak();
    
    // Rc<Refcell> is used because it's mutable and needs to be used across closures
    let start_time: Rc<RefCell<Option<Instant>>> = Rc::new(RefCell::new(None));
    
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

    let parent_project_names_model : Rc<VecModel<SharedString>> =
    Rc::new(VecModel::from(vec!["Project 1".into(), "Project 2".into(), "Project 3".into()]));
    let parent_project_names_model_rc = ModelRc::from(parent_project_names_model.clone());
    ui.set_parent_project_names_model(parent_project_names_model_rc);
    
    let open_window_count: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    
    {
        let ui_weak_clone = ui_weak.clone();
        let open_window_count_clone = open_window_count.clone();

        ui.on_parent_initialise_custom_project_name_popup(move || {
            let parent_project_names_model_clone = parent_project_names_model.clone();
            let _ = custom_project_name_dialog(ui_weak_clone.clone(), parent_project_names_model_clone, open_window_count_clone.clone());
            

            
        });
    }

    // Close all windows on main window close implementation
    ui.window().on_close_requested(move || {
        println!("window closed requested");
        if *open_window_count.borrow() > 0 {
            println!("windows still open, so keeping app open");
            slint::CloseRequestResponse::KeepWindowShown
        } else {
            println!("closing all windows");
            slint::CloseRequestResponse::HideWindow
        }
        
    });

    ui.run().unwrap();
    Ok(())
}
