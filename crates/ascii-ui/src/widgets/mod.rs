pub mod button;
pub mod checkbox;
// pub mod column;
pub mod container;
pub(crate) mod divider;
pub(crate) mod flex;
pub mod grid;
pub mod list_builder;
pub(crate) mod plugin;
pub mod scrolling_view;
pub mod tab_view;
pub mod text;
pub mod texture;

pub use button::Button;
pub use checkbox::Checkbox;
pub use container::SingleChildWidget;
pub use divider::Divider;
pub use flex::FlexWidget;
pub use grid::Grid;
pub use list_builder::ListBuilderWidget;
pub use scrolling_view::ScrollingView;
pub use tab_view::TabView;
pub use text::Text;
pub use texture::Texture;

#[macro_export]
macro_rules! row {
    ($($item:expr),*$(,)?) => {
        ascii_ui::widgets::FlexWidget::row(vec![$($item),*])
    };
}

#[macro_export]
macro_rules! col {
    ($($item:expr),*$(,)?) => {
        ascii_ui::widgets::FlexWidget::column(vec![$($item),*])
    };
}
#[macro_export]
macro_rules! text {
    ($value:expr) => {
        ascii_ui::widgets::Text::build($value)
    };
}
#[macro_export]
macro_rules! sized_box {
    (vertical: $size:expr) => {
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical($size))
    };
    (horizontal: $size:expr) => {
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal($size))
    };
}
