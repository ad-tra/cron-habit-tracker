#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release



use eframe::egui::{self, Layout, RichText, Frame, Sense, ScrollArea, Button};
use eframe::egui::style::Margin;
use eframe::egui::{FontId, TextStyle, FontFamily};
use eframe::emath::Align;
use eframe::epaint::{Color32, Rounding, Shadow, Stroke, vec2, pos2, Rect};
fn main() {
    let mut options = eframe::NativeOptions::default();
    //options.decorated = true;
    // options.fullscreen =true;

    eframe::run_native(
        "Take Home",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
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

struct MyApp {
    new_habit:  NewHabit,
}
struct NewHabit{
    is_window_visible: bool,
    name: String,
    description: String,
    color: Color32,
    frequency: HabitFrequency,
}
enum HabitFrequency{
    Daily,
    Weekly,
    Monthly,
    Custom
}
impl  MyApp {
    fn new(cc : &eframe::CreationContext<'_>) -> Self {
        configure_custom_theme(&cc.egui_ctx);
        Self {
            new_habit: NewHabit{
                is_window_visible: false,
                color: Color32::from_rgb(139, 233, 253),
                name: "".to_owned(),
                description: "".to_owned(),
                frequency: HabitFrequency::Daily
            }
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

        //TODO height and width are generated with the same logic in mind. Create an abstraction: macro or function
        let window_h_margin = if ctx.available_rect().max.x < 600.0 {ctx.available_rect().max.x * 0.1} else {ctx.available_rect().max.x / 3.0};
        let window_width = ctx.available_rect().max.x - window_h_margin;

                
        let window_v_margin = ctx.available_rect().max.y / 3.0;
        let window_height = ctx.available_rect().max.y - window_v_margin;

        if self.new_habit.is_window_visible {
            
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
                                            self.new_habit.is_window_visible = false
                                        };

                                    });
                                });

                                ui.add_space(30.0);
                                ui.label("name");
                                egui::Frame::default().inner_margin(Margin::symmetric(20.0, 10.0)).fill(Color32::from_rgb(104, 107, 120)).show(ui, |ui|{
                                    
                                    ui.add(egui::TextEdit::singleline(&mut self.new_habit.name).desired_width(f32::INFINITY).frame(false));
                                });
        
                                ui.add_space(20.0);
                                ui.label("description");
                                egui::Frame::default().inner_margin(Margin::symmetric(20.0, 10.0)).fill(Color32::from_rgb(104, 107, 120)).show(ui, |ui|{
                                    
                                    ui.add(egui::TextEdit::singleline(&mut self.new_habit.description).desired_width(f32::INFINITY).frame(false));
                                });
        
                                ui.add_space(20.0);
                                ui.label("color");
                                ui.spacing_mut().interact_size = vec2(ui.available_width(), 45.0);
                                egui::color_picker::color_edit_button_srgba(ui, &mut self.new_habit.color, egui::color_picker::Alpha::Opaque);

                            });
                            ui.with_layout(Layout::bottom_up(Align::Max), |ui|{
                                
                                ui.horizontal(|ui|{
                                    ui.button(RichText::new("Add habit").underline().color(Color32::from_rgb(80,250,123)));
                                    ui.allocate_space(vec2(ui.available_width() * 0.02,0.0));
                                    if ui.button(RichText::new("cancel").color(Color32::from_rgb(195, 195, 195))).clicked(){

                                        self.new_habit.is_window_visible =false;
                                    };

                                })
                            
                            }) 
                        })




                        
                        
                    })

                });
        }

        egui::CentralPanel::default().frame(root_frame).show(ctx, |ui| {
            
            
            
            ui.set_enabled(!self.new_habit.is_window_visible);
            
            
            ui.with_layout(egui::Layout::right_to_left(Align::LEFT), |ui|{
                ui.heading("Your Habit tracker");
            });

            ScrollArea::new([false, true]).show(ui, |ui|{
                
                HabitFrame::new(String::from("Drawing"), String::from("be one with my pencil. learn about anatomy, perspective, and color theory"), Color32::from_rgb(139, 233, 253), 1).show(ui);
                HabitFrame::new(String::from("Read"), String::from("crack the books, learn something new."), Color32::from_rgb(241, 250, 140), 12).show(ui);
            
            });
            
            let bottom_right_post = ui.max_rect().max;
            let bottom_right_rect = Rect{
                min:pos2(bottom_right_post.x - 70.0, bottom_right_post.y - 100.0),
                max: bottom_right_post
            };
            ui.allocate_ui_at_rect(bottom_right_rect, |ui|{
                ui.spacing_mut().button_padding = vec2(20.0,10.0);
                let new_habit_button = ui.add(Button::new(RichText::new("n\ne\nw\n+").size(20.0).color(Color32::from_rgb(30,30,30))).fill(Color32::from_rgb(80,250,123)));
                
                if new_habit_button.clicked() { self.new_habit.is_window_visible = true;}
                
            })
        });
        
    }
    
}








struct HabitFrame{
    heading: String,
    sub_heading: String,
    accent_color: Color32,
    streak: u32,
}

impl  HabitFrame {
    fn new(heading: String, sub_heading:String, accent_color:Color32, streak: u32) ->Self{
        Self{
            heading, sub_heading, accent_color, streak
        }
    }
    fn show(&self, ui : &mut egui::Ui){
        
        Frame::default()
            .inner_margin(Margin::symmetric(40.00, 25.00))
            .fill(Color32::from_rgb(68, 71, 90))
            .outer_margin(Margin::symmetric(0.0, 30.0))
            .show(ui,|ui| { ui.with_layout(Layout::default().with_cross_justify(true), |ui|{


                //this whole dance is to achieve the effect of justify-content:space-between
                //TODO: abstract this into a macro?? it would be nice to shorten this syntax because spacing two elements between each other on a line would be used in high frequency in the future.   
                ui.with_layout(Layout::left_to_right(Align::TOP).with_main_justify(true), |ui|{
                
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui|{
                        ui.heading(&self.heading);
                    });
                    
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui|{
                        
                        ui.label(RichText::new("Day Streak").text_style(TextStyle::Name("Heading2Filled".into())));
                        ui.label(RichText::new(&self.streak.to_string()).text_style(TextStyle::Name("Heading2Regular".into())));
                        ui.label(RichText::new("•••").color(self.accent_color).text_style(TextStyle::Name("Heading2Filled".into())));

                    });
                });
                ui.label(&self.sub_heading);


                ui.allocate_space(vec2(0.0, 20.0));
                CalendarGrid::new(24, 8, self.accent_color).show(ui);

                //TODO refactor using ui.allocate_space instead of frame margin and ui.horizontal instead of ui.with_layout. it will make this block more concise 
                Frame::default()
                .outer_margin(Margin{left:0.0, right:0.0, bottom:0.0, top: 20.0})
                .show(ui,|ui| { ui.with_layout(Layout::left_to_right(Align::Min), |ui|{
                    ui.add(egui::Button::new(RichText::new("Add Entry +").color(self.accent_color).underline()));
                    ui.add_space(15.0);
                    ui.add(egui::Button::new(RichText::new("Tick the day").underline()));
                })});   

                
        })});
    }
}


struct CalendarGrid{
    cols: u32,
    rows: u32,
    done_color: Color32,
}

impl CalendarGrid{
    fn new(cols: u32, rows: u32, done_color: Color32) -> Self {
        Self {
            cols,rows,done_color
        }
    }
    fn show(&self,  ui : &mut egui::Ui){
        for i in 0..self.rows{
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            ui.with_layout(Layout::right_to_left(Align::Min), |ui|{
                for j in 0..self.cols{
                    
                    let rect = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover()).0;
                    ui.painter().rect(rect, Rounding::default(), if (i+j) % 3 == 0 || (j+i) % 12 == 0 {self.done_color} else {Color32::from_rgb(104, 107, 120)}, Stroke::none());

                }
            });
        }
    }
}
