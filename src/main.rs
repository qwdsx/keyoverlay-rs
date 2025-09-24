use gpui::{AppContext, Application, Size, TitlebarOptions, WindowOptions, px};

use app::App;

use crate::config::Config;

mod app;
mod config;
mod key;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    let min_width = config.padding * 2
        + config.keys.len() * config.key_size
        + config.key_spacing * config.keys.len().saturating_sub(1);
    let min_height = 320;

    Application::new().run(move |cx: &mut gpui::App| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("KeyOverlay".into()),
                    ..Default::default()
                }),
                app_id: Some("keyoverlay".to_string()),
                show: true,
                window_min_size: Some(Size::new(px(min_width as f32), px(min_height as f32))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| App::new(config)),
        )
        .unwrap();
        cx.activate(true);
    });

    Ok(())
}
