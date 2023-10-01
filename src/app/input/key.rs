#[allow(dead_code)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Key {
  Escape = 1,
  Key1,
  Key2,
  Key3,
  Key4,
  Key5,
  Key6,
  Key7,
  Key8,
  Key9,
  Key0 = 41,

  KeyQ = 16,
  KeyW,
  KeyE,
  KeyR,
  KeyT,
  KeyY,
  KeyU,
  KeyI,
  KeyO,
  KeyP,
  KeyA = 30,
  KeyS,
  KeyD,
  KeyF,
  KeyG,
  KeyH,
  KeyJ,
  KeyK,
  KeyL,
}

#[allow(dead_code)]
const VARIANTS: [Key; 30] = [
  Key::Escape, 
  Key::Key0, Key::Key1, Key::Key2, Key::Key3, Key::Key4, 
  Key::Key5, Key::Key6, Key::Key7, Key::Key8, Key::Key9,

  Key::KeyQ, Key::KeyW, Key::KeyE, Key::KeyR, Key::KeyT, Key::KeyY, Key::KeyU, Key::KeyI, Key::KeyO, Key::KeyP,
  Key::KeyA, Key::KeyS, Key::KeyD, Key::KeyF, Key::KeyG, Key::KeyH, Key::KeyJ, Key::KeyK, Key::KeyL,  
];

#[allow(dead_code)]
impl Key {
  pub fn debug() {
    for variant in VARIANTS {
      let value = variant as u32;
      println!("{:?}: {}", variant, value);
    }
  }
}
