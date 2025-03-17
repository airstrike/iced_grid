// examples/calendar.rs
use iced::Alignment::*;
use iced::widget::{column, container, text};
use iced::{Element, Fill, Length, Task, Theme};

use chrono::Datelike;
use iced_grid::GridExt;

fn main() -> iced::Result {
    let size = iced::Size::new(800.0, 600.0);

    iced::application("iced • calendar grid", App::update, App::view)
        .window(iced::window::Settings {
            size,
            ..Default::default()
        })
        .centered()
        .theme(App::theme)
        .run_with(App::new)
}

#[derive(Debug, Clone)]
enum Message {}

struct App {
    current_month: chrono::DateTime<chrono::Local>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_month: chrono::Local::now(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let padding = 20.0;
        let spacing = 1.0;

        // Calendar metadata - use static array to avoid lifetime issues
        static WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

        // First day of month (0 = Sunday, 1 = Monday, etc.)
        let first_day = self
            .current_month
            .with_day(1)
            .unwrap()
            .weekday()
            .num_days_from_sunday() as usize;

        // Total days in the month
        let days_in_month = self
            .current_month
            .with_day(1)
            .unwrap()
            .with_month(self.current_month.month() + 1)
            .unwrap_or(self.current_month.with_month(1).unwrap())
            .signed_duration_since(self.current_month.with_day(1).unwrap())
            .num_days() as usize;

        // Today's day number (for highlighting)
        let today = chrono::Local::now().day() as usize;
        let current_month = chrono::Local::now().month() == self.current_month.month();
        let current_year = chrono::Local::now().year() == self.current_month.year();
        let is_current_month = current_month && current_year;

        // Weekdays header
        let weekday_header = WEEKDAYS
            .iter()
            .map(|day| {
                container(text(*day).size(14).align_x(Center))
                    .width(Length::Fill)
                    .align_x(Center)
            })
            .grid(7)
            .height(30.0)
            .spacing(spacing);

        // Calendar cells (7 columns × 6 rows)
        let calendar_cells = (0..42).map(move |i| {
            let day_number = i - first_day as i32 + 1;

            // Determine if this position has a valid day
            let is_valid_day = day_number > 0 && day_number <= days_in_month as i32;

            // Determine if this is today
            let is_today = is_current_month && is_valid_day && day_number as usize == today;

            // Cell content
            let content = if is_valid_day {
                day_number.to_string()
            } else {
                String::new()
            };

            // Cell style
            let container_style = move |theme: &Theme| {
                if is_today {
                    container::bordered_box(theme)
                        .background(theme.extended_palette().background.weak.color)
                } else if is_valid_day {
                    container::bordered_box(theme)
                } else {
                    container::Style::default()
                }
            };

            // Text style
            let text_style = move |theme: &Theme| {
                if is_today {
                    text::danger(theme)
                } else {
                    text::default(theme)
                }
            };

            container(text(content).style(text_style).size(14))
                .style(container_style)
                .align_top(Fill)
                .align_right(Fill)
                .padding(5)
        });

        let calendar_grid = calendar_cells.grid(7).spacing(spacing);

        // Full calendar layout
        container(
            column![
                text(self.current_month.format("%B %Y").to_string())
                    .size(24)
                    .width(Length::Fill)
                    .align_x(Center),
                weekday_header,
                calendar_grid,
            ]
            .spacing(spacing),
        )
        .width(Fill)
        .height(Fill)
        .padding(padding)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
