use std::marker::PhantomData;

use bevy::prelude::*;

use crate::RatatuiContext;

use super::{context::CrosstermTerminalContext, kitty::GenericKittyEnabled};

#[cfg(feature = "mouse")]
use super::mouse::GenericMouseEnabled;

/// Plugin responsible for cleaning up resources in the correct order when exiting.
///
/// If raw mode, the alternate view, and the Kitty protocol are disabled in the wrong order, it can
/// cause issues for the terminal buffer after the application exits.
pub struct CleanupPlugin<C: CrosstermTerminalContext>(PhantomData<C>);

impl<C: CrosstermTerminalContext> Default for CleanupPlugin<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: CrosstermTerminalContext> Plugin for CleanupPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, cleanup::<C>);
    }
}

fn cleanup<C: CrosstermTerminalContext>(mut exit: MessageReader<AppExit>, mut commands: Commands) {
    for _ in exit.read() {
        commands.remove_resource::<GenericKittyEnabled<C>>();
        #[cfg(feature = "mouse")]
        commands.remove_resource::<GenericMouseEnabled<C>>();
        commands.remove_resource::<RatatuiContext<C>>();
    }
}
