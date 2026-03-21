use crate::State8080;

#[test]
fn ldax_b() {
    let mut state = State8080::default();

    state.memory[0] = 0x0a;
    state.memory[0x1234] = 0x56;
    state.b = 0x12;
    state.c = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x56);
}

#[test]
fn ldax_d() {
    let mut state = State8080::default();

    state.memory[0] = 0x1a;
    state.memory[0x1234] = 0x56;
    state.d = 0x12;
    state.e = 0x34;

    let res = state.emulate_8080();

    match res {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    assert_eq!(state.pc, 0x01);
    assert_eq!(state.a, 0x56);
}
