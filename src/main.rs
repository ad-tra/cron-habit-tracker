#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod widgets;

use widgets::Habit;
use widgets::Action;
use widgets::ControlledInput;

use std::fs;

use chrono::prelude::*;


use eframe::egui::{self, Layout, RichText, Frame, ScrollArea, Button};
use eframe::egui::style::Margin;
use eframe::egui::{FontId, TextStyle, FontFamily};
use eframe::emath::Align;
use eframe::epaint::{Color32, Rounding, Shadow, Stroke, vec2, pos2};




fn main() {
    let options = eframe::NativeOptions { fullscreen: true, ..Default::default() };
    
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
                for habit in  self.habits.clone().iter()  {habit.show(ui, &mut self.actions, &mut self.habits)};
            
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
    

    let mut visuals = eframe::egui::Visuals { 
        override_text_color: Some(Color32::from_rgb(248, 248, 242)), 
        button_frame : false,
        ..Default::default() 
    };    

    visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(40, 42, 54);

    ctx.set_style(style);
    ctx.set_visuals(visuals);
    ctx.set_fonts(fonts);

}