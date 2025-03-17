use iced::widget::{container, text};
use iced::{Element, Fill, Task, Theme};

use iced_grid::GridExt;

fn main() -> iced::Result {
    iced::application("iced_grid • simple", SimpleGrid::update, SimpleGrid::view)
        .centered()
        .theme(SimpleGrid::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct SimpleGrid;

impl SimpleGrid {
    fn update(&mut self, _: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        (1..15)
            .map(|i| {
                container(text!("{i}").size(20))
                    .center(Fill)
                    .style(move |_| container::background(bg_color(i)))
            })
            .grid(3)
            .spacing(10.0)
            .padding(20.0)
            .style(|_| container::background(iced::color!(0x2A292A)))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

// Helper function to generate a classic VGA-style color
fn bg_color(i: usize) -> iced::Color {
    // Classic VGA palette used 6 bits total (2 bits per RGB channel)
    // Each channel had 4 possible values (0, 85, 170, 255 in modern equivalent)

    // Use the bits directly from the index to set RGB components
    let r = if (i & 0b100) != 0 { 0xFF } else { 0x00 };
    let g = if (i & 0b010) != 0 { 0xFF } else { 0x00 };
    let b = if (i & 0b001) != 0 { 0xFF } else { 0x00 };

    // For indices > 7, add some intensity to make colors lighter
    let intensity = if i > 7 { 0x80 } else { 0x00 };

    // Add intensity to any channels that are off
    let r = if r == 0 { intensity } else { r };
    let g = if g == 0 { intensity } else { g };
    let b = if b == 0 { intensity } else { b };

    // Combine channels into final color
    let rgb = (r << 16) | (g << 8) | b;
    iced::color!(rgb)
}
