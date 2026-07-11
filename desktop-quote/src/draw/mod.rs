use marxist_quote_core::config::{
    DisplayArgs, HorizontalAlign, VerticalAlign, parse_color_to_rgba,
};
use pango::FontDescription;
use pangocairo::functions as pc;

const QUOTE_BG_PADDING_H: f64 = 12.0;
const QUOTE_BG_PADDING_V: f64 = 6.0;
const QUOTE_BG_RADIUS: f64 = 8.0;
const AUTHOR_BG_PADDING_H: f64 = 10.0;
const AUTHOR_BG_PADDING_V: f64 = 4.0;
const AUTHOR_BG_RADIUS: f64 = 6.0;
const AUTHOR_WIDTH: f64 = 360.0;

pub fn draw_quote(cr: &cairo::Context, args: &DisplayArgs, quote: &str, author: &str) {
    let a = &args.appearance;
    let quote_layout = create_quote_layout(cr, args, quote);
    let quote_y = vertical_offset(
        a.quote_max_height as f64,
        layout_pixel_height(&quote_layout),
        a.quote_v_align,
    );

    if a.bg_enabled {
        draw_quote_background(cr, args, &quote_layout, quote_y);
    }

    draw_stroked_layout(cr, args, &quote_layout, 0.0, quote_y);
    draw_shadowed_layout(cr, args, &quote_layout, 0.0, quote_y);
    draw_filled_layout(cr, args, &quote_layout, 0.0, quote_y);

    if !author.is_empty() {
        draw_author(cr, args, author);
    }
}

fn create_quote_layout(cr: &cairo::Context, args: &DisplayArgs, quote: &str) -> pango::Layout {
    let a = &args.appearance;
    let layout = pc::create_layout(cr);
    layout.set_font_description(Some(&font_description(&a.font, a.font_size)));
    layout.set_text(quote);
    layout.set_width(a.quote_max_width * pango::SCALE);
    layout.set_wrap(pango::WrapMode::Word);
    layout.set_alignment(pango_alignment(a.quote_h_align));
    layout
}

fn create_author_layout(cr: &cairo::Context, args: &DisplayArgs, author: &str) -> pango::Layout {
    let a = &args.appearance;
    let layout = pc::create_layout(cr);
    layout.set_font_description(Some(&font_description(&a.font, a.font_size * 0.8)));
    layout.set_text(author);
    layout.set_width((AUTHOR_WIDTH as i32) * pango::SCALE);
    layout.set_alignment(pango_alignment(a.author_h_align));
    layout
}

fn font_description(family: &str, size: f32) -> FontDescription {
    let mut font_desc = FontDescription::new();
    font_desc.set_family(family);
    font_desc.set_size((size as i32) * pango::SCALE);
    font_desc
}

fn draw_quote_background(cr: &cairo::Context, args: &DisplayArgs, layout: &pango::Layout, y: f64) {
    let a = &args.appearance;
    let (r, g, b, alpha) = parse_color_to_rgba(&a.bg_color);
    let radius = if a.bg_rounded { QUOTE_BG_RADIUS } else { 0.0 };

    cr.save().unwrap();
    cr.push_group();
    cr.set_source_rgba(r, g, b, 1.0);

    for mut rect in layout_background_rects(
        layout,
        a.quote_max_width as f64,
        a.bg_fill,
        a.quote_h_align,
        QUOTE_BG_PADDING_H,
        QUOTE_BG_PADDING_V,
    ) {
        rect.y += y;
        if rect.y + rect.height * 0.8 < a.quote_max_height as f64 {
            if radius > 0.0 {
                draw_rounded_rect(cr, rect, radius);
            } else {
                cr.rectangle(rect.x, rect.y, rect.width, rect.height);
            }
            cr.fill().unwrap();
        }
    }

    cr.pop_group_to_source().unwrap();
    cr.paint_with_alpha(alpha as f64).unwrap();
    cr.restore().unwrap();
}

fn draw_author(cr: &cairo::Context, args: &DisplayArgs, author: &str) {
    let a = &args.appearance;
    let layout = create_author_layout(cr, args, author);
    let x = (a.author_x - a.quote_x) as f64;
    let y = (a.author_y - a.quote_y) as f64
        + vertical_offset(
            author_height(args),
            layout_pixel_height(&layout),
            a.author_v_align,
        );

    if a.bg_enabled {
        draw_author_background(cr, args, &layout, x, y);
    }

    draw_stroked_layout(cr, args, &layout, x, y);
    draw_shadowed_layout(cr, args, &layout, x, y);
    draw_filled_layout(cr, args, &layout, x, y);
}

fn draw_author_background(
    cr: &cairo::Context,
    args: &DisplayArgs,
    layout: &pango::Layout,
    x: f64,
    y: f64,
) {
    let a = &args.appearance;
    let (r, g, b, alpha) = parse_color_to_rgba(&a.bg_color);
    let radius = if a.bg_rounded { AUTHOR_BG_RADIUS } else { 0.0 };

    cr.save().unwrap();
    cr.push_group();
    cr.set_source_rgba(r, g, b, 1.0);

    for mut rect in layout_background_rects(
        layout,
        AUTHOR_WIDTH,
        false,
        a.author_h_align,
        AUTHOR_BG_PADDING_H,
        AUTHOR_BG_PADDING_V,
    ) {
        rect.x += x;
        rect.y += y;
        if radius > 0.0 {
            draw_rounded_rect(cr, rect, radius);
        } else {
            cr.rectangle(rect.x, rect.y, rect.width, rect.height);
        }
        cr.fill().unwrap();
    }

    cr.pop_group_to_source().unwrap();
    cr.paint_with_alpha(alpha as f64).unwrap();
    cr.restore().unwrap();
}

fn layout_background_rects(
    layout: &pango::Layout,
    container_width: f64,
    fill: bool,
    align: HorizontalAlign,
    padding_h: f64,
    padding_v: f64,
) -> Vec<Rect> {
    let mut rects = Vec::new();
    let mut iter = layout.iter();

    if fill {
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        let mut max_width: f64 = 0.0;
        let mut first_line = true;

        loop {
            let (_, logical) = iter.line_extents();
            let (ink, _) = iter.line_readonly().unwrap().extents();

            let logical_y = logical.y() as f64 / pango::SCALE as f64;
            let ink_width = ink.width() as f64 / pango::SCALE as f64;
            let logical_height = logical.height() as f64 / pango::SCALE as f64;

            if first_line {
                min_y = logical_y;
                max_y = logical_y + logical_height;
                first_line = false;
            } else {
                min_y = min_y.min(logical_y);
                max_y = max_y.max(logical_y + logical_height);
            }
            max_width = max_width.max(ink_width);

            if !iter.next_line() {
                break;
            }
        }

        let x = aligned_x(container_width, max_width, align);

        rects.push(Rect {
            x: x - padding_h,
            y: min_y - padding_v,
            width: max_width + padding_h * 2.0,
            height: (max_y - min_y) + padding_v * 2.0,
        });
        return rects;
    }

    loop {
        let (_, logical) = iter.line_extents();
        let (ink, _) = iter.line_readonly().unwrap().extents();

        let logical_x = logical.x() as f64 / pango::SCALE as f64;
        let logical_y = logical.y() as f64 / pango::SCALE as f64;
        let ink_width = ink.width() as f64 / pango::SCALE as f64;
        let logical_height = logical.height() as f64 / pango::SCALE as f64;

        let x = if container_width > 0.0 {
            aligned_x(container_width, ink_width, align)
        } else {
            logical_x
        };

        rects.push(Rect {
            x: x - padding_h,
            y: logical_y - padding_v,
            width: ink_width + padding_h * 2.0,
            height: logical_height + padding_v * 2.0,
        });

        if !iter.next_line() {
            break;
        }
    }

    rects
}

fn draw_stroked_layout(
    cr: &cairo::Context,
    args: &DisplayArgs,
    layout: &pango::Layout,
    x: f64,
    y: f64,
) {
    let a = &args.appearance;
    if !a.stroke_enabled {
        return;
    }

    let (r, g, b, alpha) = parse_color_to_rgba(&a.stroke_color);
    cr.move_to(x, y);
    cr.set_source_rgba(r, g, b, alpha);
    cr.set_line_width(a.stroke_width as f64);
    pc::layout_path(cr, layout);
    cr.stroke().unwrap();
}

fn draw_shadowed_layout(
    cr: &cairo::Context,
    args: &DisplayArgs,
    layout: &pango::Layout,
    x: f64,
    y: f64,
) {
    let a = &args.appearance;
    if !a.shadow_enabled {
        return;
    }

    let (r, g, b, alpha) = parse_color_to_rgba(&a.shadow_color);
    let size = a.shadow_size.max(1.0) as f64;
    let blur = a.shadow_blur.max(0.0) as f64;
    let offset = a.shadow_offset as f64;

    if blur <= 0.0 {
        draw_sized_shadow_layout(
            cr,
            args,
            layout,
            x + offset,
            y + offset,
            size,
            r,
            g,
            b,
            alpha,
        );
        return;
    }

    let steps = 8;
    let shadow_alpha = alpha / steps as f64;
    for step in 0..steps {
        let angle = (step as f64 / steps as f64) * std::f64::consts::TAU;
        let dx = angle.cos() * blur;
        let dy = angle.sin() * blur;
        draw_sized_shadow_layout(
            cr,
            args,
            layout,
            x + offset + dx,
            y + offset + dy,
            size,
            r,
            g,
            b,
            shadow_alpha,
        );
    }
}

fn draw_sized_shadow_layout(
    cr: &cairo::Context,
    args: &DisplayArgs,
    layout: &pango::Layout,
    x: f64,
    y: f64,
    size: f64,
    r: f64,
    g: f64,
    b: f64,
    alpha: f64,
) {
    cr.save().unwrap();
    cr.move_to(x, y);
    cr.set_source_rgba(r, g, b, alpha);

    if size > 1.0 {
        pc::layout_path(cr, layout);
        cr.set_line_join(cairo::LineJoin::Round);
        cr.set_line_cap(cairo::LineCap::Round);
        cr.set_line_width(shadow_growth_width(args, size));
        cr.stroke_preserve().unwrap();
        cr.fill().unwrap();
    } else {
        pc::show_layout(cr, layout);
    }

    cr.restore().unwrap();
}

fn shadow_growth_width(args: &DisplayArgs, size: f64) -> f64 {
    let font_size = args.appearance.font_size as f64;
    (font_size * (size - 1.0) * 0.7).max(0.0)
}

fn draw_filled_layout(
    cr: &cairo::Context,
    args: &DisplayArgs,
    layout: &pango::Layout,
    x: f64,
    y: f64,
) {
    let (r, g, b, alpha) = parse_color_to_rgba(&args.appearance.text_color);
    cr.move_to(x, y);
    cr.set_source_rgba(r, g, b, alpha);
    pc::show_layout(cr, layout);
}

fn draw_rounded_rect(cr: &cairo::Context, rect: Rect, radius: f64) {
    cr.new_sub_path();
    cr.arc(
        rect.x + rect.width - radius,
        rect.y + radius,
        radius,
        -std::f64::consts::FRAC_PI_2,
        0.0,
    );
    cr.arc(
        rect.x + rect.width - radius,
        rect.y + rect.height - radius,
        radius,
        0.0,
        std::f64::consts::FRAC_PI_2,
    );
    cr.arc(
        rect.x + radius,
        rect.y + rect.height - radius,
        radius,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
    );
    cr.arc(
        rect.x + radius,
        rect.y + radius,
        radius,
        std::f64::consts::PI,
        -std::f64::consts::FRAC_PI_2,
    );
    cr.close_path();
}

fn pango_alignment(align: HorizontalAlign) -> pango::Alignment {
    match align {
        HorizontalAlign::Left => pango::Alignment::Left,
        HorizontalAlign::Center => pango::Alignment::Center,
        HorizontalAlign::Right => pango::Alignment::Right,
    }
}

fn aligned_x(container_width: f64, content_width: f64, align: HorizontalAlign) -> f64 {
    match align {
        HorizontalAlign::Left => 0.0,
        HorizontalAlign::Center => (container_width - content_width) / 2.0,
        HorizontalAlign::Right => container_width - content_width,
    }
}

fn vertical_offset(container_height: f64, content_height: f64, align: VerticalAlign) -> f64 {
    match align {
        VerticalAlign::Top => 0.0,
        VerticalAlign::Center => ((container_height - content_height) / 2.0).max(0.0),
        VerticalAlign::Bottom => (container_height - content_height).max(0.0),
    }
}

fn layout_pixel_height(layout: &pango::Layout) -> f64 {
    let (_, height) = layout.pixel_size();
    height as f64
}

fn author_height(args: &DisplayArgs) -> f64 {
    ((args.appearance.font_size * 1.4).ceil() as i32).max(48) as f64
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
