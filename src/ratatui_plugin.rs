use std::marker::PhantomData;

use bevy::{
    app::{Plugin, PluginGroup, PluginGroupBuilder, Startup},
    prelude::{Commands, Result},
};

use crate::{RatatuiContext, context::DefaultContext};

use crate::context::TerminalContext;

/// A plugin group that includes all the plugins in the Ratatui crate.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ratatui::RatatuiPlugins;
///
/// App::new().add_plugins(RatatuiPlugins::default());
/// ```
pub struct RatatuiPlugins<C: TerminalContext = DefaultContext> {
    #[doc(hidden)]
    pub _context: PhantomData<fn() -> C>,
    /// Use kitty protocol if available and enabled.
    pub enable_kitty_protocol: bool,
    /// Capture mouse if enabled.
    pub enable_mouse_capture: bool,
    /// Forwards terminal input events to the bevy input system if enabled.
    pub enable_input_forwarding: bool,
}

pub type GenericRatatuiPlugins<C> = RatatuiPlugins<C>;

impl<C: TerminalContext> Default for RatatuiPlugins<C> {
    fn default() -> Self {
        Self {
            _context: PhantomData,
            enable_kitty_protocol: true,
            enable_mouse_capture: false,
            enable_input_forwarding: false,
        }
    }
}

impl RatatuiPlugins<DefaultContext> {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Default::default()
    }
}

impl<C: TerminalContext> PluginGroup for RatatuiPlugins<C> {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        builder = builder.add(ContextPlugin::<C>(PhantomData));

        builder = C::configure_plugin_group(&self, builder);

        builder
    }
}

/// The plugin responsible for adding the `RatatuiContext` resource to your bevy application.
pub struct ContextPlugin<C: TerminalContext = DefaultContext>(PhantomData<C>);

impl<C: TerminalContext> Default for ContextPlugin<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: TerminalContext> Plugin for ContextPlugin<C> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, context_setup::<C>);
    }
}

/// A startup system that sets up the terminal context.
pub fn context_setup<C: TerminalContext>(mut commands: Commands) -> Result {
    let terminal = RatatuiContext::<C>::init()?;
    commands.insert_resource(terminal);

    Ok(())
}
