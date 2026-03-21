#[cfg(test)]
mod test_mov {
    use crate::State8080;

    #[test]
    fn mov_bb() {
        let mut state = State8080::default();

        state.memory[0] = 0x40;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.c, 0x00);
        assert_eq!(state.d, 0x00);
        assert_eq!(state.e, 0x00);
        assert_eq!(state.h, 0x00);
        assert_eq!(state.l, 0x00);
    }

    #[test]
    fn mov_bc() {
        let mut state = State8080::default();

        state.memory[0] = 0x41;
        state.c = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.c, 0x11);
    }

    #[test]
    fn mov_bd() {
        let mut state = State8080::default();

        state.memory[0] = 0x42;
        state.d = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.d, 0x11);
    }

    #[test]
    fn mov_be() {
        let mut state = State8080::default();

        state.memory[0] = 0x43;
        state.e = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.e, 0x11);
    }

    #[test]
    fn mov_bh() {
        let mut state = State8080::default();

        state.memory[0] = 0x44;
        state.h = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.h, 0x11);
    }

    #[test]
    fn mov_bl() {
        let mut state = State8080::default();

        state.memory[0] = 0x45;
        state.l = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.l, 0x11);
    }

    #[test]
    fn mov_bm() {
        let mut state = State8080::default();

        state.memory[0] = 0x46;
        state.h = 0x12;
        state.l = 0x34;
        state.memory[0x1234] = 0x22;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x22);
    }

    #[test]
    fn mov_ba() {
        let mut state = State8080::default();

        state.memory[0] = 0x47;
        state.a = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.b, 0x11);
        assert_eq!(state.a, 0x11);
    }

    #[test]
    fn mov_ab() {
        let mut state = State8080::default();

        state.memory[0] = 0x78;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.a, 0x11);
        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn mov_cb() {
        let mut state = State8080::default();

        state.memory[0] = 0x48;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.c, 0x11);
        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn mov_db() {
        let mut state = State8080::default();

        state.memory[0] = 0x50;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.d, 0x11);
        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn mov_eb() {
        let mut state = State8080::default();

        state.memory[0] = 0x58;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.e, 0x11);
        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn mov_hb() {
        let mut state = State8080::default();

        state.memory[0] = 0x60;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.h, 0x11);
        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn mov_lb() {
        let mut state = State8080::default();

        state.memory[0] = 0x68;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.l, 0x11);
        assert_eq!(state.b, 0x11);
    }

    #[test]
    fn mov_mb() {
        let mut state = State8080::default();

        state.memory[0] = 0x70;
        state.h = 0x12;
        state.l = 0x34;
        state.b = 0x11;

        let res = state.emulate_8080();

        match res {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }

        assert_eq!(state.pc, 0x01);
        assert_eq!(state.memory[0x1234], 0x11);
        assert_eq!(state.b, 0x11);
    }
}
