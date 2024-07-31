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
pub enum GameResult {
    Win,
    Lose
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionEvent {
    UnStarted {
        user: ActorId,
        info: String
    },
    AlreadyStarted {
        user: ActorId,
        info: String 
    },
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameOver {
        user: ActorId,
        result: GameResult
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionStatus {
    GameInit,
    GameStart { 
        user: ActorId
    },
    Gaming {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameOver {
        user: ActorId,
        result: GameResult
    },
}

type SentMessageId = MessageId;
type OriginalMessageId = MessageId;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Session {
    pub target_program_id: ActorId,
    pub msg_ids: (SentMessageId, OriginalMessageId),
    pub session_status: SessionStatus,
}