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
            session_status: SessionStatus::GameInit,
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    let action: SessionAction = msg::load().expect("Unable to decode `Action`");
    
    match &session.session_status {
        SessionStatus::GameInit => {
            match action {
                SessionAction::StartGame { user} => {
                    let msg_id = msg::send(session.target_program_id, Event::GameStarted { user }, 0)
                        .expect("Error in sending a message");
                    session.msg_ids = (msg_id, msg::id());
                    exec::wait();
                }
                SessionAction::CheckWord { user, word } => {
                    msg::reply(SessionEvent::UnStarted { 
                        user,
                        info: String::from("The game is uninited") 
                    }, 0)
                        .expect("Unable to reply msg");
                }
                SessionAction::CheckGameStatus { user } => {
                    msg::reply(SessionEvent::UnStarted { 
                        user,
                        info: String::from("The game is uninited") 
                    }, 0)
                        .expect("Unable to reply msg");
                }
            }
        }
        SessionStatus::GameStart { user } => {
            let user = user.clone();
            msg::send_delayed(exec::program_id(), SessionAction::CheckGameStatus { user }, 0, WAIT_BLOCKS.into())
                .expect("Error in sending a message");
            msg::reply(SessionEvent::AlreadyStarted { 
                user,
                info: String::from("The game has already started")
            }, 0).expect("Unable to replay msg");
            session.session_status = SessionStatus::Gaming { user, contained_in_word: Vec::new(), correct_positions: Vec::new() };
        }
        SessionStatus::Gaming { user: ActorId, correct_positions, contained_in_word } => {
            let correct_positions = correct_positions.to_vec(); 
            match action {
                SessionAction::StartGame { user} => {
                    msg::reply(SessionEvent::AlreadyStarted {
                        user,
                        info: String::from("The game has already restarted")
                    }, 0).expect("Unable to replay msg");
                }
                SessionAction::CheckWord { user, word } => {
                    let msg_id = msg::send(session.target_program_id, Event::CheckWord { user, word }, 0)
                        .expect("Error in sending a message");
                    session.msg_ids = (msg_id, msg::id());
                    msg::reply(SessionEvent::WordChecked { 
                        user,
                        correct_positions: correct_positions.to_vec(), 
                        contained_in_word: contained_in_word.to_vec() 
                        }, 0).expect("Unable to replay msg");
                    exec::wait();
                }
                SessionAction::CheckGameStatus { user } => {

                }
            }
        }
        SessionStatus::GameOver { user, result } => {
            let user = user.clone();
            let result = result.clone();
            msg::reply(SessionEvent::GameOver { user, result } , 0)
                .expect("Unable to replay msg");
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
                session.session_status = SessionStatus::GameStart { user };
                let original_message_id = session.msg_ids.1;
                exec::wake(original_message_id).expect("Failed to wake message");
            }
            Event::WordChecked { user, correct_positions, contained_in_word } => {
                if correct_positions.len() == 5 {
                    session.session_status = SessionStatus::GameOver { user, result: GameResult::Win };
                } else {
                    session.session_status = SessionStatus::Gaming { user, correct_positions, contained_in_word };
                }
                let original_message_id = session.msg_ids.1;
                exec::wake(original_message_id).expect("Failed to wake message");
            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let session = unsafe { SESSION.take().expect("State is not existing") };
    msg::reply(session, 0).expect("Unable to get the state");
}