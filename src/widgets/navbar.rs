use eframe::{egui::{self, Layout, RichText}, emath::Align, epaint::Color32};

use crate::Panel;

pub struct Navbar;

impl Navbar{
    pub fn show(ui: &mut egui::Ui, app_state: &mut crate::MyApp){
        
        ui.with_layout(Layout::left_to_right(Align::TOP).with_main_justify(true), |ui|{
                
            ui.with_layout(Layout::top_down(Align::LEFT), |ui|{
                
                match app_state.core.get_account(){
                    Ok(account) => {
                        ui.add_space(15.0);
                        ui.label(format!("hello {}", account.username) );                        
                    },
                    Err(_) => {
                        if ui.add(egui::Button::new(RichText::new("login").color(Color32::from_rgb(80,250,123)).underline())).clicked(){
                            app_state.visible_panel = Panel::LoginWindow;
                        }
                    },
                }
            });
            
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                ui.heading("your habit tracker");
            });
            
        
            
        });
    }
}
