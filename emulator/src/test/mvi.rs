use crate::State8080;

#[test]
fn mvi_b() {
    let mut state = State8080::default();

    state.memory[0] = 0x06;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.b, 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_c() {
    let mut state = State8080::default();

    state.memory[0] = 0x0e;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.c, 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_d() {
    let mut state = State8080::default();

    state.memory[0] = 0x16;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.d, 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_e() {
    let mut state = State8080::default();

    state.memory[0] = 0x1e;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.e, 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_h() {
    let mut state = State8080::default();

    state.memory[0] = 0x26;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.h, 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_l() {
    let mut state = State8080::default();

    state.memory[0] = 0x2e;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.l, 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_m() {
    let mut state = State8080::default();

    state.memory[0] = 0x36;
    state.memory[1] = 0x12;
    state.h = 0x12;
    state.l = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.memory[0x1234], 0x12);
    assert_eq!(state.pc, 0x02);
}

#[test]
fn mvi_a() {
    let mut state = State8080::default();

    state.memory[0] = 0x3e;
    state.memory[1] = 0x12;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.a, 0x12);
    assert_eq!(state.pc, 0x02);
}
