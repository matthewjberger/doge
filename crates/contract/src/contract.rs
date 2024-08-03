#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Message {
    #[default]
    Empty,
    Command {
        command: Command,
    },
    Event {
        event: Event,
    },
}

impl Message {
    pub const COMMAND_TOPIC: &'static str = "command";
    pub const EVENT_TOPIC: &'static str = "event";
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    #[default]
    Empty,

    Notify {
        content: String,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Event {
    #[default]
    Empty,

    Notification {
        content: String,
    },
}
