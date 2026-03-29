use crate::State8080;

#[test]
fn xchg() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xeb;
    state.h = 0x01;
    state.l = 0x02;
    state.d = 0x03;
    state.e = 0x04;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x03);
    assert_eq!(state.l, 0x04);
    assert_eq!(state.d, 0x01);
    assert_eq!(state.e, 0x02);
}
