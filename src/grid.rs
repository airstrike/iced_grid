//! A grid layout that arranges its children in equal-sized cells.
use iced::Length::Shrink;
use iced::advanced::renderer;
use iced::widget::container::{Style, StyleFn};
use iced::widget::{Space, column, container, responsive, row};
use iced::{Element, Length, Padding, Pixels, Size};

/// A grid layout that arranges its children in equal-sized cells.
pub struct Grid<'a, Message, Theme, Renderer, I>
where
    Theme: container::Catalog,
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
    aspect_ratio: Option<f32>,
    minimum_width: Option<f32>,
    class: Theme::Class<'a>,
    _phantom: std::marker::PhantomData<(&'a Message, &'a Renderer)>,
}

impl<'a, Message, Theme, Renderer, I> Grid<'a, Message, Theme, Renderer, I>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: 'a,
    I: IntoIterator + 'a,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    /// Creates a new [`Grid`] with the given number of columns.
    ///
    /// It will arrange all of the elements in a grid layout.
    ///
    /// If columns is 0, it will calculate columns based on minimum_width and available space.
    pub fn new(columns: usize, items: I) -> Self {
        Self {
            columns,
            items,
            horizontal_spacing: 0.0,
            vertical_spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Fill,
            height: Length::Fill,
            aspect_ratio: None,
            minimum_width: None,
            class: Theme::default(),
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
    pub fn spacing(self, spacing: impl Into<Pixels>) -> Self {
        let spacing = spacing.into().0;
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

    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the aspect ratio for grid cells (width/height).
    ///
    /// For example, a 16:9 aspect ratio would be 16.0/9.0.
    /// When set, the grid will maintain this aspect ratio for all cells.
    ///
    /// If combined with a fixed number of columns, the width will be used to determine height.
    /// If combined with dynamic columns (columns=0) and minimum_width, both are used to calculate
    /// the ideal cell dimensions.
    pub fn aspect_ratio(mut self, ratio: impl Into<Pixels>) -> Self {
        self.aspect_ratio = Some(ratio.into().0);
        self
    }

    /// Sets the minimum width for grid cells.
    ///
    /// When used with columns=0, this determines how many columns can fit in the available space.
    /// If aspect_ratio is also set, the minimum width is used to calculate columns, and both width
    /// and height are calculated to maintain the aspect ratio.
    pub fn minimum_width(mut self, width: impl Into<Pixels>) -> Self {
        self.minimum_width = Some(width.into().0);
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
    Theme: container::Catalog + 'a,
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
            aspect_ratio,
            minimum_width,
            class,
            ..
        } = grid;

        // Empty or invalid grid
        if columns == 0 && minimum_width.is_none() {
            return Space::new(Shrink, Shrink).into();
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
            let content_width = resolved_size.width;
            let content_height = resolved_size.height;

            let mut items_iter = items.clone().into_iter();

            // For most iterators the lower bound is accurate; if not, we'll need to count
            let item_count = match items_iter.size_hint() {
                (lower, Some(upper)) if lower == upper => lower,
                _ => items.clone().into_iter().count(), // Fall back to counting if size_hint is unreliable
            };

            // Calculate actual columns based on settings
            let actual_columns = if columns == 0 {
                // Dynamic columns mode - calculate based on minimum_width
                if let Some(min_width) = minimum_width {
                    // Formula: (content_width + spacing) / (min_width + spacing)
                    // This accounts for having (columns-1) spacing between columns
                    let calculated_columns = ((content_width + horizontal_spacing)
                        / (min_width + horizontal_spacing))
                        .floor() as usize;
                    calculated_columns.max(1) // Ensure at least 1 column
                } else {
                    // Fallback to 1 column if no minimum width specified
                    1
                }
            } else {
                // Fixed columns mode
                columns
            };

            let row_count = if item_count == 0 {
                0
            } else {
                item_count.div_ceil(actual_columns)
            };

            if row_count == 0 {
                return container(column![]).into();
            }

            // Calculate cell dimensions based on available space and settings
            let total_h_spacing = horizontal_spacing * (actual_columns as f32 - 1.0);
            let cell_width = (content_width - total_h_spacing) / actual_columns as f32;

            let cell_height = if let Some(ratio) = aspect_ratio {
                // If aspect ratio is provided, calculate height based on width
                cell_width / ratio
            } else {
                // Otherwise distribute height evenly
                let total_v_spacing = vertical_spacing * (row_count as f32 - 1.0);
                (content_height - total_v_spacing) / row_count as f32
            };

            let grid_rows = (0..row_count).map(|_| {
                let row_elements = (0..actual_columns).map(|_| {
                    if let Some(item) = items_iter.next() {
                        container(item.into())
                            .center_x(cell_width)
                            .center_y(cell_height)
                            .into()
                    } else {
                        // Empty cell for padding incomplete rows
                        Space::new(cell_width, cell_height).into()
                    }
                });

                row(row_elements).spacing(horizontal_spacing).into()
            });

            column(grid_rows).spacing(vertical_spacing).into()
        }))
        .center_x(grid_width)
        .center_y(grid_height)
        .padding(padding)
        .class(class)
        .into()
    }
}

// Extension trait for creating grids from iterators
pub trait GridExt<'a, Message, Theme, Renderer, T>: Sized
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: 'a,
    T: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn grid(self, columns: usize) -> Grid<'a, Message, Theme, Renderer, Self>
    where
        Self: IntoIterator<Item = T>;
}

impl<'a, Message, Theme, Renderer, I, T> GridExt<'a, Message, Theme, Renderer, T> for I
where
    Message: 'a,
    Theme: container::Catalog + 'a,
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
    Theme: container::Catalog,
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
            .field("aspect_ratio", &self.aspect_ratio)
            .field("minimum_width", &self.minimum_width)
            .finish()
    }
}
