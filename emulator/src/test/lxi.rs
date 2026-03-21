use crate::State8080;

#[test]
fn lxi_b() {
    let mut state = State8080::default();

    state.memory[0] = 0x01;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x03);
    assert_eq!(state.b, 0x34);
    assert_eq!(state.c, 0x12);
}

#[test]
fn lxi_d() {
    let mut state = State8080::default();

    state.memory[0] = 0x11;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x03);
    assert_eq!(state.d, 0x34);
    assert_eq!(state.e, 0x12);
}

#[test]
fn lxi_h() {
    let mut state = State8080::default();

    state.memory[0] = 0x21;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x03);
    assert_eq!(state.h, 0x34);
    assert_eq!(state.l, 0x12);
}

#[test]
fn lxi_sp() {
    let mut state = State8080::default();

    state.memory[0] = 0x31;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x03);
    assert_eq!(state.sp, 0x3412);
}
