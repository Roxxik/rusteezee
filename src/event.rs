use glium::glutin::Event as GlEvent;

use types::{ HDirection, VDirection };

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
    Attack,
    UseItem,
    None,
}

impl From<GlEvent> for Event {
    fn from(ev: GlEvent) -> Self {
        use glium::glutin::Event as E;
        use glium::glutin::ElementState::Pressed;
        use glium::glutin::VirtualKeyCode as V;
        use glium::glutin::MouseButton as M;
        use super::event::Event::*;
        use types::HDirection::*;
        use types::VDirection::*;
        match ev {
            E::MouseInput(Pressed, M::Left ) => Attack,
            E::MouseInput(Pressed, M::Right) => UseItem,
            E::KeyboardInput(state, _, Some(key)) => {
                let t = state == Pressed;
                match (state, key) {
                    (_      , V::W)      => Move { dir: Forth, toogle: t },
                    (_      , V::A)      => Move { dir: Left , toogle: t },
                    (_      , V::S)      => Move { dir: Back , toogle: t },
                    (_      , V::D)      => Move { dir: Right, toogle: t },
                    (_      , V::Up)     => Turn { dir: Forth, toogle: t },
                    (_      , V::Left)   => Turn { dir: Left , toogle: t },
                    (_      , V::Down)   => Turn { dir: Back , toogle: t },
                    (_      , V::Right)  => Turn { dir: Right, toogle: t },
                    (_      , V::Space)  => Fly  { dir: Up   , toogle: t },
                    (_      , V::LShift) => Fly  { dir: Down , toogle: t },
                    _ => None,
                }

            },
            _ => None,
        }
    }
}
