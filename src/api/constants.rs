/// Constants — color indices and key codes

// Color constants
pub const COLOR_BLACK: u8 = 0;
pub const COLOR_NAVY: u8 = 1;
pub const COLOR_PURPLE: u8 = 2;
pub const COLOR_GREEN: u8 = 3;
pub const COLOR_BROWN: u8 = 4;
pub const COLOR_DARK_BLUE: u8 = 5;
pub const COLOR_LIGHT_BLUE: u8 = 6;
pub const COLOR_WHITE: u8 = 7;
pub const COLOR_RED: u8 = 8;
pub const COLOR_ORANGE: u8 = 9;
pub const COLOR_YELLOW: u8 = 10;
pub const COLOR_LIME: u8 = 11;
pub const COLOR_CYAN: u8 = 12;
pub const COLOR_GRAY: u8 = 13;
pub const COLOR_PINK: u8 = 14;
pub const COLOR_PEACH: u8 = 15;

// Software key IDs retain the original public values for existing games.
// Additional keys use distinct IDs. Use names, not numeric IDs, across backends.
#[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
mod keys_soft {
    pub const KEY_UNKNOWN: u32 = 300;
    pub const KEY_RETURN: u32 = 2;
    pub const KEY_ESCAPE: u32 = 0;
    pub const KEY_BACKSPACE: u32 = 3;
    pub const KEY_TAB: u32 = 4;
    pub const KEY_SPACE: u32 = 1;
    pub const KEY_NONE: u32 = 301;
    pub const KEY_A: u32 = 20;
    pub const KEY_B: u32 = 21;
    pub const KEY_C: u32 = 22;
    pub const KEY_D: u32 = 23;
    pub const KEY_E: u32 = 24;
    pub const KEY_F: u32 = 25;
    pub const KEY_G: u32 = 26;
    pub const KEY_H: u32 = 27;
    pub const KEY_I: u32 = 28;
    pub const KEY_J: u32 = 29;
    pub const KEY_K: u32 = 30;
    pub const KEY_L: u32 = 31;
    pub const KEY_M: u32 = 32;
    pub const KEY_N: u32 = 33;
    pub const KEY_O: u32 = 34;
    pub const KEY_P: u32 = 35;
    pub const KEY_Q: u32 = 36;
    pub const KEY_R: u32 = 37;
    pub const KEY_S: u32 = 38;
    pub const KEY_T: u32 = 39;
    pub const KEY_U: u32 = 40;
    pub const KEY_V: u32 = 41;
    pub const KEY_W: u32 = 42;
    pub const KEY_X: u32 = 43;
    pub const KEY_Y: u32 = 44;
    pub const KEY_Z: u32 = 45;
    pub const KEY_0: u32 = 50;
    pub const KEY_1: u32 = 51;
    pub const KEY_2: u32 = 52;
    pub const KEY_3: u32 = 53;
    pub const KEY_4: u32 = 54;
    pub const KEY_5: u32 = 55;
    pub const KEY_6: u32 = 56;
    pub const KEY_7: u32 = 57;
    pub const KEY_8: u32 = 58;
    pub const KEY_9: u32 = 59;
    pub const KEY_EXCLAIM: u32 = 302;
    pub const KEY_QUOTEDBL: u32 = 303;
    pub const KEY_HASH: u32 = 304;
    pub const KEY_PERCENT: u32 = 305;
    pub const KEY_DOLLAR: u32 = 306;
    pub const KEY_AMPERSAND: u32 = 307;
    pub const KEY_QUOTE: u32 = 308;
    pub const KEY_LEFTPAREN: u32 = 309;
    pub const KEY_RIGHTPAREN: u32 = 310;
    pub const KEY_ASTERISK: u32 = 311;
    pub const KEY_PLUS: u32 = 312;
    pub const KEY_COMMA: u32 = 313;
    pub const KEY_MINUS: u32 = 314;
    pub const KEY_PERIOD: u32 = 315;
    pub const KEY_SLASH: u32 = 316;
    pub const KEY_COLON: u32 = 317;
    pub const KEY_SEMICOLON: u32 = 318;
    pub const KEY_LESS: u32 = 319;
    pub const KEY_EQUALS: u32 = 320;
    pub const KEY_GREATER: u32 = 321;
    pub const KEY_QUESTION: u32 = 322;
    pub const KEY_AT: u32 = 323;
    pub const KEY_LEFTBRACKET: u32 = 324;
    pub const KEY_BACKSLASH: u32 = 325;
    pub const KEY_RIGHTBRACKET: u32 = 326;
    pub const KEY_CARET: u32 = 327;
    pub const KEY_UNDERSCORE: u32 = 328;
    pub const KEY_BACKQUOTE: u32 = 329;
    pub const KEY_F1: u32 = 60;
    pub const KEY_F2: u32 = 61;
    pub const KEY_F3: u32 = 62;
    pub const KEY_F4: u32 = 63;
    pub const KEY_F5: u32 = 64;
    pub const KEY_F6: u32 = 65;
    pub const KEY_F7: u32 = 66;
    pub const KEY_F8: u32 = 67;
    pub const KEY_F9: u32 = 68;
    pub const KEY_F10: u32 = 69;
    pub const KEY_F11: u32 = 70;
    pub const KEY_F12: u32 = 71;
    pub const KEY_F13: u32 = 330;
    pub const KEY_F14: u32 = 331;
    pub const KEY_F15: u32 = 332;
    pub const KEY_F16: u32 = 333;
    pub const KEY_F17: u32 = 334;
    pub const KEY_F18: u32 = 335;
    pub const KEY_F19: u32 = 336;
    pub const KEY_F20: u32 = 337;
    pub const KEY_F21: u32 = 338;
    pub const KEY_F22: u32 = 339;
    pub const KEY_F23: u32 = 340;
    pub const KEY_F24: u32 = 341;
    pub const KEY_INSERT: u32 = 342;
    pub const KEY_HOME: u32 = 343;
    pub const KEY_PAGEUP: u32 = 344;
    pub const KEY_DELETE: u32 = 345;
    pub const KEY_END: u32 = 346;
    pub const KEY_PAGEDOWN: u32 = 347;
    pub const KEY_RIGHT: u32 = 8;
    pub const KEY_LEFT: u32 = 7;
    pub const KEY_DOWN: u32 = 6;
    pub const KEY_UP: u32 = 5;
    pub const KEY_CAPSLOCK: u32 = 348;
    pub const KEY_LCTRL: u32 = 11;
    pub const KEY_LSHIFT: u32 = 9;
    pub const KEY_LALT: u32 = 13;
    pub const KEY_LGUI: u32 = 349;
    pub const KEY_RCTRL: u32 = 12;
    pub const KEY_RSHIFT: u32 = 10;
    pub const KEY_RALT: u32 = 14;
    pub const KEY_RGUI: u32 = 350;
    pub const KEY_SHIFT: u32 = 351;
    pub const KEY_CTRL: u32 = 352;
    pub const KEY_ALT: u32 = 353;
    pub const KEY_GUI: u32 = 354;
    pub const KEY_NUMLOCKCLEAR: u32 = 355;
    pub const KEY_KP_DIVIDE: u32 = 356;
    pub const KEY_KP_MULTIPLY: u32 = 357;
    pub const KEY_KP_MINUS: u32 = 358;
    pub const KEY_KP_PLUS: u32 = 359;
    pub const KEY_KP_ENTER: u32 = 360;
    pub const KEY_KP_1: u32 = 361;
    pub const KEY_KP_2: u32 = 362;
    pub const KEY_KP_3: u32 = 363;
    pub const KEY_KP_4: u32 = 364;
    pub const KEY_KP_5: u32 = 365;
    pub const KEY_KP_6: u32 = 366;
    pub const KEY_KP_7: u32 = 367;
    pub const KEY_KP_8: u32 = 368;
    pub const KEY_KP_9: u32 = 369;
    pub const KEY_KP_0: u32 = 370;
    pub const KEY_KP_PERIOD: u32 = 371;
    pub const KEY_KP_EQUALS: u32 = 372;
    pub const KEY_KP_COMMA: u32 = 373;
    pub const KEY_KP_EQUALSAS400: u32 = 374;
    pub const KEY_KP_00: u32 = 375;
    pub const KEY_KP_000: u32 = 376;
    pub const KEY_KP_LEFTPAREN: u32 = 377;
    pub const KEY_KP_RIGHTPAREN: u32 = 378;
    pub const KEY_KP_LEFTBRACE: u32 = 379;
    pub const KEY_KP_RIGHTBRACE: u32 = 380;
    pub const KEY_KP_TAB: u32 = 381;
    pub const KEY_KP_BACKSPACE: u32 = 382;
    pub const KEY_KP_A: u32 = 383;
    pub const KEY_KP_B: u32 = 384;
    pub const KEY_KP_C: u32 = 385;
    pub const KEY_KP_D: u32 = 386;
    pub const KEY_KP_E: u32 = 387;
    pub const KEY_KP_F: u32 = 388;
    pub const KEY_KP_XOR: u32 = 389;
    pub const KEY_KP_POWER: u32 = 390;
    pub const KEY_KP_PERCENT: u32 = 391;
    pub const KEY_KP_LESS: u32 = 392;
    pub const KEY_KP_GREATER: u32 = 393;
    pub const KEY_KP_AMPERSAND: u32 = 394;
    pub const KEY_KP_DBLAMPERSAND: u32 = 395;
    pub const KEY_KP_VERTICALBAR: u32 = 396;
    pub const KEY_KP_DBLVERTICALBAR: u32 = 397;
    pub const KEY_KP_COLON: u32 = 398;
    pub const KEY_KP_HASH: u32 = 399;
    pub const KEY_KP_SPACE: u32 = 400;
    pub const KEY_KP_AT: u32 = 401;
    pub const KEY_KP_EXCLAM: u32 = 402;
    pub const KEY_KP_MEMSTORE: u32 = 403;
    pub const KEY_KP_MEMRECALL: u32 = 404;
    pub const KEY_KP_MEMCLEAR: u32 = 405;
    pub const KEY_KP_MEMADD: u32 = 406;
    pub const KEY_KP_MEMSUBTRACT: u32 = 407;
    pub const KEY_KP_MEMMULTIPLY: u32 = 408;
    pub const KEY_KP_MEMDIVIDE: u32 = 409;
    pub const KEY_KP_PLUSMINUS: u32 = 410;
    pub const KEY_KP_CLEAR: u32 = 411;
    pub const KEY_KP_CLEARENTRY: u32 = 412;
    pub const KEY_KP_BINARY: u32 = 413;
    pub const KEY_KP_OCTAL: u32 = 414;
    pub const KEY_KP_DECIMAL: u32 = 415;
    pub const KEY_KP_HEXADECIMAL: u32 = 416;
    pub const KEY_PRINTSCREEN: u32 = 417;
    pub const KEY_SCROLLLOCK: u32 = 418;
    pub const KEY_PAUSE: u32 = 419;
    pub const KEY_APPLICATION: u32 = 420;
    pub const KEY_POWER: u32 = 421;
    pub const KEY_EXECUTE: u32 = 422;
    pub const KEY_HELP: u32 = 423;
    pub const KEY_MENU: u32 = 424;
    pub const KEY_SELECT: u32 = 425;
    pub const KEY_STOP: u32 = 426;
    pub const KEY_AGAIN: u32 = 427;
    pub const KEY_UNDO: u32 = 428;
    pub const KEY_CUT: u32 = 429;
    pub const KEY_COPY: u32 = 430;
    pub const KEY_PASTE: u32 = 431;
    pub const KEY_FIND: u32 = 432;
    pub const KEY_MUTE: u32 = 433;
    pub const KEY_VOLUMEUP: u32 = 434;
    pub const KEY_VOLUMEDOWN: u32 = 435;
    pub const KEY_ALTERASE: u32 = 436;
    pub const KEY_SYSREQ: u32 = 437;
    pub const KEY_CANCEL: u32 = 438;
    pub const KEY_CLEAR: u32 = 439;
    pub const KEY_PRIOR: u32 = 440;
    pub const KEY_RETURN2: u32 = 441;
    pub const KEY_SEPARATOR: u32 = 442;
    pub const KEY_OUT: u32 = 443;
    pub const KEY_OPER: u32 = 444;
    pub const KEY_CLEARAGAIN: u32 = 445;
    pub const KEY_CRSEL: u32 = 446;
    pub const KEY_EXSEL: u32 = 447;
    pub const KEY_THOUSANDSSEPARATOR: u32 = 448;
    pub const KEY_DECIMALSEPARATOR: u32 = 449;
    pub const KEY_CURRENCYUNIT: u32 = 450;
    pub const KEY_CURRENCYSUBUNIT: u32 = 451;
    pub const MOUSE_POS_X: u32 = 203;
    pub const MOUSE_POS_Y: u32 = 204;
    pub const MOUSE_WHEEL_X: u32 = 205;
    pub const MOUSE_WHEEL_Y: u32 = 206;
    pub const MOUSE_BUTTON_LEFT: u32 = 200;
    pub const MOUSE_BUTTON_MIDDLE: u32 = 202;
    pub const MOUSE_BUTTON_RIGHT: u32 = 201;
    pub const MOUSE_BUTTON_X1: u32 = 452;
    pub const MOUSE_BUTTON_X2: u32 = 453;
    pub const GAMEPAD1_AXIS_LEFTX: u32 = 454;
    pub const GAMEPAD1_AXIS_LEFTY: u32 = 455;
    pub const GAMEPAD1_AXIS_RIGHTX: u32 = 456;
    pub const GAMEPAD1_AXIS_RIGHTY: u32 = 457;
    pub const GAMEPAD1_AXIS_TRIGGERLEFT: u32 = 458;
    pub const GAMEPAD1_AXIS_TRIGGERRIGHT: u32 = 459;
    pub const GAMEPAD1_BUTTON_A: u32 = 460;
    pub const GAMEPAD1_BUTTON_B: u32 = 461;
    pub const GAMEPAD1_BUTTON_X: u32 = 462;
    pub const GAMEPAD1_BUTTON_Y: u32 = 463;
    pub const GAMEPAD1_BUTTON_BACK: u32 = 464;
    pub const GAMEPAD1_BUTTON_GUIDE: u32 = 465;
    pub const GAMEPAD1_BUTTON_START: u32 = 466;
    pub const GAMEPAD1_BUTTON_LEFTSTICK: u32 = 467;
    pub const GAMEPAD1_BUTTON_RIGHTSTICK: u32 = 468;
    pub const GAMEPAD1_BUTTON_LEFTSHOULDER: u32 = 469;
    pub const GAMEPAD1_BUTTON_RIGHTSHOULDER: u32 = 470;
    pub const GAMEPAD1_BUTTON_DPAD_UP: u32 = 471;
    pub const GAMEPAD1_BUTTON_DPAD_DOWN: u32 = 472;
    pub const GAMEPAD1_BUTTON_DPAD_LEFT: u32 = 473;
    pub const GAMEPAD1_BUTTON_DPAD_RIGHT: u32 = 474;
    pub const GAMEPAD2_AXIS_LEFTX: u32 = 475;
    pub const GAMEPAD2_AXIS_LEFTY: u32 = 476;
    pub const GAMEPAD2_AXIS_RIGHTX: u32 = 477;
    pub const GAMEPAD2_AXIS_RIGHTY: u32 = 478;
    pub const GAMEPAD2_AXIS_TRIGGERLEFT: u32 = 479;
    pub const GAMEPAD2_AXIS_TRIGGERRIGHT: u32 = 480;
    pub const GAMEPAD2_BUTTON_A: u32 = 481;
    pub const GAMEPAD2_BUTTON_B: u32 = 482;
    pub const GAMEPAD2_BUTTON_X: u32 = 483;
    pub const GAMEPAD2_BUTTON_Y: u32 = 484;
    pub const GAMEPAD2_BUTTON_BACK: u32 = 485;
    pub const GAMEPAD2_BUTTON_GUIDE: u32 = 486;
    pub const GAMEPAD2_BUTTON_START: u32 = 487;
    pub const GAMEPAD2_BUTTON_LEFTSTICK: u32 = 488;
    pub const GAMEPAD2_BUTTON_RIGHTSTICK: u32 = 489;
    pub const GAMEPAD2_BUTTON_LEFTSHOULDER: u32 = 490;
    pub const GAMEPAD2_BUTTON_RIGHTSHOULDER: u32 = 491;
    pub const GAMEPAD2_BUTTON_DPAD_UP: u32 = 492;
    pub const GAMEPAD2_BUTTON_DPAD_DOWN: u32 = 493;
    pub const GAMEPAD2_BUTTON_DPAD_LEFT: u32 = 494;
    pub const GAMEPAD2_BUTTON_DPAD_RIGHT: u32 = 495;
    pub const GAMEPAD3_AXIS_LEFTX: u32 = 496;
    pub const GAMEPAD3_AXIS_LEFTY: u32 = 497;
    pub const GAMEPAD3_AXIS_RIGHTX: u32 = 498;
    pub const GAMEPAD3_AXIS_RIGHTY: u32 = 499;
    pub const GAMEPAD3_AXIS_TRIGGERLEFT: u32 = 500;
    pub const GAMEPAD3_AXIS_TRIGGERRIGHT: u32 = 501;
    pub const GAMEPAD3_BUTTON_A: u32 = 502;
    pub const GAMEPAD3_BUTTON_B: u32 = 503;
    pub const GAMEPAD3_BUTTON_X: u32 = 504;
    pub const GAMEPAD3_BUTTON_Y: u32 = 505;
    pub const GAMEPAD3_BUTTON_BACK: u32 = 506;
    pub const GAMEPAD3_BUTTON_GUIDE: u32 = 507;
    pub const GAMEPAD3_BUTTON_START: u32 = 508;
    pub const GAMEPAD3_BUTTON_LEFTSTICK: u32 = 509;
    pub const GAMEPAD3_BUTTON_RIGHTSTICK: u32 = 510;
    pub const GAMEPAD3_BUTTON_LEFTSHOULDER: u32 = 511;
    pub const GAMEPAD3_BUTTON_RIGHTSHOULDER: u32 = 512;
    pub const GAMEPAD3_BUTTON_DPAD_UP: u32 = 513;
    pub const GAMEPAD3_BUTTON_DPAD_DOWN: u32 = 514;
    pub const GAMEPAD3_BUTTON_DPAD_LEFT: u32 = 515;
    pub const GAMEPAD3_BUTTON_DPAD_RIGHT: u32 = 516;
    pub const GAMEPAD4_AXIS_LEFTX: u32 = 517;
    pub const GAMEPAD4_AXIS_LEFTY: u32 = 518;
    pub const GAMEPAD4_AXIS_RIGHTX: u32 = 519;
    pub const GAMEPAD4_AXIS_RIGHTY: u32 = 520;
    pub const GAMEPAD4_AXIS_TRIGGERLEFT: u32 = 521;
    pub const GAMEPAD4_AXIS_TRIGGERRIGHT: u32 = 522;
    pub const GAMEPAD4_BUTTON_A: u32 = 523;
    pub const GAMEPAD4_BUTTON_B: u32 = 524;
    pub const GAMEPAD4_BUTTON_X: u32 = 525;
    pub const GAMEPAD4_BUTTON_Y: u32 = 526;
    pub const GAMEPAD4_BUTTON_BACK: u32 = 527;
    pub const GAMEPAD4_BUTTON_GUIDE: u32 = 528;
    pub const GAMEPAD4_BUTTON_START: u32 = 529;
    pub const GAMEPAD4_BUTTON_LEFTSTICK: u32 = 530;
    pub const GAMEPAD4_BUTTON_RIGHTSTICK: u32 = 531;
    pub const GAMEPAD4_BUTTON_LEFTSHOULDER: u32 = 532;
    pub const GAMEPAD4_BUTTON_RIGHTSHOULDER: u32 = 533;
    pub const GAMEPAD4_BUTTON_DPAD_UP: u32 = 534;
    pub const GAMEPAD4_BUTTON_DPAD_DOWN: u32 = 535;
    pub const GAMEPAD4_BUTTON_DPAD_LEFT: u32 = 536;
    pub const GAMEPAD4_BUTTON_DPAD_RIGHT: u32 = 537;
}

#[cfg(feature = "pyxel-core-backend")]
mod keys_pyxel {
    pub use pyxel::{
        GAMEPAD1_AXIS_LEFTX, GAMEPAD1_AXIS_LEFTY, GAMEPAD1_AXIS_RIGHTX, GAMEPAD1_AXIS_RIGHTY,
        GAMEPAD1_AXIS_TRIGGERLEFT, GAMEPAD1_AXIS_TRIGGERRIGHT, GAMEPAD1_BUTTON_A,
        GAMEPAD1_BUTTON_B, GAMEPAD1_BUTTON_BACK, GAMEPAD1_BUTTON_DPAD_DOWN,
        GAMEPAD1_BUTTON_DPAD_LEFT, GAMEPAD1_BUTTON_DPAD_RIGHT, GAMEPAD1_BUTTON_DPAD_UP,
        GAMEPAD1_BUTTON_GUIDE, GAMEPAD1_BUTTON_LEFTSHOULDER, GAMEPAD1_BUTTON_LEFTSTICK,
        GAMEPAD1_BUTTON_RIGHTSHOULDER, GAMEPAD1_BUTTON_RIGHTSTICK, GAMEPAD1_BUTTON_START,
        GAMEPAD1_BUTTON_X, GAMEPAD1_BUTTON_Y, GAMEPAD2_AXIS_LEFTX, GAMEPAD2_AXIS_LEFTY,
        GAMEPAD2_AXIS_RIGHTX, GAMEPAD2_AXIS_RIGHTY, GAMEPAD2_AXIS_TRIGGERLEFT,
        GAMEPAD2_AXIS_TRIGGERRIGHT, GAMEPAD2_BUTTON_A, GAMEPAD2_BUTTON_B, GAMEPAD2_BUTTON_BACK,
        GAMEPAD2_BUTTON_DPAD_DOWN, GAMEPAD2_BUTTON_DPAD_LEFT, GAMEPAD2_BUTTON_DPAD_RIGHT,
        GAMEPAD2_BUTTON_DPAD_UP, GAMEPAD2_BUTTON_GUIDE, GAMEPAD2_BUTTON_LEFTSHOULDER,
        GAMEPAD2_BUTTON_LEFTSTICK, GAMEPAD2_BUTTON_RIGHTSHOULDER, GAMEPAD2_BUTTON_RIGHTSTICK,
        GAMEPAD2_BUTTON_START, GAMEPAD2_BUTTON_X, GAMEPAD2_BUTTON_Y, GAMEPAD3_AXIS_LEFTX,
        GAMEPAD3_AXIS_LEFTY, GAMEPAD3_AXIS_RIGHTX, GAMEPAD3_AXIS_RIGHTY, GAMEPAD3_AXIS_TRIGGERLEFT,
        GAMEPAD3_AXIS_TRIGGERRIGHT, GAMEPAD3_BUTTON_A, GAMEPAD3_BUTTON_B, GAMEPAD3_BUTTON_BACK,
        GAMEPAD3_BUTTON_DPAD_DOWN, GAMEPAD3_BUTTON_DPAD_LEFT, GAMEPAD3_BUTTON_DPAD_RIGHT,
        GAMEPAD3_BUTTON_DPAD_UP, GAMEPAD3_BUTTON_GUIDE, GAMEPAD3_BUTTON_LEFTSHOULDER,
        GAMEPAD3_BUTTON_LEFTSTICK, GAMEPAD3_BUTTON_RIGHTSHOULDER, GAMEPAD3_BUTTON_RIGHTSTICK,
        GAMEPAD3_BUTTON_START, GAMEPAD3_BUTTON_X, GAMEPAD3_BUTTON_Y, GAMEPAD4_AXIS_LEFTX,
        GAMEPAD4_AXIS_LEFTY, GAMEPAD4_AXIS_RIGHTX, GAMEPAD4_AXIS_RIGHTY, GAMEPAD4_AXIS_TRIGGERLEFT,
        GAMEPAD4_AXIS_TRIGGERRIGHT, GAMEPAD4_BUTTON_A, GAMEPAD4_BUTTON_B, GAMEPAD4_BUTTON_BACK,
        GAMEPAD4_BUTTON_DPAD_DOWN, GAMEPAD4_BUTTON_DPAD_LEFT, GAMEPAD4_BUTTON_DPAD_RIGHT,
        GAMEPAD4_BUTTON_DPAD_UP, GAMEPAD4_BUTTON_GUIDE, GAMEPAD4_BUTTON_LEFTSHOULDER,
        GAMEPAD4_BUTTON_LEFTSTICK, GAMEPAD4_BUTTON_RIGHTSHOULDER, GAMEPAD4_BUTTON_RIGHTSTICK,
        GAMEPAD4_BUTTON_START, GAMEPAD4_BUTTON_X, GAMEPAD4_BUTTON_Y, KEY_0, KEY_1, KEY_2, KEY_3,
        KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9, KEY_A, KEY_AGAIN, KEY_ALT, KEY_ALTERASE,
        KEY_AMPERSAND, KEY_APPLICATION, KEY_ASTERISK, KEY_AT, KEY_B, KEY_BACKQUOTE, KEY_BACKSLASH,
        KEY_BACKSPACE, KEY_C, KEY_CANCEL, KEY_CAPSLOCK, KEY_CARET, KEY_CLEAR, KEY_CLEARAGAIN,
        KEY_COLON, KEY_COMMA, KEY_COPY, KEY_CRSEL, KEY_CTRL, KEY_CURRENCYSUBUNIT, KEY_CURRENCYUNIT,
        KEY_CUT, KEY_D, KEY_DECIMALSEPARATOR, KEY_DELETE, KEY_DOLLAR, KEY_DOWN, KEY_E, KEY_END,
        KEY_EQUALS, KEY_ESCAPE, KEY_EXCLAIM, KEY_EXECUTE, KEY_EXSEL, KEY_F, KEY_F1, KEY_F10,
        KEY_F11, KEY_F12, KEY_F13, KEY_F14, KEY_F15, KEY_F16, KEY_F17, KEY_F18, KEY_F19, KEY_F2,
        KEY_F20, KEY_F21, KEY_F22, KEY_F23, KEY_F24, KEY_F3, KEY_F4, KEY_F5, KEY_F6, KEY_F7,
        KEY_F8, KEY_F9, KEY_FIND, KEY_G, KEY_GREATER, KEY_GUI, KEY_H, KEY_HASH, KEY_HELP, KEY_HOME,
        KEY_I, KEY_INSERT, KEY_J, KEY_K, KEY_KP_0, KEY_KP_00, KEY_KP_000, KEY_KP_1, KEY_KP_2,
        KEY_KP_3, KEY_KP_4, KEY_KP_5, KEY_KP_6, KEY_KP_7, KEY_KP_8, KEY_KP_9, KEY_KP_A,
        KEY_KP_AMPERSAND, KEY_KP_AT, KEY_KP_B, KEY_KP_BACKSPACE, KEY_KP_BINARY, KEY_KP_C,
        KEY_KP_CLEAR, KEY_KP_CLEARENTRY, KEY_KP_COLON, KEY_KP_COMMA, KEY_KP_D, KEY_KP_DBLAMPERSAND,
        KEY_KP_DBLVERTICALBAR, KEY_KP_DECIMAL, KEY_KP_DIVIDE, KEY_KP_E, KEY_KP_ENTER,
        KEY_KP_EQUALS, KEY_KP_EQUALSAS400, KEY_KP_EXCLAM, KEY_KP_F, KEY_KP_GREATER, KEY_KP_HASH,
        KEY_KP_HEXADECIMAL, KEY_KP_LEFTBRACE, KEY_KP_LEFTPAREN, KEY_KP_LESS, KEY_KP_MEMADD,
        KEY_KP_MEMCLEAR, KEY_KP_MEMDIVIDE, KEY_KP_MEMMULTIPLY, KEY_KP_MEMRECALL, KEY_KP_MEMSTORE,
        KEY_KP_MEMSUBTRACT, KEY_KP_MINUS, KEY_KP_MULTIPLY, KEY_KP_OCTAL, KEY_KP_PERCENT,
        KEY_KP_PERIOD, KEY_KP_PLUS, KEY_KP_PLUSMINUS, KEY_KP_POWER, KEY_KP_RIGHTBRACE,
        KEY_KP_RIGHTPAREN, KEY_KP_SPACE, KEY_KP_TAB, KEY_KP_VERTICALBAR, KEY_KP_XOR, KEY_L,
        KEY_LALT, KEY_LCTRL, KEY_LEFT, KEY_LEFTBRACKET, KEY_LEFTPAREN, KEY_LESS, KEY_LGUI,
        KEY_LSHIFT, KEY_M, KEY_MENU, KEY_MINUS, KEY_MUTE, KEY_N, KEY_NONE, KEY_NUMLOCKCLEAR, KEY_O,
        KEY_OPER, KEY_OUT, KEY_P, KEY_PAGEDOWN, KEY_PAGEUP, KEY_PASTE, KEY_PAUSE, KEY_PERCENT,
        KEY_PERIOD, KEY_PLUS, KEY_POWER, KEY_PRINTSCREEN, KEY_PRIOR, KEY_Q, KEY_QUESTION,
        KEY_QUOTE, KEY_QUOTEDBL, KEY_R, KEY_RALT, KEY_RCTRL, KEY_RETURN, KEY_RETURN2, KEY_RGUI,
        KEY_RIGHT, KEY_RIGHTBRACKET, KEY_RIGHTPAREN, KEY_RSHIFT, KEY_S, KEY_SCROLLLOCK, KEY_SELECT,
        KEY_SEMICOLON, KEY_SEPARATOR, KEY_SHIFT, KEY_SLASH, KEY_SPACE, KEY_STOP, KEY_SYSREQ, KEY_T,
        KEY_TAB, KEY_THOUSANDSSEPARATOR, KEY_U, KEY_UNDERSCORE, KEY_UNDO, KEY_UNKNOWN, KEY_UP,
        KEY_V, KEY_VOLUMEDOWN, KEY_VOLUMEUP, KEY_W, KEY_X, KEY_Y, KEY_Z, MOUSE_BUTTON_LEFT,
        MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT, MOUSE_BUTTON_X1, MOUSE_BUTTON_X2, MOUSE_POS_X,
        MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y,
    };
}

#[cfg(feature = "pyxel-core-backend")]
pub use keys_pyxel::*;
#[cfg(any(feature = "wgpu-backend", feature = "wasm-backend"))]
pub use keys_soft::*;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_COLORS: u32 = 16;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_IMAGES: u32 = 3;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const IMAGE_SIZE: u32 = 256;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_TILEMAPS: u32 = 8;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const TILEMAP_SIZE: u32 = 256;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const TILE_SIZE: u32 = 8;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const FONT_WIDTH: u32 = 4;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const FONT_HEIGHT: u32 = 6;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const TONE_TRIANGLE: u8 = 0;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const TONE_SQUARE: u8 = 1;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const TONE_PULSE: u8 = 2;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const TONE_NOISE: u8 = 3;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const EFFECT_NONE: u8 = 0;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const EFFECT_SLIDE: u8 = 1;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const EFFECT_VIBRATO: u8 = 2;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const EFFECT_FADEOUT: u8 = 3;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const EFFECT_HALF_FADEOUT: u8 = 4;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const EFFECT_QUARTER_FADEOUT: u8 = 5;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_CHANNELS: u32 = 4;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_TONES: u32 = 4;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_SOUNDS: u32 = 64;

/// Pyxel 3.0 resource layout / synthesis constant; availability of an API is separate.
pub const NUM_MUSICS: u32 = 8;
