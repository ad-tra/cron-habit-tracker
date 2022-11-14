use super::CalendarGrid;
use cron::Schedule;
use lockbook_core::service::api_service::Network;
use uuid::Uuid;
use std::str::FromStr;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use lockbook_core::CoreLib;

use eframe::egui::{self, Layout, RichText, Frame, ScrollArea};
use eframe::egui::style::Margin;
use eframe::egui::{TextStyle};
use eframe::emath::Align;
use eframe::epaint::{Color32, vec2};


pub struct  HabitList;

impl HabitList {
    pub fn show(ui: &mut egui::Ui, app_state: &mut crate::MyApp){
        ScrollArea::new([false, true]).show(ui, |ui|{
            for habit in  app_state.habits.clone().iter()  {habit.show(ui, &mut app_state.actions, &mut app_state.habits, &app_state.core)};
        });
    }
}

#[derive(Serialize, Deserialize, Debug,  Clone)]
pub struct Habit{
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub color: Color32,
    pub streak: u32,
    pub frequency: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Action{
    pub habit_id: Uuid,
    pub created_at: DateTime<Utc>
}

impl Habit{
    pub fn default() -> Self {
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
    pub fn show(&self, ui : &mut egui::Ui, actions: &mut Vec<Action>, habits: &mut Vec<Habit>, core : &CoreLib<Network>){
    

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

                        
                        let actions_document = core.get_by_path("/habit-tracker/actions.json").unwrap();
                        core.write_document(actions_document.id, serde_json::to_string(&actions).unwrap().as_bytes()).unwrap();
                    }
                    if mark_done.hovered() && is_action_done {
                        egui::show_tooltip_text(ui.ctx(), egui::Id::new("my_tooltip"), " you  did this. Good Job!");

                    }


                    ui.add_space(15.0);
                    if ui.add(egui::Button::new(RichText::new("del").underline())).clicked(){

                        //TODO: add an are you sure confirmation modal.
                        habits.remove(habits.iter().position(|x| x.id.eq(&self.id)).unwrap());
                        
                        //cascade delete actions                        
                        actions.retain(|&x| x.habit_id != self.id);
                        println!("{:#?}", actions);
                        let actions_document = core.get_by_path("/habit-tracker/actions.json").unwrap();
                        core.write_document(actions_document.id, serde_json::to_string(&actions).unwrap().as_bytes()).unwrap();
                        
                        let habits_document = core.get_by_path("/habit-tracker/habits.json").unwrap();
                        core.write_document(habits_document.id, serde_json::to_string(&habits).unwrap().as_bytes()).unwrap();
                    }


                })});   

                
        })});
    }
}