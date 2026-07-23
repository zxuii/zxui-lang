use std::any::Any;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::{cell::RefCell, fs};

use indexmap::IndexMap;

use crate::object::{
    ClassData, InstanceData, MethodKind, NativeData, NativeMethodData,
    Value::{self, NativeFunction},
};

struct FsState {
    handle: RefCell<File>,
    path: Rc<str>,
}

pub fn make_fs_module(root_dir: Rc<str>) -> IndexMap<String, Value> {
    let mut map = IndexMap::new();
    map.insert("open".to_string(), fs_open(root_dir));

    map
}

fn get_state(inst: &InstanceData) -> Result<Rc<FsState>, String> {
    inst.native
        .borrow()
        .as_ref()
        .ok_or_else(|| "this instance has no native fs state".to_string())?
        .clone()
        .downcast::<FsState>()
        .map_err(|_| "instance native state is not an FsState".to_string())
}

fn build_fs_class() -> Rc<ClassData> {
    let mut methods = IndexMap::new();

    methods.insert(
        "read".to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: "read".to_string(),
            arity: 0,
            fun: Box::new(move |self_val, _| -> Result<Value, String> {
                let inst = match &self_val {
                    Value::Instance(i) => i.clone(),
                    _ => return Err("read(): called on non-instance".to_string()),
                };
                let state = get_state(&inst)?;

                let mut contents = String::new();
                match state.handle.borrow_mut().read_to_string(&mut contents) {
                    Ok(_) => {}
                    Err(e) => return Err(format!("FS.read(): failed to read: '{}'", e))
                }
                Ok(Value::String(contents))
            }),
        })),
    );

    methods.insert(
        "path".to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: "path".to_string(),
            arity: 0,
            fun: Box::new(move |self_val, _| -> Result<Value, String> {
                let inst = match &self_val {
                    Value::Instance(i) => i.clone(),
                    _ => return Err("path(): called on non-instance".to_string()),
                };
                let state = get_state(&inst)?;

                Ok(Value::String(state.path.to_string()))
            }),
        })),
    );
    
    Rc::new(ClassData::new(
        "FS".to_string(),
        methods,
        IndexMap::new(),
        None,
    ))
}

fn fs_open(root_dir: Rc<str>) -> Value {
    NativeFunction(NativeData::new(
        "open".to_string(),
        1,
        Rc::new(move |args| -> Result<Value, String> {
            let arg_path = match &args[0] {
                Value::String(s) => s.clone(),
                other => {
                    return Err(format!(
                        "fs.open(): argument must be a string, got '{}'",
                        other
                    ));
                }
            };
            let path = std::path::Path::new(root_dir.as_ref())
                .join(&arg_path)
                .to_string_lossy()
                .to_string();

            let handle = match fs::File::open(&path) {
                Ok(f) => f,
                Err(e) => return Err(format!("fs.load(): failed to open file '{}': {}", path, e)),
            };

            let state = Rc::new(FsState { handle: handle.into(), path: path.into() });

            let class = build_fs_class();
            let instance = InstanceData {
                class,
                fields: RefCell::new(IndexMap::new()),
                native: RefCell::new(Some(state as Rc<dyn Any>)),
            };

            Ok(Value::Instance(Rc::new(instance)))
        }),
    ))
}
