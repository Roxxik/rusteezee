use super::{ HDirection, VDirection };

pub enum Event {
    Turn {
        dir: HDirection,
        toogle: bool,
    },
    Move {
        dir: HDirection,
        toogle: bool,
    },
    Fly {
        dir: VDirection,
        toogle: bool,
    },
    Jump,
    Sneak {
        toogle: bool,
    },
    ToogleBlock {
        block: (i32, i32, i32),
    },
}
