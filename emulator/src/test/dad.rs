use crate::State8080;

#[test]
fn dad_b() {
    let mut state = State8080::default();

    state.memory[0x00] = 0x09;
    state.b = 0x12;
    state.c = 0x34;
    state.h = 0x11;
    state.l = 0x22;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x23);
    assert_eq!(state.l, 0x56);
}

#[test]
fn dad_d() {
    let mut state = State8080::default();

    state.memory[0x00] = 0x19;
    state.d = 0x12;
    state.e = 0x34;
    state.h = 0x11;
    state.l = 0x22;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x23);
    assert_eq!(state.l, 0x56);
}

#[test]
fn dad_sp() {
    let mut state = State8080::default();

    state.memory[0x00] = 0x39;
    state.sp = 0x1234;
    state.h = 0x11;
    state.l = 0x22;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x23);
    assert_eq!(state.l, 0x56);
}

#[test]
fn dad_h() {
    let mut state = State8080::default();

    state.memory[0x00] = 0x29;
    state.h = 0x12;
    state.l = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x24);
    assert_eq!(state.l, 0x68);
}

#[test]
fn dad_cy() {
    let mut state = State8080::default();

    state.memory[0x00] = 0x29;
    state.h = 0x88;
    state.l = 0x88;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x11);
    assert_eq!(state.l, 0x10);
    assert!(state.cc.cy)
}
