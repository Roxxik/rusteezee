use glium::{ self, glutin };

use super::{ HDirection, VDirection };

pub enum Event {
    CameraTurn {
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
        block: usize,
    },
    Exit,
}
