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

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_nz() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xc0;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.z = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_z() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xc8;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.z = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_ncy() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xd0;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.cy = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_cy() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xd8;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.cy = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_np() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xe0;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.p = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_p() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xe8;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.p = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_s() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xf0;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.s = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}

#[test]
fn ret_ns() {
    let mut state = State8080::default();

    state.memory[0x1010] = 0xf8;
    state.memory[0x4000] = 0x12;
    state.memory[0x4001] = 0x34;
    state.sp = 0x4000;
    state.pc = 0x1010;
    state.cc.s = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3413);
    assert_eq!(state.sp, 0x4002);
}
