use crate::state::{PREVIEW_QUOTE, State};
use engyls::config::{HorizontalAlign, VerticalAlign, parse_color_to_rgba};
use pango::FontDescription;
use pangocairo::functions as pc;

pub fn draw_quote(cr: &cairo::Context, s: &mut State) {
    let a = &s.args.appearance;
    let (r, g, b, alpha) = parse_color_to_rgba(&a.text_color);
    let (bg_r, bg_g, bg_b, bg_a) = parse_color_to_rgba(&a.bg_color);

    let qx = a.quote_x as f64;
    let qy = a.quote_y as f64;
    let qw = a.quote_max_width as f64;
    let qh = a.quote_max_height as f64;
    let quote_y_offset;

    let is_dragging_q = s.drag_mode == crate::events::DragMode::MoveQuote;

    // ── Quote background ──
    if a.bg_enabled {
        let padding_h = 12.0;
        let padding_v = 6.0;
        let radius = if a.bg_rounded { 8.0 } else { 0.0 };

        cr.save().unwrap();
        cr.push_group();
        cr.set_source_rgba(bg_r, bg_g, bg_b, 1.0);

        let quote_layout_bg = pc::create_layout(cr);
        let mut q_font_bg = FontDescription::new();
        q_font_bg.set_family(&a.font);
        q_font_bg.set_size((a.font_size as i32) * pango::SCALE);
        quote_layout_bg.set_font_description(Some(&q_font_bg));
        quote_layout_bg.set_text(PREVIEW_QUOTE);
        quote_layout_bg.set_width(a.quote_max_width * pango::SCALE);
        quote_layout_bg.set_wrap(pango::WrapMode::Word);
        quote_layout_bg.set_alignment(pango_alignment(a.quote_h_align));
        quote_y_offset =
            vertical_offset(qh, layout_pixel_height(&quote_layout_bg), a.quote_v_align);

        if a.bg_fill {
            // Fill mode: single container for all lines
            let mut min_y = f64::MAX;
            let mut max_y = f64::MIN;
            let mut max_width: f64 = 0.0;

            let mut iter = quote_layout_bg.iter();
            loop {
                let (_, logical) = iter.line_extents();
                let (ink, _) = iter.line_readonly().unwrap().extents();

                let lw = (ink.width() as f64) / pango::SCALE as f64;
                let lh = (logical.height() as f64) / pango::SCALE as f64;
                let ly = (logical.y() as f64) / pango::SCALE as f64;

                if ly + lh * 0.8 < qh {
                    min_y = min_y.min(ly);
                    max_y = max_y.max(ly + lh);
                    max_width = max_width.max(lw);
                }

                if !iter.next_line() {
                    break;
                }
            }

            if min_y != f64::MAX {
                let bx = qx + aligned_x(qw, max_width, a.quote_h_align) - padding_h;
                let by = qy + quote_y_offset + min_y - padding_v;
                let bw = max_width + padding_h * 2.0;
                let bh = (max_y - min_y) + padding_v * 2.0;

                if radius > 0.0 {
                    cr.new_sub_path();
                    cr.arc(
                        bx + bw - radius,
                        by + radius,
                        radius,
                        -std::f64::consts::FRAC_PI_2,
                        0.0,
                    );
                    cr.arc(
                        bx + bw - radius,
                        by + bh - radius,
                        radius,
                        0.0,
                        std::f64::consts::FRAC_PI_2,
                    );
                    cr.arc(
                        bx + radius,
                        by + bh - radius,
                        radius,
                        std::f64::consts::FRAC_PI_2,
                        std::f64::consts::PI,
                    );
                    cr.arc(
                        bx + radius,
                        by + radius,
                        radius,
                        std::f64::consts::PI,
                        -std::f64::consts::FRAC_PI_2,
                    );
                    cr.close_path();
                } else {
                    cr.rectangle(bx, by, bw, bh);
                }
                cr.fill().unwrap();
            }
        } else {
            // Compact mode: per-line boxes
            let mut iter = quote_layout_bg.iter();
            loop {
                let (_, logical) = iter.line_extents();
                let (ink, _) = iter.line_readonly().unwrap().extents();

                let lw = (ink.width() as f64) / pango::SCALE as f64;
                let lh = (logical.height() as f64) / pango::SCALE as f64;
                let ly = (logical.y() as f64) / pango::SCALE as f64;
                let lx = aligned_x(qw, lw, a.quote_h_align);

                let bx = qx + lx - padding_h;
                let by = qy + quote_y_offset + ly - padding_v;
                let bw = lw + padding_h * 2.0;
                let bh = lh + padding_v * 2.0;

                // Only draw if the line is mostly visible
                if ly + lh * 0.8 < qh {
                    if radius > 0.0 {
                        cr.new_sub_path();
                        cr.arc(
                            bx + bw - radius,
                            by + radius,
                            radius,
                            -std::f64::consts::FRAC_PI_2,
                            0.0,
                        );
                        cr.arc(
                            bx + bw - radius,
                            by + bh - radius,
                            radius,
                            0.0,
                            std::f64::consts::FRAC_PI_2,
                        );
                        cr.arc(
                            bx + radius,
                            by + bh - radius,
                            radius,
                            std::f64::consts::FRAC_PI_2,
                            std::f64::consts::PI,
                        );
                        cr.arc(
                            bx + radius,
                            by + radius,
                            radius,
                            std::f64::consts::PI,
                            -std::f64::consts::FRAC_PI_2,
                        );
                        cr.close_path();
                    } else {
                        cr.rectangle(bx, by, bw, bh);
                    }
                    cr.fill().unwrap();
                }

                if !iter.next_line() {
                    break;
                }
            }
        }

        cr.pop_group_to_source().unwrap();
        cr.paint_with_alpha(bg_a as f64).unwrap();
        cr.restore().unwrap();
    }

    // ── Quote Text ──
    cr.save().unwrap();
    cr.rectangle(qx, qy, qw, qh);
    cr.clip();

    if is_dragging_q {
        cr.push_group();
    }

    let quote_layout = pc::create_layout(cr);
    let mut q_font = FontDescription::new();
    q_font.set_family(&a.font);
    q_font.set_size((a.font_size as i32) * pango::SCALE);
    quote_layout.set_font_description(Some(&q_font));
    quote_layout.set_text(PREVIEW_QUOTE);
    quote_layout.set_width(a.quote_max_width * pango::SCALE);
    quote_layout.set_wrap(pango::WrapMode::Word);
    quote_layout.set_alignment(pango_alignment(a.quote_h_align));
    let quote_y_offset = vertical_offset(qh, layout_pixel_height(&quote_layout), a.quote_v_align);

    // Stroke
    cr.move_to(qx, qy + quote_y_offset);
    if a.stroke_enabled {
        let (sr, sg, sb, sa) = parse_color_to_rgba(&a.stroke_color);
        cr.set_source_rgba(sr, sg, sb, sa);
        cr.set_line_width(a.stroke_width as f64);
        pc::layout_path(cr, &quote_layout);
        cr.stroke().unwrap();
    }

    // Text
    cr.move_to(qx, qy + quote_y_offset);
    cr.set_source_rgba(r, g, b, alpha);
    pc::show_layout(cr, &quote_layout);

    if is_dragging_q {
        cr.pop_group_to_source().unwrap();
        cr.paint_with_alpha(0.7).unwrap();
    }

    cr.restore().unwrap();

    // Info label
    cr.set_source_rgba(0.3, 0.8, 1.0, 1.0);
    cr.move_to(qx, qy + qh + 14.0);
    let info = pc::create_layout(cr);
    let mut ifont = FontDescription::new();
    ifont.set_family("sans-serif");
    ifont.set_size(13 * pango::SCALE);
    info.set_font_description(Some(&ifont));
    let label = format!("Container {}×{} px", a.quote_max_width, a.quote_max_height);
    info.set_text(&label);
    pc::show_layout(cr, &info);
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
