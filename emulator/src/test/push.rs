use crate::State8080;

#[test]
fn push_b() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xc5;
    state.b = 0x01;
    state.c = 0x02;
    state.sp = 0x2000;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x02);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_d() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xd5;
    state.d = 0x01;
    state.e = 0x02;
    state.sp = 0x2000;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x02);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_h() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xe5;
    state.h = 0x01;
    state.l = 0x02;
    state.sp = 0x2000;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x02);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_psw_s() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf5;
    state.a = 0x01;
    state.sp = 0x2000;
    state.cc.s = true;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x82);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_psw_z() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf5;
    state.a = 0x01;
    state.sp = 0x2000;
    state.cc.z = true;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x42);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_psw_ac() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf5;
    state.a = 0x01;
    state.sp = 0x2000;
    state.cc.ac = true;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x12);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_psw_p() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf5;
    state.a = 0x01;
    state.sp = 0x2000;
    state.cc.p = true;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x06);
    assert_eq!(state.sp, 0x1ffe);
}

#[test]
fn push_psw_cy() {
    let mut state = State8080::default();

    state.memory[0x00] = 0xf5;
    state.a = 0x01;
    state.sp = 0x2000;
    state.cc.cy = true;

    let ret = state.emulate_8080();

    match ret {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.memory[0x1fff], 0x01);
    assert_eq!(state.memory[0x1ffe], 0x03);
    assert_eq!(state.sp, 0x1ffe);
}
