use iced::advanced::renderer;
use iced::widget::{column, container, responsive, row};
use iced::{Element, Length, Padding, Size};

/// A grid layout that arranges its children in equal-sized cells.
pub struct Grid<'a, Message, Theme, Renderer, I>
where
    I: IntoIterator,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    columns: usize,
    items: I,
    horizontal_spacing: f32,
    vertical_spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    _phantom: std::marker::PhantomData<(&'a Message, &'a Theme, &'a Renderer)>,
}

impl<'a, Message, Theme, Renderer, I> Grid<'a, Message, Theme, Renderer, I>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a,
    I: IntoIterator + 'a,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    /// Creates a new [`Grid`] with the given number of columns.
    ///
    /// It will arrange all of the elements in a grid layout.
    pub fn new(columns: usize, items: I) -> Self {
        Self {
            columns: columns.max(1), // Ensure at least 1 column
            items,
            horizontal_spacing: 0.0,
            vertical_spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Fill,
            height: Length::Fill,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Sets the horizontal spacing between grid elements.
    pub fn horizontal_spacing(mut self, spacing: impl Into<f32>) -> Self {
        self.horizontal_spacing = spacing.into();
        self
    }

    /// Sets the vertical spacing between grid elements.
    pub fn vertical_spacing(mut self, spacing: impl Into<f32>) -> Self {
        self.vertical_spacing = spacing.into();
        self
    }

    /// Sets both horizontal and vertical spacing between grid elements.
    pub fn spacing(self, spacing: impl Into<f32>) -> Self {
        let spacing = spacing.into();
        self.horizontal_spacing(spacing).vertical_spacing(spacing)
    }

    /// Sets the padding of the [`Grid`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Grid`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Grid`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

/// Creates a grid with the given number of columns and items.
pub fn grid<'a, Message, Theme, Renderer, I>(
    columns: usize,
    items: I,
) -> Grid<'a, Message, Theme, Renderer, I>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a,
    I: IntoIterator + 'a,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    Grid::new(columns, items)
}

impl<'a, Message, Theme, Renderer, I> From<Grid<'a, Message, Theme, Renderer, I>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
    I: IntoIterator + Clone + 'a,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn from(grid: Grid<'a, Message, Theme, Renderer, I>) -> Self {
        let Grid {
            columns,
            items,
            horizontal_spacing,
            vertical_spacing,
            padding,
            width: grid_width,
            height: grid_height,
            ..
        } = grid;

        if columns == 0 {
            return container(column![]).into();
        }

        container(responsive(move |container_size: Size| {
            // Create limits based on container size
            let limits = iced::advanced::layout::Limits::new(
                Size::ZERO,
                Size::new(container_size.width, container_size.height),
            );

            // Resolve grid width and height within limits
            let resolved_size = limits.resolve(grid_width, grid_height, Size::ZERO);

            // Calculate available content width/height after padding
            let content_width = resolved_size.width - padding.left - padding.right;
            let content_height = resolved_size.height - padding.top - padding.bottom;

            // Collect items once for counting
            let collected_items: Vec<_> = items.clone().into_iter().collect();
            let item_count = collected_items.len();

            // Calculate row count (at least 1 row if there are items)
            let row_count = if item_count == 0 {
                0
            } else {
                (item_count + columns - 1) / columns // Ceiling division
            };

            if row_count == 0 {
                return container(column![]).into();
            }

            // Calculate cell width and height
            let total_h_spacing = horizontal_spacing * (columns as f32 - 1.0);
            let cell_width = (content_width - total_h_spacing) / columns as f32;

            let total_v_spacing = vertical_spacing * (row_count as f32 - 1.0);
            let cell_height = (content_height - total_v_spacing) / row_count as f32;

            // Create the grid row by row
            let mut item_iter = collected_items.into_iter();
            let grid_rows = (0..row_count).map(|_| {
                let row_elements = (0..columns).map(|_| {
                    // Get next item or use empty container for padding
                    let element = if let Some(item) = item_iter.next() {
                        item.into()
                    } else {
                        column![].into()
                    };

                    // Wrap the element in a container of the calculated size
                    container(element)
                        .center_x(cell_width)
                        .center_y(cell_height)
                        .into()
                });

                row(row_elements).spacing(horizontal_spacing).into()
            });

            column(grid_rows).spacing(vertical_spacing).into()
        }))
        .width(grid_width)
        .height(grid_height)
        .padding(padding)
        .into()
    }
}

// Extension trait for creating grids from iterators
pub trait GridExt<'a, Message, Theme, Renderer, T>: Sized
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a,
    T: Into<Element<'a, Message, Theme, Renderer>>,
{
    /// Arranges the elements in a grid with the specified number of columns.
    fn grid(self, columns: usize) -> Grid<'a, Message, Theme, Renderer, Self>
    where
        Self: IntoIterator<Item = T>;
}

impl<'a, Message, Theme, Renderer, I, T> GridExt<'a, Message, Theme, Renderer, T> for I
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a,
    I: IntoIterator<Item = T> + 'a,
    T: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn grid(self, columns: usize) -> Grid<'a, Message, Theme, Renderer, Self> {
        Grid::new(columns, self)
    }
}

impl<'a, Message, Theme, Renderer, I> std::fmt::Debug for Grid<'a, Message, Theme, Renderer, I>
where
    I: IntoIterator,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("columns", &self.columns)
            .field("horizontal_spacing", &self.horizontal_spacing)
            .field("vertical_spacing", &self.vertical_spacing)
            .field("padding", &self.padding)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
