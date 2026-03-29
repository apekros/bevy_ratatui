//! Enhanced kitty keyboard protocol.
use std::{
    io::{self, Write},
    marker::PhantomData,
};

use bevy::prelude::*;
use ratatui::crossterm::{
    ExecutableCommand,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    terminal::supports_keyboard_enhancement,
};

use crate::{crossterm_context::context::CrosstermTerminalContext, ratatui_plugin::context_setup};

/// Plugin responsible for enabling the Kitty keyboard protocol in the current buffer.
///
/// Provides additional information involving keyboard events. For example, key release events will
/// be reported.
///
/// Refer to the above link for a list of terminals that support the protocol. An `Ok` result is not
/// a guarantee that all features are supported: you should have fallbacks that you use until you
/// detect the event type you are looking for.
///
/// [kitty keyboard protocol]: https://sw.kovidgoyal.net/kitty/keyboard-protocol/
pub struct KittyPlugin<C: CrosstermTerminalContext>(PhantomData<C>);

impl<C: CrosstermTerminalContext> Default for KittyPlugin<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: CrosstermTerminalContext> Plugin for KittyPlugin<C> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, kitty_setup::<C>.after(context_setup::<C>));
    }
}

fn kitty_setup<C: CrosstermTerminalContext>(mut commands: Commands) {
    if enable_kitty_protocol::<C>().is_ok() {
        commands.insert_resource(GenericKittyEnabled::<C>(PhantomData));
    }
}

/// A resource indicating that the Kitty keyboard protocol was successfully enabled in the current
/// buffer.
#[derive(Resource)]
pub struct GenericKittyEnabled<C: CrosstermTerminalContext>(PhantomData<C>);

pub type KittyEnabled = GenericKittyEnabled<crate::context::CrosstermContext>;

impl<C: CrosstermTerminalContext> Drop for GenericKittyEnabled<C> {
    fn drop(&mut self) {
        let _ = disable_kitty_protocol::<C>();
    }
}

/// Enables support for the Kitty keyboard protocol.
///
/// See [KittyPlugin].
pub fn enable_kitty_protocol<C: CrosstermTerminalContext>() -> io::Result<()> {
    if supports_keyboard_enhancement()? {
        C::terminal_writer()?
            .execute(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all()))?;
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Kitty keyboard protocol is not supported by this terminal.",
    ))
}

/// Disables the Kitty keyboard protocol, restoring the buffer to normal.
///
/// See [KittyPlugin].
pub fn disable_kitty_protocol_with_writer<W: Write>(mut writer: W) -> io::Result<()> {
    writer.execute(PopKeyboardEnhancementFlags)?;
    Ok(())
}

pub fn disable_kitty_protocol<C: CrosstermTerminalContext>() -> io::Result<()> {
    disable_kitty_protocol_with_writer(C::terminal_writer()?)
}
