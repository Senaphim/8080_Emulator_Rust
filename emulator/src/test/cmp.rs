use crate::state::State8080;

#[test]
fn and_b() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa0;
    state.a = 0x11;
    state.b = 0x10;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}
