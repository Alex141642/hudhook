use imgui::Key;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_ADD, VK_B,
    VK_BACK, VK_C, VK_CAPITAL, VK_D, VK_DECIMAL, VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END,
    VK_ESCAPE, VK_EXECUTE, VK_EXSEL, VK_F, VK_F1, VK_F10, VK_F11, VK_F12, VK_F2, VK_F3, VK_F4,
    VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_G, VK_GAMEPAD_A, VK_GAMEPAD_B, VK_GAMEPAD_DPAD_DOWN,
    VK_GAMEPAD_DPAD_LEFT, VK_GAMEPAD_DPAD_RIGHT, VK_GAMEPAD_DPAD_UP, VK_GAMEPAD_LEFT_SHOULDER,
    VK_GAMEPAD_LEFT_THUMBSTICK_DOWN, VK_GAMEPAD_LEFT_THUMBSTICK_LEFT,
    VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT, VK_GAMEPAD_LEFT_THUMBSTICK_UP, VK_GAMEPAD_LEFT_TRIGGER,
    VK_GAMEPAD_MENU, VK_GAMEPAD_RIGHT_SHOULDER, VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN,
    VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT, VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT,
    VK_GAMEPAD_RIGHT_THUMBSTICK_UP, VK_GAMEPAD_RIGHT_TRIGGER, VK_GAMEPAD_VIEW, VK_GAMEPAD_X,
    VK_GAMEPAD_Y, VK_H, VK_HOME, VK_I, VK_INSERT, VK_J, VK_K, VK_L, VK_LBUTTON, VK_LCONTROL,
    VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON, VK_MENU, VK_MULTIPLY, VK_N, VK_NEXT,
    VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6,
    VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5,
    VK_OEM_6, VK_OEM_7, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_PLUS, VK_P, VK_PAUSE,
    VK_PRIOR, VK_Q, VK_R, VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT,
    VK_RWIN, VK_S, VK_SCROLL, VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V,
    VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2, VK_Y, VK_Z,
};

pub(crate) const KEYS: [(Key, VIRTUAL_KEY); 132] = [
    (Key::Tab, VK_TAB),
    (Key::LeftArrow, VK_LEFT),
    (Key::RightArrow, VK_RIGHT),
    (Key::UpArrow, VK_UP),
    (Key::DownArrow, VK_DOWN),
    (Key::PageUp, VK_PRIOR),
    (Key::PageDown, VK_NEXT),
    (Key::Home, VK_HOME),
    (Key::End, VK_END),
    (Key::Insert, VK_INSERT),
    (Key::Delete, VK_DELETE),
    (Key::Backspace, VK_BACK),
    (Key::Space, VK_SPACE),
    (Key::Enter, VK_RETURN),
    (Key::Escape, VK_ESCAPE),
    (Key::LeftCtrl, VK_LCONTROL),
    (Key::LeftShift, VK_LSHIFT),
    (Key::LeftAlt, VK_LMENU),
    (Key::LeftSuper, VK_LWIN),
    (Key::RightCtrl, VK_RCONTROL),
    (Key::RightShift, VK_RSHIFT),
    (Key::RightAlt, VK_RMENU),
    (Key::RightSuper, VK_RWIN),
    (Key::Menu, VK_MENU),
    (Key::Alpha0, VK_0),
    (Key::Alpha1, VK_1),
    (Key::Alpha2, VK_2),
    (Key::Alpha3, VK_3),
    (Key::Alpha4, VK_4),
    (Key::Alpha5, VK_5),
    (Key::Alpha6, VK_6),
    (Key::Alpha7, VK_7),
    (Key::Alpha8, VK_8),
    (Key::Alpha9, VK_9),
    (Key::A, VK_A),
    (Key::B, VK_B),
    (Key::C, VK_C),
    (Key::D, VK_D),
    (Key::E, VK_E),
    (Key::F, VK_F),
    (Key::G, VK_G),
    (Key::H, VK_H),
    (Key::I, VK_I),
    (Key::J, VK_J),
    (Key::K, VK_K),
    (Key::L, VK_L),
    (Key::M, VK_M),
    (Key::N, VK_N),
    (Key::O, VK_O),
    (Key::P, VK_P),
    (Key::Q, VK_Q),
    (Key::R, VK_R),
    (Key::S, VK_S),
    (Key::T, VK_T),
    (Key::U, VK_U),
    (Key::V, VK_V),
    (Key::W, VK_W),
    (Key::X, VK_X),
    (Key::Y, VK_Y),
    (Key::Z, VK_Z),
    (Key::F1, VK_F1),
    (Key::F2, VK_F2),
    (Key::F3, VK_F3),
    (Key::F4, VK_F4),
    (Key::F5, VK_F5),
    (Key::F6, VK_F6),
    (Key::F7, VK_F7),
    (Key::F8, VK_F8),
    (Key::F9, VK_F9),
    (Key::F10, VK_F10),
    (Key::F11, VK_F11),
    (Key::F12, VK_F12),
    (Key::Apostrophe, VK_OEM_7),
    (Key::Comma, VK_OEM_COMMA),
    (Key::Minus, VK_OEM_MINUS),
    (Key::Period, VK_OEM_PERIOD),
    (Key::Slash, VK_OEM_2),
    (Key::Semicolon, VK_OEM_1),
    (Key::Equal, VK_OEM_PLUS),
    (Key::LeftBracket, VK_OEM_4),
    (Key::Backslash, VK_OEM_5),
    (Key::RightBracket, VK_OEM_6),
    (Key::GraveAccent, VK_OEM_3),
    (Key::CapsLock, VK_CAPITAL),
    (Key::ScrollLock, VK_SCROLL),
    (Key::NumLock, VK_NUMLOCK),
    (Key::PrintScreen, VK_SNAPSHOT),
    (Key::Pause, VK_PAUSE),
    (Key::Keypad0, VK_NUMPAD0),
    (Key::Keypad1, VK_NUMPAD1),
    (Key::Keypad2, VK_NUMPAD2),
    (Key::Keypad3, VK_NUMPAD3),
    (Key::Keypad4, VK_NUMPAD4),
    (Key::Keypad5, VK_NUMPAD5),
    (Key::Keypad6, VK_NUMPAD6),
    (Key::Keypad7, VK_NUMPAD7),
    (Key::Keypad8, VK_NUMPAD8),
    (Key::Keypad9, VK_NUMPAD9),
    (Key::KeypadDecimal, VK_DECIMAL),
    (Key::KeypadDivide, VK_DIVIDE),
    (Key::KeypadMultiply, VK_MULTIPLY),
    (Key::KeypadSubtract, VK_SUBTRACT),
    (Key::KeypadAdd, VK_ADD),
    (Key::KeypadEnter, VK_EXECUTE),
    (Key::KeypadEqual, VK_EXSEL),
    (Key::GamepadStart, VK_GAMEPAD_MENU),
    (Key::GamepadBack, VK_GAMEPAD_VIEW),
    (Key::GamepadFaceLeft, VK_GAMEPAD_X),
    (Key::GamepadFaceRight, VK_GAMEPAD_B),
    (Key::GamepadFaceUp, VK_GAMEPAD_Y),
    (Key::GamepadFaceDown, VK_GAMEPAD_A),
    (Key::GamepadDpadLeft, VK_GAMEPAD_DPAD_LEFT),
    (Key::GamepadDpadRight, VK_GAMEPAD_DPAD_RIGHT),
    (Key::GamepadDpadUp, VK_GAMEPAD_DPAD_UP),
    (Key::GamepadDpadDown, VK_GAMEPAD_DPAD_DOWN),
    (Key::GamepadL1, VK_GAMEPAD_LEFT_SHOULDER),
    (Key::GamepadR1, VK_GAMEPAD_RIGHT_SHOULDER),
    (Key::GamepadL2, VK_GAMEPAD_LEFT_TRIGGER),
    (Key::GamepadR2, VK_GAMEPAD_RIGHT_TRIGGER),
    (Key::GamepadLStickLeft, VK_GAMEPAD_LEFT_THUMBSTICK_LEFT),
    (Key::GamepadLStickRight, VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT),
    (Key::GamepadLStickUp, VK_GAMEPAD_LEFT_THUMBSTICK_UP),
    (Key::GamepadLStickDown, VK_GAMEPAD_LEFT_THUMBSTICK_DOWN),
    (Key::GamepadRStickLeft, VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT),
    (Key::GamepadRStickRight, VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT),
    (Key::GamepadRStickUp, VK_GAMEPAD_RIGHT_THUMBSTICK_UP),
    (Key::GamepadRStickDown, VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN),
    (Key::MouseLeft, VK_LBUTTON),
    (Key::MouseRight, VK_RBUTTON),
    (Key::MouseMiddle, VK_MBUTTON),
    (Key::MouseX1, VK_XBUTTON1),
    (Key::MouseX2, VK_XBUTTON2),
    // (Key::MouseWheelX),
    // (Key::MouseWheelY),
    // (Key::ReservedForModCtrl),
    // (Key::ReservedForModShift),
    // (Key::ReservedForModAlt),
    // (Key::ReservedForModSuper),
    // (Key::ModCtrl),
    // (Key::ModShift),
    // (Key::ModAlt),
    // (Key::ModSuper),
    // (Key::ModShortcut),
    // (Key::GamepadL3),
    // (Key::GamepadR3),
];