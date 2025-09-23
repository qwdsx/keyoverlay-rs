use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
    thread,
};

use eframe::{
    CreationContext,
    egui::{CentralPanel, Color32, Context, FontFamily, FontId, Frame, Stroke, TextStyle, Vec2},
};
use rdev::{EventType, Key};

use crate::key::KeyExt;

pub struct App {
    keys: Arc<RwLock<HashSet<Key>>>,
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        set_style(&cc.egui_ctx);

        let keys = Arc::new(RwLock::new(HashSet::default()));
        let keys_clone = keys.clone();

        thread::spawn(move || {
            rdev::listen(move |event| match event.event_type {
                EventType::KeyPress(key) => {
                    keys_clone.write().unwrap().insert(key);
                }
                EventType::KeyRelease(key) => {
                    keys_clone.write().unwrap().remove(&key);
                }
                _ => (),
            })
            .expect("failed to set up listener");
        });

        Self { keys }
    }
}

fn set_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [(TextStyle::Body, FontId::new(20.0, FontFamily::Proportional))].into();
    style.spacing.item_spacing = Vec2::splat(24.0);
    style.visuals.override_text_color = Some(Color32::WHITE);
    style.interaction.selectable_labels = false;

    ctx.set_style(style);
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let frame = Frame::default().outer_margin(ctx.style().spacing.item_spacing);

        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.horizontal(|ui| {
                [Key::KeyZ, Key::KeyX].iter().for_each(|key| {
                    let mut frame = Frame::default().stroke(Stroke::new(2.0, Color32::WHITE));

                    let pressed = self.keys.read().map(|hs| hs.contains(key)).unwrap_or(false);
                    if pressed {
                        frame = frame.fill(Color32::GRAY);
                    }

                    frame.show(ui, |ui| {
                        ui.set_width(60.0);
                        ui.set_height(60.0);
                        ui.centered_and_justified(|ui| {
                            ui.label(key.to_str());
                        });
                    });
                });
            })
        });
    }
}
