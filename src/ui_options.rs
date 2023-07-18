use serde::{Deserialize, Serialize};

/// Extra options for an activity's UI.
#[derive(Debug, Deserialize, Serialize, YaSerialize, YaDeserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum UiOptions {
    /// No extra UI options. This is the default
    #[yaserde(rename = "none")]
    #[default]
    None,
    /// Add a bar at the bottom of the screen to display action items in the app
    /// bar (also known as the action bar), when constrained for horizontal
    /// space (such as when in portrait mode on a handset). Instead of a
    /// small number of action items appearing in the app bar at the top of the
    /// screen, the app bar is split into the top navigation section and the
    /// bottom bar for action items. This ensures a reasonable amount of
    /// space is made available not only for the action items, but also for
    /// navigation and title elements at the top. Menu items are not split
    /// across the two bars; they always appear together.
    #[yaserde(rename = "splitActionBarWhenNarrow")]
    SplitActionBarWhenNarrow,
}
