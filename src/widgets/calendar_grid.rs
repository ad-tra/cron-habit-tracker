use crate::widgets::Habit;
use crate::widgets::Action; 

use cron::Schedule;
use chrono::prelude::*;
use eframe::egui::{self, Layout, Sense};
use eframe::emath::Align;
use eframe::epaint::{Color32, Rounding, Stroke, vec2};

pub struct CalendarGrid{
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
    pub fn new(cols: u32, rows: u32, done_color: Color32) -> Self {
        Self {
            cols,rows,done_color
        }
    }
    pub fn show(&self, ui: &mut egui::Ui, habit: &Habit, schedule: Schedule, actions : &mut Vec<Action>) -> bool{
        
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
            // println!("{:#?}", actions);

            let start_range = match fire_times.get(i){
                Some(e) => *e,
                None => {
                    // println!("no start range");
                    break;
                },
            };
            let end_range = match fire_times.get(i+1){
                Some(e) => *e,
                None => {
                    // println!("no end range");
                    break;
                },
            };
            let action = match actions.get(0){
                Some(e) => *e,
                None => {
                    // println!("no action");
                    *cell = Cell::NotDone;
                    continue;
                },
            };


            // println!("start: {:#?}\nend: {:#?}", start_range, end_range);
            
            //check if action date time is contained in the range of two consecutive fire times
            let is_inside_range = action.created_at.signed_duration_since(start_range).num_seconds().is_negative() && action.created_at.signed_duration_since(end_range).num_seconds().is_positive();

            if i == 0 {
                is_action_done = is_inside_range;
            }
            // println!("{}", is_inside_range);
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
        is_action_done
    }
}