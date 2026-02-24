use ratatui::{layout::Rect, style::Color, symbols::Marker, widgets::canvas::Canvas, Frame};

pub fn draw_house(frame: &mut Frame, area: Rect) {
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, f64::from(area.width)])
        .y_bounds([0.0, f64::from(area.height)])
        .paint(move |ctx| {
            let width = f64::from(area.width);
            let height = f64::from(area.height);

            let house_left = width * 0.2;
            let house_right = width * 0.5;
            let house_bottom = height * 0.1;
            let house_top = height * 0.4;
            let roof_top = height * 0.6;

            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: house_left,
                y1: house_bottom,
                x2: house_right,
                y2: house_bottom,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: house_left,
                y1: house_bottom,
                x2: house_left,
                y2: house_top,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: house_right,
                y1: house_bottom,
                x2: house_right,
                y2: house_top,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: house_left,
                y1: house_top,
                x2: house_right,
                y2: house_top,
                color: Color::White,
            });

            let center_x = (house_left + house_right) / 2.0;
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: house_left,
                y1: house_top,
                x2: center_x,
                y2: roof_top,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: house_right,
                y1: house_top,
                x2: center_x,
                y2: roof_top,
                color: Color::White,
            });

            let door_left = center_x - width * 0.04;
            let door_right = center_x + width * 0.04;
            let door_top = house_bottom + height * 0.14;
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: door_left,
                y1: house_bottom,
                x2: door_left,
                y2: door_top,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: door_right,
                y1: house_bottom,
                x2: door_right,
                y2: door_top,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: door_left,
                y1: door_top,
                x2: door_right,
                y2: door_top,
                color: Color::White,
            });

            let window_width = width * 0.06;
            let window_height = height * 0.06;
            let window_spacing = width * 0.04;
            let window_y = house_bottom + height * 0.2;

            let window1_left = center_x - window_width - window_spacing / 2.0;
            let window1_right = center_x - window_spacing / 2.0;
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window1_left,
                y1: window_y,
                x2: window1_right,
                y2: window_y,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window1_left,
                y1: window_y,
                x2: window1_left,
                y2: window_y + window_height,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window1_right,
                y1: window_y,
                x2: window1_right,
                y2: window_y + window_height,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window1_left,
                y1: window_y + window_height,
                x2: window1_right,
                y2: window_y + window_height,
                color: Color::White,
            });

            let window2_left = center_x + window_spacing / 2.0;
            let window2_right = center_x + window_width + window_spacing / 2.0;
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window2_left,
                y1: window_y,
                x2: window2_right,
                y2: window_y,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window2_left,
                y1: window_y,
                x2: window2_left,
                y2: window_y + window_height,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window2_right,
                y1: window_y,
                x2: window2_right,
                y2: window_y + window_height,
                color: Color::White,
            });
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: window2_left,
                y1: window_y + window_height,
                x2: window2_right,
                y2: window_y + window_height,
                color: Color::White,
            });
        });

    frame.render_widget(canvas, area);
}
