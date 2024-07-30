#![no_std]

use gmeta::{In, InOut, Out, Metadata};
use gstd::{prelude::*, ActorId, MessageId};

pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<SessionAction, SessionEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Session>;
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
    CheckGameStatus {
        user: ActorId,
    },
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
    Gaming,
    GameOver,
    GameStart{ 
        user: ActorId
    },
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct Session {
    pub target_program_id: ActorId,
    pub msg_ids: (SentMessageId, OriginalMessageId),
    pub session_status: Option<SessionStatus>,
}