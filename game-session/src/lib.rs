#![no_std]
use collections::HashMap;
use game_session_io::*;
use gstd::{exec, msg, prelude::*, ActorId, MessageId, debug};
extern crate wordle_io;
use wordle_io::{Action, Event};

static mut SESSION: Option<Session> = None;
static mut GAME_STATUS_MAP: Option<HashMap<ActorId, GameStatus>> = None;
const INIT_BLOCKS: u32 = 3;
const INIT_ATTEMPS: u32 = 3;

#[no_mangle]
extern "C" fn init() {
    debug!("===INIT===");
    let target_program_id = msg::load().expect("Unable to decode Init");
    unsafe {
        SESSION = Some(Session {
            target_program_id,
            msg_ids: (MessageId::zero(), MessageId::zero()),
            session_status: SessionStatus::Waiting,
        });
        GAME_STATUS_MAP = Some(HashMap::new())
    }
}

#[no_mangle]
extern "C" fn handle() {
    debug!("===HANDLE START===");
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    let game_status_map = unsafe { GAME_STATUS_MAP.as_mut().expect("The game status map is not initialized") };

    debug!("---SESSION: {:?}---", session);
    debug!("---GAME_STATUS_MAP: {:?}---", game_status_map);

    let action: SessionAction = msg::load().expect("Unable to decode `Action`");
    match &session.session_status {
        SessionStatus::Waiting => {
            match action {
                SessionAction::StartGame { user} => {
                    debug!("===WAITING AND START GAME===");
                    let msg_id = msg::send(session.target_program_id, Action::StartGame { user }, 0)
                        .expect("Error in sending a message");
                    session.msg_ids = (msg_id, msg::id());
                    session.session_status = SessionStatus::MessageSent;
                    debug!("---SESSION: {:?}---", session);
                    exec::wait();
                }
                SessionAction::CheckWord { user, word } => {
                    debug!("===WAITING AND CHECK WORD===");
                    let query_user = user.clone();
                    let current_game_status = game_status_map.get(&query_user).expect("Unable to get user");
                    if current_game_status.left_attempts <= 0 {
                        debug!("===*******************===");
                        return;
                    }
                    let msg_id = msg::send(session.target_program_id, Action::CheckWord { user, word }, 0)
                        .expect("Error in sending a message");
                    session.msg_ids = (msg_id, msg::id());
                    session.session_status = SessionStatus::MessageSent;
                    debug!("---SESSION: {:?}---", session);
                    exec::wait();
                }
                SessionAction::CheckGameStatus { user } => {

                }
            }
        }
        SessionStatus::MessageSent => {
            
        }
        SessionStatus::MessageReceive(event) => {
            debug!("===MESSAGE RECEIVE===");
            let event = event.clone();
            let session_event;
            debug!("---EVENT PARAM: {:?}---", event);
            debug!("---GAME_STATUS_MAP: {:?}---", game_status_map);

            match event {
                Event::GameStarted { user } => {
                    let game_status = GameStatus {
                        left_seconds: INIT_BLOCKS * 60,
                        left_attempts: INIT_ATTEMPS,
                        game_result: None,
                    };
                    game_status_map.insert(user, game_status);
                    msg::send_delayed(exec::program_id(), SessionAction::CheckGameStatus { user }, 0, INIT_BLOCKS)
                        .expect("Unable to send delayed message");
                    session_event = SessionEvent::GameStarted { user };
                }
                Event::WordChecked { user, ref correct_positions, ref contained_in_word } => {
                    let query_user = user.clone();
                    let current_game_status = game_status_map.get(&query_user).expect("Unable to get user");
                    debug!("---CURRENT GAME STATUS: {:?}---", current_game_status);
                    let left_attemps = current_game_status.left_attempts - 1;
                    let correct_position_len = correct_positions.len();
                    let game_status;
                    if correct_position_len == 5 {
                        game_status = GameStatus{
                            left_seconds: INIT_BLOCKS * 60,
                            left_attempts: left_attemps,
                            game_result: Some(GameResult::Win),
                        };
                    } else {
                        if left_attemps <= 0 {
                            game_status = GameStatus{
                                left_seconds: INIT_BLOCKS * 60,
                                left_attempts: left_attemps,
                                game_result: Some(GameResult::Lose),
                            };
                        } else {
                            game_status = GameStatus{
                                left_seconds: INIT_BLOCKS * 60,
                                left_attempts: current_game_status.left_attempts - 1,
                                game_result: None,
                            };
                        }
                    }
                    game_status_map.insert(user, game_status);
                    session_event = SessionEvent::WordChecked { user, correct_positions: correct_positions.to_vec(), contained_in_word: contained_in_word.to_vec() };
                }
            };
            msg::reply(session_event, 0).expect("Error in sending a reply");
            session.session_status = SessionStatus::Waiting;
        }
    };
    unsafe {
        SESSION = Some(Session {
            target_program_id: session.target_program_id,
            msg_ids: session.msg_ids,
            session_status: session.session_status.clone(),
        });
    };
    debug!("---GAME_STATUS_MAP: {:?}---", game_status_map);
    debug!("---SESSION: {:?}---", session);
    debug!("===HANDLE ENDED===");
}

#[no_mangle]
extern "C" fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("The session is not initialized") };
    
    if reply_to == session.msg_ids.0 {
        let event: Event = msg::load().expect("Unable to decode `Event`");
        session.session_status = SessionStatus::MessageReceive(event);
        let original_message_id = session.msg_ids.1;
        exec::wake(original_message_id);
    }
}

#[no_mangle]
extern "C" fn state() {
    let session = unsafe { SESSION.take().expect("State is not existing") };
    msg::reply(session, 0).expect("Unable to get the state");
}