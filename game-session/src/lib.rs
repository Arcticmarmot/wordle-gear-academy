#![no_std]
use game_session_io::*;
use gstd::{exec, msg, prelude::*, ActorId, MessageId};
extern crate wordle_io;
use wordle_io::{Action, Event};

static mut SESSION: Option<Session> = None;
const WAIT_BLOCKS: u8 = 2;
const ATTEMPT_TIMES: u8 = 2;

#[no_mangle]
extern "C" fn init() {
    let target_program_id = msg::load().expect("Unable to decode Init");
    unsafe {
        SESSION = Some(Session {
            target_program_id,
            msg_ids: (MessageId::zero(), MessageId::zero()),
            session_status: None,
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    let action: SessionAction = msg::load().expect("Unable to decode `Action`");
    
    match &session.session_status {
        None => {
            match action {
                SessionAction::StartGame { user} => {
                    let msg_id = msg::send(session.target_program_id, Event::GameStarted { user }, 0)
                    .expect("Error in sending a message");
                    session.msg_ids = (msg_id, msg::id());
                    exec::wait();
                }
                _ => {
                    msg::reply(SessionEvent::UnStarted { info: String::from("The game is unexisted") }, 0)
                    .expect("Unable to reply msg");
                }
            }
        }
        Some(SessionStatus::GameStart { user: ActorId }) => {
            msg::send_delayed(exec::program_id(), SessionAction::CheckGameStatus { user }, 0, WAIT_BLOCKS.into())
                .expect("Error in sending a message");
            msg::reply(SessionEvent::AlreadyStarted { info: String::from("The game has already started")}, 0)
            .expect("Unable to replay msg");
            session.session_status = Some(SessionStatus::Gaming);
        }
        Some(SessionStatus::Gaming) => {
            match action {
                SessionAction::StartGame { user} => {

                }
                SessionAction::CheckWord { user, word } => {

                }
                SessionAction::CheckGameStatus { user } => {

                }
            }
        }
        Some(SessionStatus::GameOver) => {
            
        }
    }
    unsafe {
        SESSION = Some(Session {
            target_program_id: session.target_program_id,
            msg_ids: session.msg_ids,
            session_status: session.session_status.clone(),
        });
    }
}

#[no_mangle]
extern "C" fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    
    if reply_to == session.msg_ids.0 {
        let reply_message: Event = msg::load().expect("Unable to decode `Event`");
        match reply_message {
            Event::GameStarted { user } => {
                let original_message_id = session.msg_ids.1;
                session.session_status = Some(SessionStatus::GameStart { user });
                exec::wake(original_message_id).expect("Failed to wake message");
            }
            Event::WordChecked { user, correct_positions, contained_in_word } => {

            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let session = unsafe { SESSION.take().expect("State is not existing") };
    msg::reply(session, 0).expect("Unable to get the state");
}