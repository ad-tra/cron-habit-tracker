#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use cron::Schedule;
use uuid::Uuid;
use std::fs;
use std::str::FromStr;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use eframe::egui::{self, Layout, RichText, Frame, Sense, ScrollArea, Button};
use eframe::egui::style::Margin;
use eframe::egui::{FontId, TextStyle, FontFamily};
use eframe::emath::Align;
use eframe::epaint::{Color32, Rounding, Shadow, Stroke, vec2, pos2};
fn main() {
    let mut options = eframe::NativeOptions::default();
    //options.decorated = false;
    options.fullscreen =true;

    eframe::run_native(
        "Take Home",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}


struct MyApp {
    new_habit:  Habit,
    habits: Vec<Habit>,
    actions: Vec<Action>,
    is_new_habit_window_visible: bool,
}

impl  MyApp {
    fn new(cc : &eframe::CreationContext<'_>) -> Self {
        configure_custom_theme(&cc.egui_ctx);

        Self {
            is_new_habit_window_visible: false,
            new_habit: Habit::default(),
            habits: serde_json::from_reader(fs::File::open("habits.json").unwrap()).unwrap(),
            actions:  serde_json::from_reader(fs::File::open("actions.json").unwrap()).unwrap()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        


        let root_frame = egui::Frame {
            inner_margin: Margin::symmetric(50.00, 50.00),
            outer_margin: Margin::same(0.0),
            rounding: Rounding::same(0.0),
            shadow: Shadow{extrusion: 0.0, color: Color32::BLACK},
            fill: Color32::from_rgb(40, 42, 54),
            stroke: Stroke::none() 
        };
        let window_frame = egui::Frame{
            fill: Color32::from_rgb(68, 71, 90),
            inner_margin: Margin::same(0.0),
            ..root_frame
        };


        let window_h_margin = if ctx.available_rect().max.x < 600.0 {ctx.available_rect().max.x * 0.1} else {ctx.available_rect().max.x / 3.0};
        let window_width = ctx.available_rect().max.x - window_h_margin;

                
        let window_v_margin = ctx.available_rect().max.y / 3.0;
        let window_height = ctx.available_rect().max.y - window_v_margin;

        if self.is_new_habit_window_visible {
            
            egui::Window::new("add habit window")
                .frame(window_frame)
                .fixed_pos(pos2(window_h_margin/2.0, window_v_margin/2.0))
                .title_bar(false)
                .resizable(false)
                .show(ctx, |ui|{

                    ui.set_height(window_height);
                    ui.set_width(window_width);
                    egui::Frame::default().inner_margin(Margin::symmetric(50.0, 40.0)).show(ui, |ui|{

                        ui.with_layout(Layout::top_down(Align::LEFT).with_main_justify(true), |ui|{
                            ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
                               
                                ui.with_layout(Layout::left_to_right(Align::TOP).with_main_justify(true), |ui|{
                        
                                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                                        ui.heading("Add a new Habit");
                                    });
                                    
                                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                                        if ui.button(RichText::new("X").size(40.0).underline().color(Color32::from_rgb(255,85,85))).clicked() {
                                            self.is_new_habit_window_visible = false
                                        };

                                    });
                                });

                                ControlledInput::from("name",&mut self.new_habit.name).spacing_top(30.0).show(ui);
                                ControlledInput::from("description", &mut self.new_habit.description).show(ui);
                                ControlledInput::from("frequency", &mut self.new_habit.frequency).hint_text("* * * * * *").show(ui);

                                ui.add_space(20.0);
                                ui.label("color");
                                ui.spacing_mut().interact_size = vec2(ui.available_width(), 45.0);
                                egui::color_picker::color_edit_button_srgba(ui, &mut self.new_habit.color, egui::color_picker::Alpha::Opaque);

                            });
                            ui.with_layout(Layout::bottom_up(Align::Max), |ui|{
                                
                                ui.horizontal(|ui|{
                                    if ui.button(RichText::new("Add habit").underline().color(Color32::from_rgb(80,250,123))).clicked(){

                                        self.new_habit.created_at = Some(Utc::now());
                                        self.habits.push(self.new_habit.clone());
                                        

                                        //TODO: instead of deserializing manually here, impl a Deserializer for app state.  
                                        fs::write("habits.json", serde_json::to_string(&self.habits).unwrap()).expect("should be able to write content to habits.json");

                                        self.new_habit = Habit::default();
                                        self.is_new_habit_window_visible =false;
                                    
                                    };
                                    ui.allocate_space(vec2(ui.available_width() * 0.02,0.0));
                                    if ui.button(RichText::new("cancel").color(Color32::from_rgb(195, 195, 195))).clicked(){

                                        self.is_new_habit_window_visible =false;
                                    };

                                })
                            
                            }) 
                        })




                        
                        
                    })

                });
        }

        egui::CentralPanel::default().frame(root_frame).show(ctx, |ui| {
            
            
            
            ui.set_enabled(!self.is_new_habit_window_visible);
            
            
            ui.with_layout(egui::Layout::right_to_left(Align::LEFT), |ui|{
                ui.heading("your habit tracker");
            });

            ScrollArea::new([false, true]).show(ui, |ui|{
                for habit in self.habits.iter()  {habit.show(ui, &mut self.actions)};
            
            });
            

        ui.allocate_ui_at_rect(ui.ctx().available_rect(), |ui|{
            ui.with_layout(Layout::bottom_up(Align::RIGHT),|ui|{
                Frame::default().outer_margin(Margin::symmetric(ui.available_width()*0.02, ui.available_height()*0.04)).show(ui, |ui|{
                    
                    ui.spacing_mut().button_padding = vec2(20.0,10.0);
                    if ui.add(Button::new(RichText::new("n\ne\nw\n+").size(20.0).color(Color32::from_rgb(30,30,30))).fill(Color32::from_rgb(80,250,123))).clicked(){
                        self.is_new_habit_window_visible = true;
                    }    
                });
            })
                
            })
        });
        
    }
    
}
#[derive(Serialize, Deserialize, Debug,  Clone)]
struct Habit{
    id: Uuid,
    name: String,
    description: String,
    color: Color32,
    streak: u32,
    frequency: String,
    created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Action{
    habit_id: Uuid,
    created_at: DateTime<Utc>
}

impl Habit{
    fn default() -> Self {
        Habit{
            id: Uuid::new_v4(),
            color: Color32::from_rgb(139, 233, 253),
            name: "".to_owned(),
            description: "".to_owned(),
            streak: 0,
            frequency: "".to_owned(),
            created_at: None,
        }
    }
    fn show(&self, ui : &mut egui::Ui, actions: &mut Vec<Action>){
    

        Frame::default()
            .inner_margin(Margin::symmetric(40.00, 25.00))
            .fill(Color32::from_rgb(68, 71, 90))
            .outer_margin(Margin::symmetric(0.0, 30.0))
            .show(ui,|ui| { ui.with_layout(Layout::default().with_cross_justify(true), |ui|{


                //this whole dance is to achieve the effect of justify-content:space-between
                //TODO: abstract this into a macro?? it would be nice to shorten this syntax because spacing two elements between each other on a line would be used in high frequency in the future.   
                ui.with_layout(Layout::left_to_right(Align::TOP).with_main_justify(true), |ui|{
                
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                        ui.heading(&self.name);
                    });
                    
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                        
                        ui.label(RichText::new("Day Streak").text_style(TextStyle::Name("Heading2Filled".into())));
                        ui.label(RichText::new(self.streak.to_string()).text_style(TextStyle::Name("Heading2Regular".into())));
                        ui.label(RichText::new("•••").color(self.color).text_style(TextStyle::Name("Heading2Filled".into())));

                    });
                });
                ui.label(&self.description);

                
                ui.allocate_space(vec2(0.0, 20.0));
                let is_action_done = CalendarGrid::new(24, 8, self.color).show(ui, self, Schedule::from_str(self.frequency.as_str()).unwrap(), actions);

                //TODO refactor using ui.allocate_space instead of frame margin and ui.horizontal instead of ui.with_layout. it will make this block more concise 
                Frame::default()
                .outer_margin(Margin{left:0.0, right:0.0, bottom:0.0, top: 20.0})
                .show(ui,|ui| { ui.with_layout(Layout::left_to_right(Align::Min), |ui|{

                    
                    let mark_done = ui.add(egui::Button::new(RichText::new("mark done!").color(self.color.linear_multiply(if is_action_done {0.2} else {1.0})).underline()));

                    if mark_done.clicked() && !is_action_done{ 
                        actions.push(Action { habit_id: self.id, created_at: Utc::now() });
                        fs::write("actions.json", serde_json::to_string(&actions).unwrap()).expect("should be able to write content to habits.json");
                    }
                    if mark_done.hovered() && is_action_done {
                        egui::show_tooltip_text(ui.ctx(), egui::Id::new("my_tooltip"), " you  did this. Good Job!");

                    }
                    ui.add_space(15.0);
                    if ui.add(egui::Button::new(RichText::new("update").underline())).hovered(){

                        egui::show_tooltip_text(ui.ctx(), egui::Id::new("update_not_implemented"), "not implemented yet");
                    }

                })});   

                
        })});
    }
}




//TODO: add validation, and make a form struct that spawns controlled inputs
struct ControlledInput<'a> {
    label: String,
    spacing_top: f32,
    hint_text: Option<String>,
    state: &'a mut String,
}
impl<'a> ControlledInput<'a>{
    fn spacing_top(mut self, spacing_top: f32) -> Self {
        self.spacing_top = spacing_top;
        self
    }
    fn hint_text(mut self, hint_text: &str) -> Self {
        self.hint_text = Some(hint_text.to_string());
        self
    }
    fn from(label: &str, state: &'a mut String) -> Self {
        ControlledInput { label: String::from(label), spacing_top: 20.0, hint_text: None, state}
    }
    fn show(mut self, ui: &mut egui::Ui){
        
        ui.add_space(self.spacing_top);
        ui.label(&self.label);
        
        egui::Frame::default().inner_margin(Margin::symmetric(20.0, 10.0)).fill(Color32::from_rgb(104, 107, 120)).show(ui, |ui|{
            if self.hint_text.is_none(){
                let mut text = "enter ".to_string();
                text.push_str(&self.label); 
                self.hint_text = Some(text);
            }            


            ui.add(egui::TextEdit::singleline(self.state).hint_text(RichText::from(self.hint_text.unwrap()).color(Color32::from_rgba_unmultiplied(248, 248, 242, 75))).desired_width(f32::INFINITY).frame(false));
        });   
    }
}




struct CalendarGrid{
    cols: u32,
    rows: u32,
    done_color: Color32,
}

#[derive(Clone,PartialEq)]
enum Cell {
    Done,
    NotDone,
    OutOfBounds,
}
impl CalendarGrid{
    fn new(cols: u32, rows: u32, done_color: Color32) -> Self {
        Self {
            cols,rows,done_color
        }
    }
    fn show(&self, ui: &mut egui::Ui, habit: &Habit, schedule: Schedule, actions : &mut Vec<Action>) -> bool{
        
        //fire_times that don't exceed the present + one in the future.
        let fire_times  = schedule.after(&habit.created_at.unwrap()).take_while(|&x| Utc::now().signed_duration_since(x).num_seconds().is_positive()).count()+1;
        let mut fire_times: Vec<DateTime<Utc>> = schedule.after(&habit.created_at.unwrap()).take(fire_times).collect(); 
        fire_times.reverse();
        fire_times.push(habit.created_at.unwrap());
        


        let mut actions : Vec<&Action> =  actions.iter().filter(|x| x.habit_id == habit.id).collect();
        actions.sort_by(|a,b| b.created_at.cmp(&a.created_at));

        let available_slots = self.rows * self.cols;
        let mut flat_grid : Vec<Cell> = vec![Cell::OutOfBounds; available_slots  as usize];
        

        if fire_times.len() > available_slots as usize {
            todo!("the habit has been running for a long time. it exceeds the available slots, TODO add a scrolling mechanism between slot-windows");
        };

        let mut  is_action_done = false;

        for (i, cell) in flat_grid.iter_mut().enumerate(){
            println!("{:#?}", actions);

            let start_range = match fire_times.get(i){
                Some(e) => *e,
                None => {
                    println!("no start range");
                    break;
                },
            };
            let end_range = match fire_times.get(i+1){
                Some(e) => *e,
                None => {
                    println!("no end range");
                    break;
                },
            };
            let action = match actions.get(0){
                Some(e) => *e,
                None => {
                    println!("no action");
                    *cell = Cell::NotDone;
                    continue;
                },
            };


            println!("start: {:#?}\nend: {:#?}", start_range, end_range);
            
            //check if action date time is contained in the range of two consecutive fire times
            let is_inside_range = action.created_at.signed_duration_since(start_range).num_seconds().is_negative() && action.created_at.signed_duration_since(end_range).num_seconds().is_positive();

            if i == 0 {
                is_action_done = is_inside_range;
            }
            println!("{}", is_inside_range);
            if is_inside_range {
                *cell = Cell::Done;
                actions.remove(0);
            }
            else{
                *cell = Cell::NotDone;
            }
        }


        //render the flat_grid into a grid. uses some index arithmetic  
        for i in 0..self.rows{
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            ui.with_layout(Layout::right_to_left(Align::Min), |ui|{
                for j in 0..self.cols{
                    
                    let rect = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover()).0;
                    
                    let cell_color =  match flat_grid[(i *self.cols ) as usize + j as usize]{
                        Cell::Done => self.done_color,
                        Cell::NotDone => Color32::from_rgb(104, 107, 120),
                        Cell::OutOfBounds => Color32::from_rgb(104, 107, 120).linear_multiply(0.2),
                    };

                    ui.painter().rect(rect, Rounding::default(), cell_color, Stroke::none());

                }
            });
        }
        return is_action_done;
    }
}

//fonts, and colors
fn configure_custom_theme(ctx: &egui::Context) {
    
    let mut fonts = egui::FontDefinitions::default();
    let mut style = (*ctx.style()).clone();


    fonts.font_data.insert("sabo_filled".to_owned(),egui::FontData::from_static(include_bytes!("../assets/fonts/Sabo-Filled.otf")));
    fonts.font_data.insert("sabo_regular".to_owned(), egui::FontData::from_static(include_bytes!("../assets/fonts/Sabo-Regular.otf")));
    fonts.font_data.insert("cascadia_regular".to_owned(), egui::FontData::from_static(include_bytes!("../assets/fonts/Cascadia-Regular.otf")));
    
    fonts.families.entry(FontFamily::Name("Sabo".into())).or_default().push("sabo_filled".to_owned());
    fonts.families.entry(FontFamily::Name("SaboRegular".into())).or_default().push("sabo_regular".to_owned());
    fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "cascadia_regular".to_owned());



    style.text_styles = [
        (TextStyle::Heading, FontId::new(50.0, FontFamily::Name(("Sabo").into()))),
        (TextStyle::Name("Heading2Filled".into()), FontId::new(30.0, FontFamily::Name(("Sabo").into()))),
        (TextStyle::Name("Heading2Regular".into()), FontId::new(30.0, FontFamily::Name(("SaboRegular").into()))),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),
        (TextStyle::Monospace, FontId::new(18.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(25.0,  FontFamily::Name(("Sabo").into()))),
        (TextStyle::Small, FontId::new(12.0, FontFamily::Monospace)),
    ].into();
    

    let mut visuals = egui::Visuals::default();
    visuals.override_text_color = Some(Color32::from_rgb(248, 248, 242));
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(40, 42, 54);
    visuals.button_frame = false;
    

    ctx.set_style(style);
    ctx.set_visuals(visuals);
    ctx.set_fonts(fonts);

}