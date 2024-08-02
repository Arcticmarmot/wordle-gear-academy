#![no_std]
use collections::HashMap;
use game_session_io::*;
use gstd::{exec, msg, prelude::*, ActorId, MessageId, debug};
use wordle_io::{Action, Event};

static mut SESSION: Option<Session> = None;
static mut GAME_STATUS_MAP: Option<HashMap<ActorId, GameStatus>> = None;
const INIT_BLOCKS: u64 = 200;
const SECONDS_BLOCKS_RATIO: u64 = 3;
const INIT_SECONDS: u64 = INIT_BLOCKS * SECONDS_BLOCKS_RATIO;
const INIT_ATTEMPS: u32 = 3;
const WORD_LEN: u32 = 5;

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

    debug!("---SESSION ACTION: {:?}---", action);

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
                    let current_game_status = game_status_map.get_mut(&user)
                        .expect("Unable to get user");
                    match current_game_status.game_result {
                        Some(GameResult::Win) | Some(GameResult::Lose) => {
                            debug!("===GAME RESULT IS FIXED===");
                            let session_event = SessionEvent::GameStatus(current_game_status.clone());
                            debug!("---SESSION EVENT: {:?}---", session_event);
                            msg::reply(session_event, 0).expect("Unable to reply");
                        }
                        None => {
                            let left_seconds = get_left_seconds(&current_game_status);
                            let left_attempts = current_game_status.left_attempts;
                            if left_seconds > 0 && left_attempts > 0 {
                                let msg_id = msg::send(session.target_program_id, Action::CheckWord { user, word: word.clone() }, 0)
                                    .expect("Error in sending a message");
                                session.msg_ids = (msg_id, msg::id());
                                session.session_status = SessionStatus::MessageSent;
                                current_game_status.left_seconds = left_seconds;
                                current_game_status.left_attempts = left_attempts - 1;
                                // 新建猜测历史记录
                                current_game_status.history.push(WordGuessResult{
                                    word,
                                    correct_positions: None,
                                    contained_in_word: None
                                });
                                debug!("---GAME_STATUS_MAP: {:?}---", game_status_map);
                                exec::wait();
                            } else {
                                debug!("===CANNOT SEND A MESSAGE===");
                                current_game_status.game_result = Some(GameResult::Lose);
                                if left_seconds == 0 {
                                    current_game_status.left_seconds = 0;
                                }
                                let session_event = SessionEvent::GameError(String::from("The left seconds or attemps is over"));
                                debug!("---SESSION EVENT: {:?}---", session_event);
                                msg::reply(session_event, 0).expect("Unable to reply");
                            }
                        }
                    }
                }
                SessionAction::CheckGameStatus { user } => {
                    debug!("===CHECK GAME STATUS===");
                    let current_game_status = game_status_map.get_mut(&user)
                        .expect("Unable to get user");
                    match current_game_status.game_result {
                        Some(GameResult::Win) | Some(GameResult::Lose) => {
                        }
                        None => {
                            let left_seconds = get_left_seconds(&current_game_status);
                            if left_seconds > 0 {
                                current_game_status.left_seconds = left_seconds;
                            } else {
                                current_game_status.left_seconds = 0;
                                current_game_status.game_result = Some(GameResult::Lose);
                            }
                        }
                    }
                    let session_event = SessionEvent::GameStatus(current_game_status.clone());
                    msg::reply(session_event, 0).expect("Unable to reply");
                }
            }
        }
        SessionStatus::MessageSent => {
            debug!("===MESSAGE SENT===");
            msg::reply(SessionEvent::GameError(String::from("Message has already sent")), 0).expect("Error in sending a reply");
        }
        SessionStatus::MessageReceive(event) => {
            debug!("===MESSAGE RECEIVE===");
            debug!("---GAME_STATUS_MAP: {:?}---", game_status_map);
            debug!("---EVENT PARAM: {:?}---", event);
            let event = event.clone();
            let session_event;
            match event {
                Event::GameStarted { user } => {
                    msg::send_delayed(exec::program_id(), SessionAction::CheckGameStatus { user }, 0, INIT_BLOCKS as u32)
                    .expect("Unable to send delayed message");
                    let current_timestamp = get_timestamp();
                    let game_status = GameStatus {
                        start_timestamp: current_timestamp,
                        left_seconds: INIT_SECONDS,
                        left_attempts: INIT_ATTEMPS,
                        history: Vec::new(),
                        game_result: None,
                    };
                    game_status_map.insert(user, game_status);
                    session_event = SessionEvent::GameStarted { user };
                }
                Event::WordChecked { user, ref correct_positions, ref contained_in_word } => {
                    let current_game_status = game_status_map.get_mut(&user).expect("Unable to get user");
                    debug!("---CURRENT GAME STATUS: {:?}---", current_game_status);
                    let current_left_attemps = current_game_status.left_attempts;
                    let left_seconds = get_left_seconds(&current_game_status);
                    debug!("---LEFT SECONDS: {}---", left_seconds);

                    let correct_position_len = correct_positions.len();

                    current_game_status.left_seconds = left_seconds;

                    let current_history = &mut current_game_status.history;
                    let current_history_len = current_history.len();
                    current_game_status.left_seconds = left_seconds;
                    let last_guess = current_history.get_mut(current_history_len - 1)
                        .expect("Unable to get last guess");
                    last_guess.correct_positions = Some(correct_positions.to_vec());
                    last_guess.contained_in_word = Some(contained_in_word.to_vec());

                    if correct_position_len == WORD_LEN as usize{
                        current_game_status.game_result = Some(GameResult::Win);
                        session_event = SessionEvent::GameStatus(current_game_status.clone());
                    } else {
                        if left_seconds == 0 || current_left_attemps == 0 {
                            current_game_status.game_result = Some(GameResult::Lose);
                            session_event = SessionEvent::GameStatus(current_game_status.clone());
                        } else {
                            session_event = SessionEvent::WordChecked { 
                                user, 
                                correct_positions: correct_positions.to_vec(), 
                                contained_in_word: contained_in_word.to_vec()
                            };
                        }
                    }
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
        let _ = exec::wake(original_message_id);
    }
}

#[no_mangle]
extern "C" fn state() {
    let session = unsafe { SESSION.take().expect("State is not existing") };
    msg::reply(session, 0).expect("Unable to get the state");
}

fn get_timestamp() -> u64 {
    exec::block_timestamp() / 1000
}

fn get_left_seconds(game_status: &GameStatus) -> u64 {
    let current_time = get_timestamp();
    debug!("---TIMESTAMP :{}---", current_time);
    let elapse_time = current_time - game_status.start_timestamp;
    if elapse_time > INIT_SECONDS {
        0
    } else {
        INIT_SECONDS - elapse_time
    }
}