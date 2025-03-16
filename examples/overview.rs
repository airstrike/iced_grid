use iced::Alignment::*;
use iced::widget::{button, center, column, container, text};
use iced::{Element, Length, Task, Theme};

use iced_grid::{GridExt, grid};

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed(usize),
}

#[derive(Default)]
struct Grid;

impl Grid {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ButtonPressed(index) => {
                println!("Button {} was pressed", index);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Grid Layout Examples")
            .size(28)
            .width(Length::Fill)
            .align_x(Center);

        // Example 1: Basic grid using the grid function directly
        let basic_grid = {
            let items = (0..12).map(|i| {
                let button = button(text!("Item {i}")).on_press(Message::ButtonPressed(i));

                container(button).padding(5).align_x(Center).align_y(Center)
            });

            let grid = grid(4, items).spacing(10.0).padding(20);

            center(grid).padding(10)
        };

        // Example 2: Using the extension trait with styled cells
        let ext_trait_grid = {
            let items = (0..9).map(|i| {
                let content = text(format!("Cell {}", i)).size(16).align_x(Center);

                let style = move |theme: &Theme| {
                    container::Style::default()
                        .background(if i % 2 == 0 {
                            theme.extended_palette().background.strong.color
                        } else {
                            theme.extended_palette().background.weak.color
                        })
                        .color(theme.extended_palette().background.base.text)
                };

                center(content).style(style)
            });

            let grid = items.grid(3).spacing(5.0).padding(10);

            center(grid).padding(10)
        };

        // Layout all examples vertically
        let content = column![
            title,
            container(text("Basic 4-column grid with buttons").size(18)).padding([10, 0]),
            basic_grid,
            container(text("3-column grid using extension trait with styled cells").size(18))
                .padding([20, 0]),
            ext_trait_grid,
        ]
        .spacing(10)
        .padding(20)
        .align_x(Center);

        center(content).style(container::rounded_box).into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

fn main() -> iced::Result {
    iced::application("iced_grid • overview", Grid::update, Grid::view)
        .centered()
        .theme(Grid::theme)
        .run()
}
