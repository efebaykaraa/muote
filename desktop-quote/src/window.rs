use crate::draw::draw_quote;
use gtk::prelude::*;
use marxist_quote_core::config::DisplayArgs;

pub fn run_display(args: DisplayArgs, quote_text: &str, author_text: &str) {
    crate::desktop::force_x11_backend();
    crate::desktop::init_gtk();

    let window = crate::desktop::build_desktop_window("Marxist Quote");
    crate::desktop::enable_transparency(&window);

    let (window_x, window_y, window_width, window_height) =
        display_bounds(&args, !author_text.is_empty());

    let args_draw = args.clone();
    let q_text = quote_text.to_string();
    let a_text = author_text.to_string();

    window.connect_draw(move |_, cr| {
        cr.translate(
            (args_draw.appearance.quote_x - window_x) as f64,
            (args_draw.appearance.quote_y - window_y) as f64,
        );
        draw_quote(cr, &args_draw, &q_text, &a_text);
        false.into()
    });

    window.set_default_size(window_width, window_height);
    window.resize(window_width, window_height);
    window.show_all();
    window.move_(window_x, window_y);

    crate::desktop::run_main();
}

fn display_bounds(args: &DisplayArgs, has_author: bool) -> (i32, i32, i32, i32) {
    let a = &args.appearance;
    let padding = (a.stroke_width.ceil() as i32)
        .max(a.shadow_offset.ceil() as i32)
        .max(16);

    let mut left = a.quote_x;
    let mut top = a.quote_y;
    let mut right = a.quote_x + a.quote_max_width;
    let mut bottom = a.quote_y + a.quote_max_height;

    if has_author {
        let author_width = 360;
        let author_height = ((a.font_size * 1.4).ceil() as i32).max(48);
        left = left.min(a.author_x);
        top = top.min(a.author_y);
        right = right.max(a.author_x + author_width);
        bottom = bottom.max(a.author_y + author_height);
    }

    left -= padding;
    top -= padding;
    right += padding * 2;
    bottom += padding * 2;

    (left, top, (right - left).max(1), (bottom - top).max(1))
}
