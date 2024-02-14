mod debug_options;
mod info_counts;

use bevy::app::Plugin;
pub use debug_options::DebugOptions;
pub use info_counts::InfoCounts;

use self::{debug_options::DebugOptionsPlugin, info_counts::InfoCountsPlugin};

pub struct UiSectionsPlugin;
impl Plugin for UiSectionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((InfoCountsPlugin, DebugOptionsPlugin));
    }
}
