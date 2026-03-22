use crate::state::State8080;

#[test]
fn call_none() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xcd;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_nz() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xc4;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.z = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_z() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xcc;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.z = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_ncy() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xd4;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.cy = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_cy() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xdc;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.cy = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_np() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xe4;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.p = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_p() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xec;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.p = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_s() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xf4;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.s = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}

#[test]
fn call_ns() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xfc;
    state.memory[0x1011] = 0x12;
    state.memory[0x1012] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.s = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412);
    assert_eq!(state.memory[0x3fff], 0x10);
    assert_eq!(state.memory[0x3ffe], 0x10);
    assert_eq!(state.sp, 0x3ffe);
}
