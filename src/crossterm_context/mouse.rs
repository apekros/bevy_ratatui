//! Mouse support.
use std::marker::PhantomData;

use bevy::prelude::*;
use ratatui::crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture},
};

use super::context::CrosstermTerminalContext;

/// Plugin responsible for enabling mouse capture.
pub struct MousePlugin<C: CrosstermTerminalContext>(PhantomData<C>);

impl<C: CrosstermTerminalContext> Default for MousePlugin<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: CrosstermTerminalContext> Plugin for MousePlugin<C> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, mouse_setup::<C>);
    }
}

/// Resource indicating that mouse capture was successfully enabled in the current terminal buffer.
#[derive(Resource)]
pub struct GenericMouseEnabled<C: CrosstermTerminalContext>(PhantomData<C>);

pub type MouseEnabled = GenericMouseEnabled<crate::context::CrosstermContext>;

fn mouse_setup<C: CrosstermTerminalContext>(mut commands: Commands) -> Result {
    C::terminal_writer()?.execute(EnableMouseCapture)?;
    commands.insert_resource(GenericMouseEnabled::<C>(PhantomData));
    Ok(())
}

impl<C: CrosstermTerminalContext> Drop for GenericMouseEnabled<C> {
    fn drop(&mut self) {
        if let Ok(mut writer) = C::terminal_writer() {
            let _ = writer.execute(DisableMouseCapture);
        }
    }
}
