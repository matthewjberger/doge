use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum EngineMessage<C, E>
where
    C: fmt::Debug,
    E: fmt::Debug,
{
    #[default]
    Empty,
    EngineCommand {
        command: EngineCommand,
    },
    EngineEvent {
        event: EngineEvent,
    },
    AppCommand {
        command: C,
    },
    AppEvent {
        event: E,
    },
}

pub const ENGINE_COMMAND_TOPIC: &str = "engine_command";
pub const ENGINE_EVENT_TOPIC: &str = "engine_event";

pub const APP_COMMAND_TOPIC: &str = "app_command";
pub const APP_EVENT_TOPIC: &str = "app_event";

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum EngineCommand {
    #[default]
    Empty,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum EngineEvent {
    #[default]
    Empty,
}
