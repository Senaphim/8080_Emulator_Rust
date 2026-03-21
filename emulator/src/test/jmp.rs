use crate::state::State8080;

#[test]
fn jmp_none() {
    let mut state = State8080::default();

    state.memory[0] = 0xc3;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_nz() {
    let mut state = State8080::default();

    state.memory[0] = 0xc2;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.z = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_z() {
    let mut state = State8080::default();

    state.memory[0] = 0xca;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.z = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_nc() {
    let mut state = State8080::default();

    state.memory[0] = 0xd2;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.cy = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_c() {
    let mut state = State8080::default();

    state.memory[0] = 0xda;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.cy = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_np() {
    let mut state = State8080::default();

    state.memory[0] = 0xe2;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.p = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_p() {
    let mut state = State8080::default();

    state.memory[0] = 0xea;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.p = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_s() {
    let mut state = State8080::default();

    state.memory[0] = 0xf2;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.s = true;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}

#[test]
fn jmp_ns() {
    let mut state = State8080::default();

    state.memory[0] = 0xfa;
    state.memory[1] = 0x12;
    state.memory[2] = 0x34;
    state.cc.s = false;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x3412)
}
