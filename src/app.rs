use std::{
    collections::VecDeque,
    ops::Mul,
    sync::{Arc, RwLock},
    thread,
    time::SystemTime,
};

use gpui::{
    Context, Div, IntoElement, Render, Window, div, linear_color_stop, linear_gradient, prelude::*,
    px, rgb, rgba,
};
use rdev::{EventType, Key};

use crate::{config::Config, key::KeyExt};

struct KeyColumn {
    key: Key,
    label: String,
    pressed: bool,
    events: VecDeque<SystemTime>,
}

pub struct App {
    config: Config,
    keys: Arc<RwLock<Vec<KeyColumn>>>,
}

impl App {
    pub fn new(config: Config) -> Self {
        let keys = Arc::new(RwLock::new(
            config
                .keys
                .iter()
                .map(|key| KeyColumn {
                    key: key.key.to_owned(),
                    label: key.label.to_string(),
                    pressed: false,
                    events: VecDeque::with_capacity(64),
                })
                .collect::<Vec<_>>(),
        ));

        let keys_clone = keys.clone();

        thread::spawn(move || {
            rdev::listen(move |event| {
                let (key, pressed) = match event.event_type {
                    EventType::KeyPress(key) => (key, true),
                    EventType::KeyRelease(key) => (key, false),
                    _ => return,
                };

                let mut e = keys_clone.write().unwrap();
                let Some(key_column) = e.iter_mut().find(|e| e.key == key) else {
                    return;
                };

                if pressed == key_column.pressed {
                    return;
                }
                key_column.pressed = pressed;

                if key_column.events.len() >= 64 {
                    key_column.events.pop_back();
                }

                key_column.events.push_front(event.time);
            })
            .expect("failed to set up listener");
        });

        Self { config, keys }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();

        let now = SystemTime::now();

        let Config {
            key_size,
            key_spacing,
            scroll_speed,
            active_color,
            padding,
            ..
        } = self.config;

        div()
            .size_full()
            .flex()
            .p(px(padding as f32))
            .bg(rgb(0x000000))
            .justify_end()
            .items_end()
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(key_spacing as f32))
                    .pb(px(5.))
                    .children(self.keys.read().unwrap().iter().map(|key_column| {
                        let bg_color = if key_column.pressed {
                            rgb(0xffffff)
                        } else {
                            rgb(active_color as u32)
                        };

                        let mut blocks: Vec<Div> = vec![];

                        let mut events_iter = key_column
                            .pressed
                            .then_some(now)
                            .into_iter()
                            .chain(key_column.events.iter().copied());

                        while let Some(release_time) = events_iter.next() {
                            let Some(press_time) = events_iter.next() else {
                                break;
                            };

                            let height = release_time
                                .duration_since(press_time)
                                .map(|d| d.as_secs_f32())
                                .unwrap_or(0.)
                                .mul(scroll_speed as f32)
                                .floor();

                            let pos = now
                                .duration_since(release_time)
                                .map(|d| d.as_secs_f32())
                                .unwrap_or(0.)
                                .mul(scroll_speed as f32)
                                .floor();

                            if pos > window.viewport_size().height.0 {
                                break;
                            }

                            let block = div()
                                .absolute()
                                .bottom(px(pos))
                                .w(px(key_size as f32 * 0.9))
                                .h(px(height))
                                .rounded_sm()
                                .bg(rgb(active_color as u32));

                            blocks.push(block);
                        }

                        div()
                            .flex()
                            .flex_col_reverse()
                            .w(px(key_size as f32))
                            .relative()
                            .children([
                                div()
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .text_lg()
                                    .text_color(rgb(0x606060)),
                                div()
                                    .flex()
                                    .bg(bg_color)
                                    .w(px(key_size as f32))
                                    .h_2()
                                    .rounded_sm()
                                    .justify_center()
                                    .items_center(),
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .children(blocks)
                            ])
                    })),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h_32()
                    .bg(linear_gradient(
                        0.,
                        linear_color_stop(rgba(0x00000000), 0.),
                        linear_color_stop(rgba(0x000000ff), 1.),
                    )),
            )
    }
}
