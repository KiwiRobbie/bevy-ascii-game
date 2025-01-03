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
pub(crate) mod stack;
pub mod tab_view;
pub mod text;
pub mod text_edit;
pub mod texture;

pub use button::Button;
pub use checkbox::Checkbox;
pub use container::SingleChildWidget;
pub use divider::Divider;
pub use flex::{FlexWidget, MultiChildWidget};
pub use grid::Grid;
pub use list_builder::ListBuilderWidget;
pub use scrolling_view::ScrollingView;
pub use stack::Stack;
pub use tab_view::TabView;
pub use text::Text;
pub use text_edit::TextEdit;
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
    (height: $size:expr) => {
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::vertical($size))
    };
    (width: $size:expr) => {
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::horizontal($size))
    };

    (width: $width:expr, height: $height:expr) => {
        widgets::SingleChildWidget::build(None).with(attachments::SizedBox::new($width, $height))
    };
}
