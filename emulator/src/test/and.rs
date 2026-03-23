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

#[test]
fn and_c() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa1;
    state.a = 0x11;
    state.c = 0x10;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}

#[test]
fn and_d() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa2;
    state.a = 0x11;
    state.d = 0x10;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}

#[test]
fn and_e() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa3;
    state.a = 0x11;
    state.e = 0x10;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}

#[test]
fn and_h() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa4;
    state.a = 0x11;
    state.h = 0x10;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}

#[test]
fn and_l() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa5;
    state.a = 0x11;
    state.l = 0x10;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}

#[test]
fn and_m() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa6;
    state.memory[0x1234] = 0x10;
    state.a = 0x11;
    state.h = 0x12;
    state.l = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
}

#[test]
fn and_a() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa7;
    state.a = 0x11;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x11);
}

#[test]
fn and_cy() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa0;
    state.a = 0x11;
    state.b = 0x10;
    state.cc.cy = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
    assert!(!state.cc.cy);
}

#[test]
fn and_z() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa0;
    state.a = 0x11;
    state.b = 0x10;
    state.cc.z = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
    assert!(!state.cc.z);
}

#[test]
fn and_s() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa0;
    state.a = 0x11;
    state.b = 0x10;
    state.cc.s = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
    assert!(!state.cc.s);
}

#[test]
fn and_p() {
    let mut state = State8080::default();

    state.memory[0x0000] = 0xa0;
    state.a = 0x11;
    state.b = 0x10;
    state.cc.p = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x0001);
    assert_eq!(state.a, 0x10);
    assert!(!state.cc.p);
}
