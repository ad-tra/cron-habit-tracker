use eframe::{epaint::Color32, egui::style::Margin, egui, egui::RichText, };


//TODO: add validation, and make a form struct that spawns controlled inputs
pub struct ControlledInput<'a> {
    label: String,
    spacing_top: f32,
    hint_text: Option<String>,
    state: &'a mut String,
}
impl<'a> ControlledInput<'a>{
    pub fn spacing_top(mut self, spacing_top: f32) -> Self {
        self.spacing_top = spacing_top;
        self
    }
    pub fn hint_text(mut self, hint_text: &str) -> Self {
        self.hint_text = Some(hint_text.to_string());
        self
    }
    pub fn from(label: &str, state: &'a mut String) -> Self {
        ControlledInput { label: String::from(label), spacing_top: 20.0, hint_text: None, state}
    }
    pub fn show(mut self, ui: &mut egui::Ui){
        
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