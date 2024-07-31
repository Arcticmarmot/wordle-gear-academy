use gstd::{debug, ActorId};
use gtest::{Log, Program, System};
// extern crate wordle_io;
// use wordle_io::{Action, Event};
use game_session_io::{SessionAction, SessionEvent};


const USER: u64 = 3;
const TARGET_PROGRAM_ADDRESS: u64 = 2;

#[test]
fn test() {
    let system = System::new();
    system.init_logger();

    let proxy_program = Program::current(&system);
    let target_program = Program::from_file(&system, "../target/wasm32-unknown-unknown/release/wordle.opt.wasm");

    let init_target_program_result = target_program.send_bytes(USER, []);
    assert!(!init_target_program_result.main_failed());

    let target_program_address: ActorId = TARGET_PROGRAM_ADDRESS.into();
    let init_proxy_program_result = proxy_program.send(USER, target_program_address);
    assert!(!init_proxy_program_result.main_failed());

    let start_result = proxy_program.send(USER, SessionAction::StartGame {user: USER.into()});
    // gstd::debug!("Message payload: {:?}", start_result);
    assert!(!start_result.main_failed());
    // let log = Log::builder()
    //     .source(1)
    //     .dest(3)
    //     .payload(Event::GameStarted{USER.into()});
    // assert!(result.contains(&log));

}