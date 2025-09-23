use gpui::{
    AppContext, Application, Size, TitlebarOptions, WindowBackgroundAppearance, WindowOptions, px,
};

use app::App;

mod app;
mod key;

fn main() {
    Application::new().run(|cx: &mut gpui::App| {
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("KeyOverlay".into()),
                    ..Default::default()
                }),
                app_id: Some("keyoverlay-rs".to_string()),
                window_background: WindowBackgroundAppearance::Opaque,
                show: true,
                is_resizable: true,
                window_min_size: Some(Size::new(px(128.), px(288.))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| App::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}
