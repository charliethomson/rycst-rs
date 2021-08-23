use sdl2::keyboard::Keycode;
use std::{collections::HashMap, iter::FromIterator};

lazy_static::lazy_static! {
    static ref initial_map: HashMap<Keycode, bool> = {
        HashMap::from_iter(ALL_KEYCODE_VARIANTS.iter().copied().map(|k| (k, false)))
    };
}

pub struct KeyStateHandler(HashMap<Keycode, bool>);
impl KeyStateHandler {
    pub fn new() -> Self {
        Self(initial_map.clone())
    }

    pub fn press(&mut self, keycode: Keycode) {
        self.0.insert(keycode, true).unwrap();
    }

    pub fn release(&mut self, keycode: Keycode) {
        self.0.insert(keycode, false).unwrap();
    }

    pub fn is_pressed(&self, keycode: Keycode) -> bool {
        self.0.get(&keycode).copied().unwrap_or_default()
    }
}

#[rustfmt::skip] const ALL_KEYCODE_VARIANTS: [Keycode; 235] = [Keycode::Backspace,Keycode::Tab,Keycode::Return,Keycode::Escape,Keycode::Space,Keycode::Exclaim,Keycode::Quotedbl,Keycode::Hash,Keycode::Dollar,Keycode::Percent,Keycode::Ampersand,Keycode::Quote,Keycode::LeftParen,Keycode::RightParen,Keycode::Asterisk,Keycode::Plus,Keycode::Comma,Keycode::Minus,Keycode::Period,Keycode::Slash,Keycode::Num0,Keycode::Num1,Keycode::Num2,Keycode::Num3,Keycode::Num4,Keycode::Num5,Keycode::Num6,Keycode::Num7,Keycode::Num8,Keycode::Num9,Keycode::Colon,Keycode::Semicolon,Keycode::Less,Keycode::Equals,Keycode::Greater,Keycode::Question,Keycode::At,Keycode::LeftBracket,Keycode::Backslash,Keycode::RightBracket,Keycode::Caret,Keycode::Underscore,Keycode::Backquote,Keycode::A,Keycode::B,Keycode::C,Keycode::D,Keycode::E,Keycode::F,Keycode::G,Keycode::H,Keycode::I,Keycode::J,Keycode::K,Keycode::L,Keycode::M,Keycode::N,Keycode::O,Keycode::P,Keycode::Q,Keycode::R,Keycode::S,Keycode::T,Keycode::U,Keycode::V,Keycode::W,Keycode::X,Keycode::Y,Keycode::Z,Keycode::Delete,Keycode::CapsLock,Keycode::F1,Keycode::F2,Keycode::F3,Keycode::F4,Keycode::F5,Keycode::F6,Keycode::F7,Keycode::F8,Keycode::F9,Keycode::F10,Keycode::F11,Keycode::F12,Keycode::PrintScreen,Keycode::ScrollLock,Keycode::Pause,Keycode::Insert,Keycode::Home,Keycode::PageUp,Keycode::End,Keycode::PageDown,Keycode::Right,Keycode::Left,Keycode::Down,Keycode::Up,Keycode::NumLockClear,Keycode::KpDivide,Keycode::KpMultiply,Keycode::KpMinus,Keycode::KpPlus,Keycode::KpEnter,Keycode::Kp1,Keycode::Kp2,Keycode::Kp3,Keycode::Kp4,Keycode::Kp5,Keycode::Kp6,Keycode::Kp7,Keycode::Kp8,Keycode::Kp9,Keycode::Kp0,Keycode::KpPeriod,Keycode::Application,Keycode::Power,Keycode::KpEquals,Keycode::F13,Keycode::F14,Keycode::F15,Keycode::F16,Keycode::F17,Keycode::F18,Keycode::F19,Keycode::F20,Keycode::F21,Keycode::F22,Keycode::F23,Keycode::F24,Keycode::Execute,Keycode::Help,Keycode::Menu,Keycode::Select,Keycode::Stop,Keycode::Again,Keycode::Undo,Keycode::Cut,Keycode::Copy,Keycode::Paste,Keycode::Find,Keycode::Mute,Keycode::VolumeUp,Keycode::VolumeDown,Keycode::KpComma,Keycode::KpEqualsAS400,Keycode::AltErase,Keycode::Sysreq,Keycode::Cancel,Keycode::Clear,Keycode::Prior,Keycode::Return2,Keycode::Separator,Keycode::Out,Keycode::Oper,Keycode::ClearAgain,Keycode::CrSel,Keycode::ExSel,Keycode::Kp00,Keycode::Kp000,Keycode::ThousandsSeparator,Keycode::DecimalSeparator,Keycode::CurrencyUnit,Keycode::CurrencySubUnit,Keycode::KpLeftParen,Keycode::KpRightParen,Keycode::KpLeftBrace,Keycode::KpRightBrace,Keycode::KpTab,Keycode::KpBackspace,Keycode::KpA,Keycode::KpB,Keycode::KpC,Keycode::KpD,Keycode::KpE,Keycode::KpF,Keycode::KpXor,Keycode::KpPower,Keycode::KpPercent,Keycode::KpLess,Keycode::KpGreater,Keycode::KpAmpersand,Keycode::KpDblAmpersand,Keycode::KpVerticalBar,Keycode::KpDblVerticalBar,Keycode::KpColon,Keycode::KpHash,Keycode::KpSpace,Keycode::KpAt,Keycode::KpExclam,Keycode::KpMemStore,Keycode::KpMemRecall,Keycode::KpMemClear,Keycode::KpMemAdd,Keycode::KpMemSubtract,Keycode::KpMemMultiply,Keycode::KpMemDivide,Keycode::KpPlusMinus,Keycode::KpClear,Keycode::KpClearEntry,Keycode::KpBinary,Keycode::KpOctal,Keycode::KpDecimal,Keycode::KpHexadecimal,Keycode::LCtrl,Keycode::LShift,Keycode::LAlt,Keycode::LGui,Keycode::RCtrl,Keycode::RShift,Keycode::RAlt,Keycode::RGui,Keycode::Mode,Keycode::AudioNext,Keycode::AudioPrev,Keycode::AudioStop,Keycode::AudioPlay,Keycode::AudioMute,Keycode::MediaSelect,Keycode::Www,Keycode::Mail,Keycode::Calculator,Keycode::Computer,Keycode::AcSearch,Keycode::AcHome,Keycode::AcBack,Keycode::AcForward,Keycode::AcStop,Keycode::AcRefresh,Keycode::AcBookmarks,Keycode::BrightnessDown,Keycode::BrightnessUp,Keycode::DisplaySwitch,Keycode::KbdIllumToggle,Keycode::KbdIllumDown,Keycode::KbdIllumUp,Keycode::Eject,Keycode::Sleep,];
