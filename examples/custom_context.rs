use std::fs::{File, OpenOptions};

use bevy::{app::AppExit, prelude::*};
use bevy_ratatui::{
    RatatuiContext, RatatuiPlugins,
    context::{CrosstermTerminalContext, TerminalContext, configure_crossterm_plugin_group},
    event::KeyMessage,
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand, cursor,
        event::KeyCode,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    text::Text,
};

#[derive(Deref, DerefMut, Debug)]
struct DevTtyContext(Terminal<CrosstermBackend<File>>);

impl TerminalContext for DevTtyContext {
    type Backend = CrosstermBackend<File>;

    fn init() -> Result<Self> {
        let mut tty = Self::terminal_writer()?;
        tty.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self(Terminal::new(CrosstermBackend::new(tty))?))
    }

    fn restore() -> Result {
        let mut tty = Self::terminal_writer()?;
        tty.execute(LeaveAlternateScreen)?.execute(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn configure_plugin_group(
        group: &RatatuiPlugins<Self>,
        builder: bevy::app::PluginGroupBuilder,
    ) -> bevy::app::PluginGroupBuilder {
        configure_crossterm_plugin_group(group, builder)
    }
}

impl CrosstermTerminalContext for DevTtyContext {
    type Writer = File;

    fn terminal_writer() -> std::io::Result<Self::Writer> {
        OpenOptions::new().read(true).write(true).open("/dev/tty")
    }
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f32(1. / 60.),
            )),
            RatatuiPlugins::<DevTtyContext>::default(),
        ))
        .add_systems(PreUpdate, input_system)
        .add_systems(Update, draw_system)
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext<DevTtyContext>>) -> Result {
    context.draw(|frame| {
        let text = Text::raw("hello from /dev/tty\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

fn input_system(mut messages: MessageReader<KeyMessage>, mut exit: MessageWriter<AppExit>) {
    for message in messages.read() {
        if let KeyCode::Char('q') = message.code {
            exit.write_default();
        }
    }
}
