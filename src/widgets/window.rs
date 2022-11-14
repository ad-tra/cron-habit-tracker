use eframe::egui::{self, Layout, RichText, Ui};
use eframe::egui::style::Margin;
use eframe::emath::Align;
use eframe::epaint::{Color32, vec2, pos2};


pub struct Window <'a>{
    title: &'a str,
    cta_label: &'a str,
    size: (f32, f32),
    app_state: &'a mut crate::MyApp
}
impl <'a> Window <'a>{
    pub fn new( title: &'a str, cta_label: &'a str, size: (f32,f32), app_state: &'a mut crate::MyApp) -> Self {
        Self {
            title,cta_label, size,app_state
        }
    }

    //receives three callbacks: 
    //on_exit_click: what happens when the user clicks "x" or "cancel"
    //on_cta_click: what happens when the user clicks on the call to action (login, add resource, etc)
    //children: similar to React's children prop. defines ui elements that should be displayed in the window
    pub fn show(&mut self, ctx : &egui::Context ,on_exit_click: impl Fn(&mut crate::MyApp), on_cta_click: impl FnOnce(&mut Ui, &mut crate::MyApp),children: impl FnOnce(&mut Ui, &mut crate::MyApp)){
        match self.app_state.visible_panel{
            crate::Panel::Root => {return;},
            _ => ()
        };


        let window_frame = egui::Frame{
            fill: Color32::from_rgb(68, 71, 90),
            ..Default::default()

        };
        
        //responsive dance. make the window fell width on mobile (max.x < 600)
        //TODO: find out if there is a better way to center a window, doing the calculations on your own can be verbose and error prone.
        let window_h_margin = if ctx.available_rect().max.x < 600.0 {ctx.available_rect().max.x * 0.1} else {ctx.available_rect().max.x / self.size.0};
        let window_width = ctx.available_rect().max.x - window_h_margin;    
        let window_v_margin = ctx.available_rect().max.y /  self.size.1;
        let window_height = ctx.available_rect().max.y - window_v_margin;

        egui::Window::new(String::from(self.title))
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
                                ui.heading(String::from(self.title));

                            });
                            
                            ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                                if ui.button(RichText::new("X").size(40.0).underline().color(Color32::from_rgb(255,85,85))).clicked() { 
                                    on_exit_click(&mut self.app_state);
                                    
                                };
        
                            });
                        });
                        
                        // inject child ui nodes here here
                        children(ui, &mut self.app_state);
                    });
                    ui.with_layout(Layout::bottom_up(Align::Max), |ui|{
                        
                        ui.horizontal(|ui|{
                            if ui.button(RichText::new(String::from(self.cta_label)).underline().color(Color32::from_rgb(80,250,123))).clicked(){

                                on_cta_click(ui, &mut self.app_state);
                                
                            };
                            ui.allocate_space(vec2(ui.available_width() * 0.02,0.0));
                            if ui.button(RichText::new("cancel").color(Color32::from_rgb(195, 195, 195))).clicked(){
        
                                on_exit_click(&mut self.app_state);
                            };
        
                        })
                    
                    }) 
                })  
            })
        });
    } 
}
