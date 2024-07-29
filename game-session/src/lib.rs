#![no_std]
use game_session_io::*;
use gstd::{exec, msg, prelude::*, ActorId};

static mut SESSION: Option<Session> = None;
const WAIT_BLOCKS: u8 = 3;
#[no_mangle]
extern "C" fn init() {
    let target_program_id = msg::load().expect("Unable to decode Init");
    unsafe {
        SESSION = Some(Session {
            target_program_id,
            msg_ids: (MessageId::zero(), MessageId::zero()),
            session_status: SessionStatus::UnExisted,
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    let action: SessionAction = msg::load().expect("Unable to decode `Action`");
    
    match &session.session_status {
        SessionStatus::UnExisted => {
            match action {
                Action::StartGame { user} => {
                    let msg_id = msg::send(session.target_program_id, action, 0)
                    .expect("Error in sending a message");
                    session.msg_ids = (msg_id, msg::id());
                    exec::wait();
                }
                _ => {
                    msg::reply(SessionEvent::UnStarted { info: String::from("The game is unexisted") }, 0);
                }
            }
        }
        SessionStatus::GameStart { user: ActorId } => {
            msg::send_delayed(exec::program_id(), SessionAction::CheckGameStatus, 0, WAIT_BLOCKS)
                .expect("Error in sending a message");
            msg::reply(SessionEvent::AlreadyStarted { info: String::from("The game is already started, and user_id is " + user)}, 0);
            session.session_status = SessionStatus::Gaming;
        }
        SessionStatus::Gaming => {
            match action {
                Action::StartGame { user} => {
                }
                Action::CheckWord { user, word } => {

                }
                Action::CheckGameStatus => {

                }
            }
        }
        SessionStatus::GameOver => {

        }
        
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
                exec::wake(original_message_id).expect("Failed to wake message");
            }
            Event::WordChecked { user, correct_positions, contained_in_word } => {

            }
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    
}