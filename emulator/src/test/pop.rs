use crate::State8080;

#[test]
fn pop_b() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xc1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x02;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.b, 0x01);
    assert_eq!(state.c, 0x02);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_d() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xd1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x02;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.d, 0x01);
    assert_eq!(state.e, 0x02);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_h() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xe1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x02;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.h, 0x01);
    assert_eq!(state.l, 0x02);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_psw_s() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x82;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x01);
    assert!(state.cc.s);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_psw_z() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x42;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x01);
    assert!(state.cc.z);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_psw_ac() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x12;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x01);
    assert!(state.cc.ac);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_psw_p() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x06;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x01);
    assert!(state.cc.p);
    assert_eq!(state.sp, 0x2000);
}

#[test]
fn pop_psw_cy() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf1;
    state.memory[0x1fff] = 0x01;
    state.memory[0x1ffe] = 0x03;
    state.sp = 0x1ffe;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x01);
    assert!(state.cc.cy);
    assert_eq!(state.sp, 0x2000);
}
