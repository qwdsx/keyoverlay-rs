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

use crate::key::KeyExt;

struct KeyColumn {
    key: Key,
    label: &'static str,
    pressed: bool,
    events: VecDeque<SystemTime>,
}

pub struct App {
    keys: Arc<RwLock<Vec<KeyColumn>>>,
}

impl App {
    pub fn new() -> Self {
        let keys = Arc::new(RwLock::new(
            [Key::KeyZ, Key::KeyX]
                .iter()
                .map(|key| KeyColumn {
                    key: key.to_owned(),
                    label: key.to_str(),
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

                key_column.events.push_front(SystemTime::now());
            })
            .expect("failed to set up listener");
        });

        Self { keys }
    }
}

impl Render for App {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();

        let now = SystemTime::now();
        let key_size = 40.;
        let gap = 12.;
        let scroll_speed = 360.;
        let active_color = 0x808080;
        let padding = 16.;

        div()
            .size_full()
            .flex()
            .p(px(padding))
            .bg(rgb(0x000000))
            .justify_end()
            .items_end()
            .text_color(rgb(0xffffff))
            .child(div().flex().flex_row().gap(px(gap)).children(
                self.keys.read().unwrap().iter().map(|key_column| {
                    let pressed = key_column.pressed;
                    let bg_color = if pressed { active_color } else { 0x000000 };

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
                            .mul(scroll_speed)
                            .floor();

                        let pos = now
                            .duration_since(release_time)
                            .map(|d| d.as_secs_f32())
                            .unwrap_or(0.)
                            .mul(scroll_speed)
                            .floor();

                        if pos > window.viewport_size().height.0 {
                            break;
                        }

                        let block = div()
                            .absolute()
                            .bottom(px(pos + key_size))
                            .w(px(key_size))
                            .h(px(height))
                            .bg(rgb(active_color));

                        blocks.push(block);
                    }

                    div()
                        .flex()
                        .flex_col_reverse()
                        .w(px(key_size))
                        .relative()
                        .child(
                            div()
                                .flex()
                                .size(px(key_size))
                                .bg(rgb(bg_color))
                                .border_2()
                                .border_color(rgb(0xffffff))
                                .justify_center()
                                .items_center()
                                .child(key_column.label),
                        )
                        .children(blocks)
                }),
            ))
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
