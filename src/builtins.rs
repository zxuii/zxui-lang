use crate::object::Value;

use indexmap::IndexMap;
use libloading::{Library, Symbol};
use std::{
    cell::RefCell,
    ffi::CString,
    io::{self, Write},
    rc::Rc,
};

// helper permudah hidup
fn expect_number(v: &Value, fname: &str, i: usize) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        other => Err(format!(
            "{}(): argument {} must be a number, got '{}'.",
            fname,
            i + 1,
            other
        )),
    }
}

fn expect_string(v: &Value, fname: &str, i: usize) -> Result<String, String> {
    match v {
        Value::String(s) => Ok(s.clone()),
        other => Err(format!(
            "{}(): argument {} must be a string, got '{}'.",
            fname,
            i + 1,
            other
        )),
    }
}

// fn expect_boolean(v: &Value, fname: &str, i: usize) -> Result<bool, String> {
//     match v {
//         Value::Boolean(b) => Ok(*b),
//         other => Err(format!("{}(): argument {} must be a boolean, got '{}'.", fname, i + 1, other)),
//     }
// }

// -------------------------- UNTUK RAYLIB --------------------------

// fungsi fungsi raylib
type InitWindowFn = unsafe extern "C" fn(width: i32, height: i32, title: *const i8);
type WindowShouldCloseFn = unsafe extern "C" fn() -> bool;
type BeginDrawingFn = unsafe extern "C" fn();
type EndDrawingFn = unsafe extern "C" fn();
type CloseWindowFn = unsafe extern "C" fn();
// sebenarnya di raylib color itu struct yang berisi 4 biji u8 `R`, `G`, `B`, `A`
// tapi karena itu bisa di taruh di i32 dan mengsimplifikasi segalanya kenapa engga
type ClearBackgroundFn = unsafe extern "C" fn(color: u32);
type DrawRectangleFn =
    unsafe extern "C" fn(pos_x: i32, pos_y: i32, width: i32, height: i32, color: u32);
type IsKeyDownFn = unsafe extern "C" fn(key: i32) -> bool;
type GetFrameTimeFn = unsafe extern "C" fn() -> f32;

// untuk mempermudah buat struct
pub struct Raylib {
    _lib: Library,
    pub init_window: InitWindowFn,
    pub window_should_close: WindowShouldCloseFn,
    pub begin_drawing: BeginDrawingFn,
    pub end_drawing: EndDrawingFn,
    pub close_window: CloseWindowFn,
    pub clear_background: ClearBackgroundFn,
    pub draw_rectangle: DrawRectangleFn,
    pub is_key_down: IsKeyDownFn,
    pub get_frame_time: GetFrameTimeFn,
}

impl Raylib {
    pub fn new(lib_path: String) -> Result<Self, libloading::Error> {
        unsafe {
            let lib = Library::new(lib_path)?;
            let init_window = {
                let sym: Symbol<InitWindowFn> = lib.get(b"InitWindow\0")?;
                *sym
            };
            let window_should_close = {
                let sym: Symbol<WindowShouldCloseFn> = lib.get(b"WindowShouldClose\0")?;
                *sym
            };
            let begin_drawing = {
                let sym: Symbol<BeginDrawingFn> = lib.get(b"BeginDrawing\0")?;
                *sym
            };
            let end_drawing = {
                let sym: Symbol<EndDrawingFn> = lib.get(b"EndDrawing\0")?;
                *sym
            };
            let close_window = {
                let sym: Symbol<CloseWindowFn> = lib.get(b"CloseWindow\0")?;
                *sym
            };
            let clear_background = {
                let sym: Symbol<ClearBackgroundFn> = lib.get(b"ClearBackground\0")?;
                *sym
            };
            let draw_rectangle = {
                let sym: Symbol<DrawRectangleFn> = lib.get(b"DrawRectangle\0")?;
                *sym
            };
            let is_key_down = {
                let sym: Symbol<IsKeyDownFn> = lib.get(b"IsKeyDown\0")?;
                *sym
            };
            let get_frame_time = {
                let sym: Symbol<GetFrameTimeFn> = lib.get(b"GetFrameTime\0")?;
                *sym
            };
            Ok(Self {
                _lib: lib,
                init_window,
                window_should_close,
                begin_drawing,
                end_drawing,
                close_window,
                clear_background,
                draw_rectangle,
                is_key_down,
                get_frame_time,
            })
        }
    }
}

pub fn raylib_init_window(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "initWindow".to_string(),
        3,
        Rc::new(move |args| -> Result<Value, String> {
            let width = expect_number(&args[0], "initWindow", 0)? as i32;
            let height = expect_number(&args[1], "initWindow", 1)? as i32;
            let title = expect_string(&args[2], "initWindow", 2)?;
            let title_c = CString::new(title).unwrap();

            unsafe { (raylib.init_window)(width, height, title_c.as_ptr()) };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_windows_should_close(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "windowShouldClose".to_string(),
        0,
        Rc::new(move |_| -> Result<Value, String> {
            let val = unsafe { (raylib.window_should_close)() };
            Ok(Value::Boolean(val))
        }),
    )
}

pub fn raylib_begin_drawing(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "beginDrawing".to_string(),
        0,
        Rc::new(move |_| -> Result<Value, String> {
            unsafe { (raylib.begin_drawing)() };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_end_drawing(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "endDrawing".to_string(),
        0,
        Rc::new(move |_| -> Result<Value, String> {
            unsafe { (raylib.end_drawing)() };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_close_window(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "closeWindow".to_string(),
        0,
        Rc::new(move |_| -> Result<Value, String> {
            unsafe { (raylib.close_window)() };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_get_frame_time(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "getFrameTime".to_string(),
        0,
        Rc::new(move |_| -> Result<Value, String> {
            let val = unsafe { (raylib.get_frame_time)() };
            Ok(Value::Number(val as f64))
        }),
    )
}

fn resolve_color(color: &str) -> Result<u32, String> {
    // diambil dari AI juga biar cepet dan asli akurat
    match color {
        "light_gray" => Ok(0xC8C8C8FF),
        "gray" => Ok(0x828282FF),
        "dark_gray" => Ok(0x505050FF),
        "yellow" => Ok(0xFDF900FF),
        "gold" => Ok(0xFFCB00FF),
        "orange" => Ok(0xFFA100FF),
        "pink" => Ok(0xFF6DC2FF),
        "red" => Ok(0xE62937FF),
        "maroon" => Ok(0xBE2137FF),
        "green" => Ok(0x00E430FF),
        "lime" => Ok(0x00AA2CFF),
        "dark_green" => Ok(0x00752CFF),
        "sky_blue" => Ok(0x66BFFFFF),
        "blue" => Ok(0x0079F1FF),
        "dark_blue" => Ok(0x0052ACFF),
        "purple" => Ok(0xC87AFFFF),
        "violet" => Ok(0x873CBEFF),
        "dark_purple" => Ok(0x701F7EFF),
        "beige" => Ok(0xD3B083FF),
        "brown" => Ok(0x7F6A4DFF),
        "dark_brown" => Ok(0x4C3F2FFF),
        "white" => Ok(0xFFFFFFFF),
        "black" => Ok(0x000000FF),
        "blank" => Ok(0x00000000),
        "magenta" => Ok(0xFF00FFFF),
        "ray_white" => Ok(0xF5F5F5FF),

        _ => Err(format!(
            "clearBackground(): unknown color named '{}'",
            color
        )),
    }
}

fn resolve_key(key: &str) -> Result<i32, String> {
    // hasil generate AI biar mempercepat awoawokawok
    match key {
        "null" => Ok(0),

        // Alphanumeric
        "apostrophe" => Ok(39),
        "comma" => Ok(44),
        "minus" => Ok(45),
        "period" => Ok(46),
        "slash" => Ok(47),
        "0" => Ok(48),
        "1" => Ok(49),
        "2" => Ok(50),
        "3" => Ok(51),
        "4" => Ok(52),
        "5" => Ok(53),
        "6" => Ok(54),
        "7" => Ok(55),
        "8" => Ok(56),
        "9" => Ok(57),
        "semicolon" => Ok(59),
        "equal" => Ok(61),
        "a" => Ok(65),
        "b" => Ok(66),
        "c" => Ok(67),
        "d" => Ok(68),
        "e" => Ok(69),
        "f" => Ok(70),
        "g" => Ok(71),
        "h" => Ok(72),
        "i" => Ok(73),
        "j" => Ok(74),
        "k" => Ok(75),
        "l" => Ok(76),
        "m" => Ok(77),
        "n" => Ok(78),
        "o" => Ok(79),
        "p" => Ok(80),
        "q" => Ok(81),
        "r" => Ok(82),
        "s" => Ok(83),
        "t" => Ok(84),
        "u" => Ok(85),
        "v" => Ok(86),
        "w" => Ok(87),
        "x" => Ok(88),
        "y" => Ok(89),
        "z" => Ok(90),
        "left_bracket" => Ok(91),
        "backslash" => Ok(92),
        "right_bracket" => Ok(93),
        "grave" => Ok(96),

        // Function keys
        "space" => Ok(32),
        "escape" => Ok(256),
        "enter" => Ok(257),
        "tab" => Ok(258),
        "backspace" => Ok(259),
        "insert" => Ok(260),
        "delete" => Ok(261),
        "right" => Ok(262),
        "left" => Ok(263),
        "down" => Ok(264),
        "up" => Ok(265),
        "page_up" => Ok(266),
        "page_down" => Ok(267),
        "home" => Ok(268),
        "end" => Ok(269),
        "caps_lock" => Ok(280),
        "scroll_lock" => Ok(281),
        "num_lock" => Ok(282),
        "print_screen" => Ok(283),
        "pause" => Ok(284),
        "f1" => Ok(290),
        "f2" => Ok(291),
        "f3" => Ok(292),
        "f4" => Ok(293),
        "f5" => Ok(294),
        "f6" => Ok(295),
        "f7" => Ok(296),
        "f8" => Ok(297),
        "f9" => Ok(298),
        "f10" => Ok(299),
        "f11" => Ok(300),
        "f12" => Ok(301),
        "left_shift" => Ok(340),
        "left_control" => Ok(341),
        "left_alt" => Ok(342),
        "left_super" => Ok(343),
        "right_shift" => Ok(344),
        "right_control" => Ok(345),
        "right_alt" => Ok(346),
        "right_super" => Ok(347),
        "kb_menu" => Ok(348),

        // Keypad
        "kp_0" => Ok(320),
        "kp_1" => Ok(321),
        "kp_2" => Ok(322),
        "kp_3" => Ok(323),
        "kp_4" => Ok(324),
        "kp_5" => Ok(325),
        "kp_6" => Ok(326),
        "kp_7" => Ok(327),
        "kp_8" => Ok(328),
        "kp_9" => Ok(329),
        "kp_decimal" => Ok(330),
        "kp_divide" => Ok(331),
        "kp_multiply" => Ok(332),
        "kp_subtract" => Ok(333),
        "kp_add" => Ok(334),
        "kp_enter" => Ok(335),
        "kp_equal" => Ok(336),

        // Android
        "back" => Ok(4),
        "menu" => Ok(5),
        "volume_up" => Ok(24),
        "volume_down" => Ok(25),

        _ => Err(format!("isKeyDown(): unknown key named '{}'", key)),
    }
}

pub fn raylib_clear_background(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "clearBackground".to_string(),
        1,
        Rc::new(move |args| -> Result<Value, String> {
            let color = expect_string(&args[0], "clearBackground", 0)?;
            let c = resolve_color(color.as_str())?;

            unsafe { (raylib.clear_background)(c) };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_draw_rectangle(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "drawRectangle".to_string(),
        5,
        Rc::new(move |args| -> Result<Value, String> {
            let pos_x = expect_number(&args[0], "drawRectangle", 0)?;
            let pos_y = expect_number(&args[1], "drawRectangle", 1)?;
            let width = expect_number(&args[2], "drawRectangle", 2)?;
            let height = expect_number(&args[3], "drawRectangle", 3)?;
            let color = expect_string(&args[4], "drawRectangle", 4)?;
            let c = resolve_color(color.as_str())?;

            unsafe {
                (raylib.draw_rectangle)(pos_x as i32, pos_y as i32, width as i32, height as i32, c)
            };
            Ok(Value::Null)
        }),
    )
}

pub fn raylib_is_key_down(raylib: Rc<Raylib>) -> Value {
    Value::native_fun(
        "isKeyDown".to_string(),
        1,
        Rc::new(move |args| -> Result<Value, String> {
            let key = expect_string(&args[0], "iskeyDown", 0)?;
            let k = resolve_key(key.as_str())?;

            let val = unsafe { (raylib.is_key_down)(k) };
            Ok(Value::Boolean(val))
        }),
    )
}

pub fn module_raylib() -> IndexMap<String, Value> {
    let ray = Rc::new(
        Raylib::new("./raylib/lib/raylib.dll".to_string()).expect("failed to load raylib.dll")
    );

    let mut map = IndexMap::new();
    map.insert("initWindow".to_string(),        raylib_init_window(ray.clone()));
    map.insert("windowShouldClose".to_string(), raylib_windows_should_close(ray.clone()));
    map.insert("beginDrawing".to_string(),      raylib_begin_drawing(ray.clone()));
    map.insert("endDrawing".to_string(),        raylib_end_drawing(ray.clone()));
    map.insert("closeWindow".to_string(),       raylib_close_window(ray.clone()));
    map.insert("clearBackground".to_string(),   raylib_clear_background(ray.clone()));
    map.insert("drawRectangle".to_string(),     raylib_draw_rectangle(ray.clone()));
    map.insert("isKeyDown".to_string(),         raylib_is_key_down(ray.clone()));
    map.insert("getFrameTime".to_string(),      raylib_get_frame_time(ray));
    map
}

// -------------------- UNTUK NATIVE BIASA --------------------------

pub fn native_println(args: Vec<Value>) -> Result<Value, String> {
    let _ = native_print(args);
    println!();
    Ok(Value::Null)
}

pub fn native_print(args: Vec<Value>) -> Result<Value, String> {
    for arg in &args {
        print!("{}", arg);
    }
    Ok(Value::Null)
}

pub fn native_readline(args: Vec<Value>) -> Result<Value, String> {
    let mut input = String::new();

    print!("{}", args[0]);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    let trimmed = input.trim_end_matches(['\n', '\r']).to_string();

    Ok(Value::String(trimmed))
}

pub fn native_typeof(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(_) => Ok(Value::String("string".to_string())),
        Value::Array(_) => Ok(Value::String("array".to_string())),
        Value::Map(_) => Ok(Value::String("map".to_string())),
        Value::Number(_) => Ok(Value::String("number".to_string())),
        Value::Function(_) => Ok(Value::String("fun".to_string())),
        Value::Boolean(_) => Ok(Value::String("boolean".to_string())),
        Value::Null => Ok(Value::String("null".to_string())),
        Value::NativeFunction(_) => Ok(Value::String("native fun".to_string())),
        Value::Class(_) => Ok(Value::String("class".to_string())),
        Value::Instance(_) => Ok(Value::String("instance".to_string())),
    }
}

pub fn native_number(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::String(str) => match str.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(e) => Err(format!(
                "number(): failed to parse number from string '{}': {}",
                str,
                e.to_string()
            )),
        },

        Value::Boolean(b) => {
            if *b {
                Ok(Value::Number(1.0))
            } else {
                Ok(Value::Number(0.0))
            }
        }

        Value::Number(num) => Ok(Value::Number(*num)),

        _ => Err(format!(
            "number(): type '{}' cannot be converted to number.",
            args[0]
        )),
    }
}

pub fn native_string(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::String(format!("{}", args[0])))
}

pub fn native_boolean(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Boolean(b) => Ok(Value::Boolean(*b)),
        Value::Null => Ok(Value::Boolean(false)),
        Value::Number(num) => Ok(Value::Boolean(*num != 0.0)),
        Value::String(str) => Ok(Value::Boolean(!str.is_empty())),
        Value::Array(arr) => Ok(Value::Boolean(!arr.borrow().is_empty())),
        Value::Map(map) => Ok(Value::Boolean(!map.borrow().is_empty())),
        Value::Function { .. }
        | Value::NativeFunction { .. }
        | Value::Class(_)
        | Value::Instance(_) => Ok(Value::Boolean(true)),
    }
}

pub fn native_push(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => {
            arr.borrow_mut().push(args[1].clone());
            Ok(Value::Null)
        }

        _ => Err(format!("push(): first argument must be array.")),
    }
}

pub fn native_pop(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => match arr.borrow_mut().pop() {
            Some(val) => Ok(val),
            None => Err(format!("pop(): cannot popping empty array.")),
        },
        _ => Err(format!("pop(): cannot popping non-array type.")),
    }
}

pub fn native_remove(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => match &args[1] {
            Value::Number(num) => {
                if *num >= 0.0 {
                    let i = *num as usize;
                    let len = arr.borrow().len();
                    if i < len {
                        Ok(arr.borrow_mut().remove(i))
                    } else {
                        Err(format!(
                            "remove(): index out of bounds. need index of {}, but only has {} indices.",
                            i, len
                        ))
                    }
                } else {
                    Err("remove(): index cannot be negative number".to_string())
                }
            }
            _ => Err("remove(): second argument must be a number.".to_string()),
        },
        Value::Map(map) => match &args[1] {
            Value::String(key) => match map.borrow_mut().shift_remove(key) {
                Some(val) => Ok(val),
                None => Ok(Value::Null),
            },
            _ => Err(
                "remove(): second argument must be a string key when removing from map."
                    .to_string(),
            ),
        },
        _ => Err("remove(): first argument must be array or map.".to_string()),
    }
}

pub fn native_len(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Array(arr) => Ok(Value::Number(arr.borrow().len() as f64)),
        Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
        Value::Map(map) => Ok(Value::Number(map.borrow().len() as f64)),
        _ => Err("len(): argument must be an array, string, or map.".to_string()),
    }
}

pub fn native_range(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() || args.len() > 3 {
        return Err(format!(
            "range(): expected 1, 2, or 3 arguments, got {}.",
            args.len()
        ));
    }

    let mut num_args = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if let Value::Number(n) = arg {
            num_args.push(*n);
        } else {
            return Err(format!(
                "range(): argument {} must be a number, got '{}'.",
                i + 1,
                arg
            ));
        }
    }

    let (start, stop, step) = match num_args.len() {
        1 => (0.0, num_args[0], 1.0),
        2 => (num_args[0], num_args[1], 1.0),
        3 => (num_args[0], num_args[1], num_args[2]),
        _ => unreachable!(),
    };

    if step == 0.0 {
        return Err("range(): step argument must not be zero.".to_string());
    }

    let mut result = Vec::new();
    let mut current = start;

    if step > 0.0 {
        while current < stop {
            result.push(Value::Number(current));
            current += step;
        }
    } else {
        while current > stop {
            result.push(Value::Number(current));
            current += step;
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(result))))
}

pub fn native_keys(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Map(map) => {
            let keys: Vec<Value> = map
                .borrow()
                .keys()
                .map(|k| Value::String(k.clone()))
                .collect();
            Ok(Value::Array(Rc::new(RefCell::new(keys))))
        }
        _ => Err("keys(): argument must be a map.".to_string()),
    }
}

pub fn native_values(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Map(map) => {
            let values: Vec<Value> = map.borrow().values().cloned().collect();
            Ok(Value::Array(Rc::new(RefCell::new(values))))
        }
        _ => Err("values(): argument must be a map.".to_string()),
    }
}

pub fn native_has_key(args: Vec<Value>) -> Result<Value, String> {
    match (&args[0], &args[1]) {
        (Value::Map(map), Value::String(key)) => Ok(Value::Boolean(map.borrow().contains_key(key))),
        (Value::Map(_), _) => Err("hasKey(): second argument must be a string key.".to_string()),
        _ => Err("hasKey(): first argument must be a map.".to_string()),
    }
}

pub fn native_clear(args: Vec<Value>) -> Result<Value, String> {
    match &args[0] {
        Value::Map(map) => {
            map.borrow_mut().clear();
            Ok(Value::Null)
        }
        Value::Array(arr) => {
            arr.borrow_mut().clear();
            Ok(Value::Null)
        }
        _ => Err("clear(): argument must be a map or an array.".to_string()),
    }
}
