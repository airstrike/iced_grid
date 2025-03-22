//! A grid layout that arranges its children in equal-sized cells.
use iced::Alignment::Center;
use iced::Length::Shrink;
use iced::advanced::renderer;
use iced::widget::container::{Style, StyleFn};
use iced::widget::{Space, column, container, responsive, row, scrollable};
use iced::{Element, Length, Padding, Pixels, Size};

/// A grid layout that arranges its children in equal-sized cells.
pub struct Grid<'a, Message, Theme, Renderer, I>
where
    Theme: Catalog,
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
    min_width: Option<f32>,
    scroll: bool,
    class: <Theme as container::Catalog>::Class<'a>,
    _phantom: std::marker::PhantomData<(&'a Message, &'a Renderer)>,
}

impl<'a, Message, Theme, Renderer, I> Grid<'a, Message, Theme, Renderer, I>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: 'a,
    I: IntoIterator + 'a,
    I::Item: Into<Element<'a, Message, Theme, Renderer>>,
{
    /// Creates a new [`Grid`] with the given number of columns.
    ///
    /// It will arrange all of the elements in a grid layout.
    ///
    /// If columns is 0, it will calculate columns based on min_width and available space.
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
            min_width: None,
            scroll: false,
            class: <Theme as container::Catalog>::default(),
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
        <Theme as container::Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    pub fn class(mut self, class: impl Into<<Theme as container::Catalog>::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the aspect ratio for grid cells (width/height).
    ///
    /// For example, a 16:9 aspect ratio would be 16.0/9.0.
    /// When set, the grid will maintain this aspect ratio for all cells.
    ///
    /// If combined with a fixed number of columns, the width will be used to determine height.
    /// If combined with dynamic columns (columns=0) and min_width, both are used to calculate
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
    pub fn min_width(mut self, width: impl Into<Pixels>) -> Self {
        self.min_width = Some(width.into().0);
        self
    }

    /// Makes the grid scrollable.
    ///
    /// This wraps the grid in a scrollable container that handles both vertical
    /// and horizontal overflow. This is particularly useful when combined with
    /// aspect_ratio and/or min_width to ensure proper scrolling behavior.
    pub fn scrollable(mut self) -> Self {
        self.scroll = true;
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
    Theme: Catalog + 'a,
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
    Theme: Catalog + 'a,
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
            width,
            height,
            aspect_ratio,
            min_width,
            scroll,
            class,
            ..
        } = grid;

        // empty or invalid grid
        if columns == 0 && min_width.is_none() {
            return Space::new(Shrink, Shrink).into();
        }

        let content = responsive(move |container_size: Size| {
            let limits = iced::advanced::layout::Limits::new(
                Size::ZERO,
                Size::new(container_size.width, container_size.height),
            );

            let resolved_size = limits.resolve(width, height, Size::ZERO);
            let mut items_iter = items.clone().into_iter();

            // for most iterators the lower bound is accurate but if not,
            // we fallback to cloning and counting
            let item_count = match items_iter.size_hint() {
                (lower, Some(upper)) if lower == upper => lower,
                _ => items.clone().into_iter().count(),
            };

            // if we have a dynamic number of columns,
            // then calculate that # based on min_width
            let actual_columns = if columns == 0 {
                if let Some(min_width) = min_width {
                    let calculated_columns = ((resolved_size.width + horizontal_spacing)
                        / (min_width + horizontal_spacing))
                        .floor() as usize;
                    calculated_columns.max(1)
                } else {
                    1
                }
            } else {
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

            // calculate cell dimensions based on available space and settings
            let total_h_spacing = horizontal_spacing * (actual_columns as f32 - 1.0);
            let cell_width = (resolved_size.width - total_h_spacing) / actual_columns as f32;

            let cell_height = if let Some(ratio) = aspect_ratio {
                // if aspect ratio is provided, calculate height based on width
                cell_width / ratio
            } else {
                // otherwise distribute height evenly
                let total_v_spacing = vertical_spacing * (row_count as f32 - 1.0);
                (resolved_size.height - total_v_spacing) / row_count as f32
            };

            let grid_rows = (0..row_count).map(|_| {
                let row_elements = (0..actual_columns).map(|_| {
                    if let Some(item) = items_iter.next() {
                        container(item.into())
                            .center_x(cell_width)
                            .center_y(cell_height)
                            .into()
                    } else {
                        Space::new(cell_width, cell_height).into()
                    }
                });

                row(row_elements).spacing(horizontal_spacing).into()
            });

            let grid_layout = column(grid_rows).spacing(vertical_spacing).align_x(Center);

            // TODO: this bit doesn't really work.
            // if we have an aspect ratio, we can calculate a fixed height for the grid
            // so that the scrollable doesn't have an "infinite scrolling" child, but this
            // doesn't quite work because responsive() always fills its parent's available space
            if aspect_ratio.is_some() {
                // total grid height including spacing
                let total_v_spacing = vertical_spacing * (row_count as f32 - 1.0);
                let total_grid_height = (cell_height * row_count as f32) + total_v_spacing;

                grid_layout
                    .height(total_grid_height)
                    .width(resolved_size.width)
                    .padding(padding)
                    .into()
            } else {
                // just return the grid layout
                grid_layout.into()
            }
        });

        // TODO: this doesn't work either because of the way responsive() works
        // let has_defined_cell_size = aspect_ratio.is_some() || min_width.is_some();
        //
        // let grid_container = if has_defined_cell_size {
        //     // for grids with defined cell sizes (through aspect ratio or minimum width),
        //     // make the container "tight" to the content
        //     container(content).center(Shrink)
        // } else if columns > 0 {
        //     // for fixed column grids without defined cell sizes,
        //     // use the grid width/height Lengths chosen by the user
        //     container(content).center_x(width).center_y(height)
        // } else {
        //     // for dynamic column grids without defined cell sizes, use the
        //     // specified width Length but make height "tight" to prevent
        //     // infinite scrolling
        //     container(content).center_x(width).center_y(Shrink)
        // }
        // .padding(padding)
        // .class(class);
        let grid_container = container(content).width(Shrink).height(Shrink).class(class);

        if scroll {
            scrollable(grid_container).spacing(2.0).into()
        } else {
            grid_container.into()
        }
    }
}

// Extension trait for creating grids from iterators
pub trait GridExt<'a, Message, Theme, Renderer, T>: Sized
where
    Message: 'a,
    Theme: Catalog + 'a,
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
    Theme: Catalog + 'a,
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
    Theme: Catalog,
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
            .field("min_width", &self.min_width)
            .field("scroll", &self.scroll)
            .finish()
    }
}

pub trait Catalog: iced::widget::container::Catalog + iced::widget::scrollable::Catalog {}

impl<T> Catalog for T where T: iced::widget::container::Catalog + iced::widget::scrollable::Catalog {}
