use gtest::{Program, System};
use game_session_io::{Session, SessionAction};


const USER1: u64 = 8;
const USER2: u64 = 9;

#[test]
fn test() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program = Program::current(&system);
    let target_program = Program::from_file(&system, "../target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame {user: USER1.into()});
    assert!(!start_result.main_failed());
    // std::thread::sleep(std::time::Duration::from_secs(3));
    
    system.spend_blocks(3);
    // USER1 第一次猜测
    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("huodd") });
    assert!(!check_word_result.main_failed());

    system.spend_blocks(3);

    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("houuu") });
    assert!(!check_word_result.main_failed());


    // USER1 猜测中途再次开始游戏 游戏成功重置
    // let start_result = proxy_program.send(USER1, SessionAction::StartGame {user: USER1.into()});
    // assert!(!start_result.main_failed());

    // let check_result = proxy_program.send(USER1, SessionAction::CheckGameStatus { user: USER1.into() });
    // assert!(!check_result.main_failed());


    // let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("huodd") });
    // assert!(!check_word_result.main_failed());

    // let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("house") });
    // assert!(!check_word_result.main_failed());

    // let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("house") });
    // assert!(!check_word_result.main_failed());

    // // USER2 开始游戏
    // let start_result = proxy_program.send(USER2, SessionAction::StartGame {user: USER2.into()});
    // assert!(!start_result.main_failed());

    // // USER2 第一次猜测
    // let check_word_result = proxy_program.send(USER2, SessionAction::CheckWord { user: USER2.into(), word: String::from("house") });
    // assert!(!check_word_result.main_failed());
}

#[test]
fn test_main() {
    let system = System::new();
    system.init_logger();

    // 初始化Proxy程序和Target程序
    let proxy_program = Program::current(&system);
    let target_program = Program::from_file(&system, "../target/wasm32-unknown-unknown/release/wordle.opt.wasm");
    let init_target_program_result = target_program.send_bytes(USER1, []);
    assert!(!init_target_program_result.main_failed());
    let init_proxy_program_result = proxy_program.send(USER1, target_program.id());
    assert!(!init_proxy_program_result.main_failed());

    // USER1 未开始游戏直接猜测
    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("huodd") });
    assert!(check_word_result.main_failed()); // 报错

    // USER1 开始游戏
    let start_result = proxy_program.send(USER1, SessionAction::StartGame {user: USER1.into()});
    assert!(!start_result.main_failed());


    // USER1 第一次猜测
    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("huodd") });
    assert!(!check_word_result.main_failed());

    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("huodd") });
    assert!(!check_word_result.main_failed());

    // USER1 猜测中途再次开始游戏 游戏成功重置
    // let start_result = proxy_program.send(USER1, SessionAction::StartGame {user: USER1.into()});
    // assert!(!start_result.main_failed());

    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("huodd") });
    assert!(!check_word_result.main_failed());

    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("house") });
    assert!(!check_word_result.main_failed());

    let check_word_result = proxy_program.send(USER1, SessionAction::CheckWord { user: USER1.into(), word: String::from("house") });
    assert!(!check_word_result.main_failed());

    // // USER2 开始游戏
    // let start_result = proxy_program.send(USER2, SessionAction::StartGame {user: USER2.into()});
    // assert!(!start_result.main_failed());

    // // USER2 第一次猜测
    // let check_word_result = proxy_program.send(USER2, SessionAction::CheckWord { user: USER2.into(), word: String::from("house") });
    // assert!(!check_word_result.main_failed());
}