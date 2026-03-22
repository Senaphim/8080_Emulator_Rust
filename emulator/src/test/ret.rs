use crate::state::State8080;

#[test]
fn ret_none() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xc9;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.sp, 0x4002);
}
