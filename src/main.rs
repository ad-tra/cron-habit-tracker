#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod widgets;
mod theme;


use chrono::Utc;

use lockbook_core::Core;
use lockbook_core::CoreLib;
use lockbook_core::SyncAllError;

use lockbook_core::service::api_service::Network;
use lockbook_core::Error as LbError;
use lockbook_core::{ ImportError};

use widgets::Habit;
use widgets::Action;
use widgets::HabitList;
use widgets::Window;
use widgets::ControlledInput;
use widgets::Navbar;



use eframe::egui::{self, Layout, RichText, Frame, Button};
use eframe::egui::style::Margin;
use eframe::emath::Align;
use eframe::epaint::{Color32, vec2};




fn main() {
    let core = Core::init(&lockbook_core::Config{writeable_path:String::from("./"), logs: true, colored_logs: true}).unwrap();
    core_startup(&core);
    
    let options = eframe::NativeOptions { fullscreen: true, ..Default::default() };
    eframe::run_native(
        "Take Home",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc, core))),
    );
}

enum Panel{
    Root,
    LoginWindow,
    HabitWindow,
}
pub struct MyApp {
    new_habit:  Habit,
    habits: Vec<Habit>,
    actions: Vec<Action>,
    visible_panel: Panel,
    auth_token: String,
    core: CoreLib<Network>
}


impl  MyApp {
    fn new(cc : &eframe::CreationContext<'_>, core: CoreLib<Network>) -> Self {
        theme::configure_custom_theme(&cc.egui_ctx);
        MyApp{
            visible_panel: Panel::Root,
            new_habit: Habit::default(),
            //TODO make a core helper fn that reads from file if account is logged in and the call it from habits and actions
            habits:{
                match core.get_account(){
                    Ok(_account) => serde_json::from_str(String::from_utf8( core.read_document(core.get_by_path("/habit-tracker/habits.json").unwrap().id).unwrap()).unwrap().as_str()).unwrap(),
                    Err(_err) => serde_json::from_str("[]").unwrap()
                }
            },
            actions:{
                match core.get_account(){
                    Ok(_account) => serde_json::from_str(String::from_utf8( core.read_document(core.get_by_path("/habit-tracker/actions.json").unwrap().id).unwrap()).unwrap().as_str()).unwrap(),
                    Err(_err) => serde_json::from_str("[]").unwrap()
                }
            },            
            auth_token: String::from(""),
            core,
        }
    }
}

impl eframe::App for MyApp {
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        core_sync(&self.core);
    }
    
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs_f32(60.0 * 5.0)
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


    
        egui::CentralPanel::default().frame(Frame{inner_margin: Margin::symmetric(50.00, 50.00),fill: Color32::from_rgb(40, 42, 54),..Default::default()}).show(ctx, |ui| {
            
            
            match &self.visible_panel{
                Panel::Root => ui.set_enabled(true),
                _ => ui.set_enabled(false)
            }
            Navbar::show(ui, self);
            HabitList::show(ui, self);
            //an absolute positioned vertical button
            if self.core.get_account().is_ok(){ 
                ui.allocate_ui_at_rect(ui.ctx().available_rect(), |ui|{ ui.with_layout(Layout::bottom_up(Align::RIGHT),|ui|{
                    Frame::default().outer_margin(Margin::symmetric(ui.available_width()*0.02, ui.available_height()*0.04)).show(ui, |ui|{
                        
                        ui.spacing_mut().button_padding = vec2(20.0,10.0);
                        if ui.add(Button::new(RichText::new("n\ne\nw\n+").size(20.0).color(Color32::from_rgb(30,30,30))).fill(Color32::from_rgb(80,250,123))).clicked(){
                            self.visible_panel = Panel::HabitWindow;
                        }
                    });
                })});
            }
            else{}

        });
        



        //TODO figure out if  there is  a better way to abstract the window widget. callbacks might be an anti pattern in rust.  
        match self.visible_panel{
            Panel::LoginWindow => {
                Window::new("Login to Core ", "login", (2.0, 2.0), self).show(ctx,
                    |app_state|{ // on exit 
                        app_state.visible_panel = Panel::Root;
                    },|_ui, app_state|{ // on cta click
                    
                        let _a = app_state.core.import_account(app_state.auth_token.as_str()).map_err(|err| match err{
                            LbError::UiError(err) => match err {
                                ImportError::AccountStringCorrupted => println!("Account string corrupted, not imported"),
                                ImportError::AccountExistsAlready =>  println!("Account already exists"),
                                ImportError::AccountDoesNotExist =>  println!("An account with this username was not found on the server"),
                                ImportError::UsernamePKMismatch => println!("The public_key in this account_string does not match what is on the server"),
                                ImportError::CouldNotReachServer => println!("Could not reach server!"),
                                ImportError::ClientUpdateRequired => println!("An update to your application is required to do this action!")
                            },
                            LbError::Unexpected(msg) => println!("{}",msg),
                        });
                        core_sync(&app_state.core);

                        let file_paths   = vec!["/habit-tracker/habits.json", "/habit-tracker/actions.json"];
                        for file_path in file_paths{
                            match &app_state.core.create_at_path(file_path){ //can't start the file with empty content because serde will panic when serializing  
                                Ok(file) => {let _ =&app_state.core.write_document(file.id,"[]".as_bytes()).unwrap();},
                                _ => ()
                            }
                        }
                        core_startup(&app_state.core);
                        app_state.visible_panel = Panel::Root;

                },|ui, app_state|{ //children
                    ControlledInput::from("token",&mut app_state.auth_token).spacing_top(30.0).show(ui);
                });
            },
            Panel::HabitWindow =>{
                Window::new("Add new Habit ", "add habit", (3.0, 3.0), self).show(ctx,|app_state|{
                    app_state.visible_panel = Panel::Root;
        
                }, |_ui,app_state|{
                    app_state.new_habit.created_at = Some(Utc::now());
                    app_state.habits.push(app_state.new_habit.clone());
                    
                    let habits_document = app_state.core.get_by_path("/habit-tracker/habits.json").unwrap();
                    app_state.core.write_document(habits_document.id, serde_json::to_string(&app_state.habits).unwrap().as_bytes()).unwrap();
        
                    app_state.new_habit = Habit::default();
                    app_state.visible_panel = Panel::Root;
                    
                },|ui, app_state|{
                    //TODO: there should be a form abstraction that takes in input names, storage locations, and validation rules and outputs a form that keeps track of submission status, and validation errors. kind of like a mini version of React's Formik 
                    ControlledInput::from("name",&mut app_state.new_habit.name).spacing_top(30.0).show(ui);
                    ControlledInput::from("description", &mut app_state.new_habit.description).show(ui);
                    ControlledInput::from("frequency", &mut app_state.new_habit.frequency).hint_text("* * * * * *").show(ui);
        
                    ui.add_space(20.0);
                    ui.label("color");
                    ui.spacing_mut().interact_size = vec2(ui.available_width(), 45.0);
                    egui::color_picker::color_edit_button_srgba(ui, &mut app_state.new_habit.color, egui::color_picker::Alpha::Opaque);
                });
            }
            Panel::Root => ()
        }





    }
    
}


//TODO move these core helps somewhere else to not clutter main
//init core and create necessary files if they don't exist (habits.json and actions.json)
fn core_startup(core:  &CoreLib<Network>){

    if core.get_account().is_err() {return}

    
    core_sync(&core);

    let file_paths = vec!["/habit-tracker/habits.json", "/habit-tracker/actions.json"];
    for file_path in file_paths{
        match core.create_at_path(file_path){ //can't start the file with empty content because serde will panic when serializing  
            Ok(file) => {core.write_document(file.id,"[]".as_bytes()).unwrap();},
            _ => ()
        }
    }
    core_sync(&core);
    
}

fn core_sync(core: &CoreLib<Network>) {
    core.sync(Some(Box::new(|_sync_progress|{}))).map_err(|err| match err {
        LbError::UiError(err) => match err {
            SyncAllError::Retry => println!("Please retry syncing."),
            SyncAllError::ClientUpdateRequired => println!("update required"),
            SyncAllError::CouldNotReachServer => println!("network issue"),
        },
        LbError::Unexpected(msg) => println!("{}", msg),
    }
    ).ok();
}

//TODO get this to work. current problem is that the complier doesn't know wether the generic type implements serialization or no.
//TODO add better error handling here. refactor unwraps  
// fn core_write_by_path<T>(core: &CoreLib<Network>, path  :&str, data: &Vec<T>) {
//     let document = core.get_by_path(path).unwrap();
//     core.write_document(document.id, serde_json::to_string(data).unwrap().as_bytes()).unwrap();
//}