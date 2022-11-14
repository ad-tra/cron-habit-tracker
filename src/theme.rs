use eframe::{egui::{self, TextStyle}, epaint::{FontFamily, FontId, Color32}};



//TODO refactor commonly used colors into theme constants ex instead of doing Color32::from(200,200,200), write theme::GRAY
pub fn configure_custom_theme(ctx: &egui::Context) {
    
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