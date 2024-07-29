#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId, MessageId};

pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = ();
    type Handle = InOut<SessionAction, SessionEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionAction {
    StartGame {
        user: ActorId,
    },
    CheckWord {
        user: ActorId,
        word: String,
    },
    CheckGameStatus
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionEvent {
    UnStarted { info: String },
    AlreadyStarted { info: String }
}

type SentMessageId = MessageId;
type OriginalMessageId = MessageId;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum SessionStatus {
    #[default]
    UnExisted,
    GameStart{ 
        user: ActorId
    },
    Gaming,
    GameOver,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct Session {
    pub target_program_id: ActorId,
    pub msg_ids: (SentMessageId, OriginalMessageId),
    pub session_status: SessionStatus,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    StartGame {
        user: ActorId,
    },
    CheckWord {
        user: ActorId,
        word: String,
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]

pub enum Event {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}