use crate::object::{ClassData, InstanceData, MethodKind, NativeMethodData, Value};

use indexmap::IndexMap;
use libffi::middle::{Arg, Cif, CodePtr, Ret, Type};
use libloading::Library;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::rc::Rc;

const STRUCT_TAG_KEY: &str = "__struct_name__";

#[derive(Clone, Debug)]
pub enum CType {
    Void,
    I64,
    U64,
    I32,
    U32,
    I16,
    U16,
    I8,
    U8,
    F32,
    F64,
    Bool,
    Str,
    Ptr,
    Struct(String),
}

fn parse_ctype(s: &str) -> Result<CType, String> {
    let t = s.trim();
    Ok(match t {
        "void" => CType::Void,
        "i64" => CType::I64,
        "u64" => CType::U64,
        "i32" => CType::I32,
        "u32" => CType::U32,
        "i16" => CType::I16,
        "u16" => CType::U16,
        "i8" => CType::I8,
        "u8" => CType::U8,
        "f32" => CType::F32,
        "f64" => CType::F64,
        "bool" => CType::Bool,
        "str" => CType::Str,
        "ptr" => CType::Ptr,
        other => CType::Struct(other.to_string()),
    })
}

fn ctype_to_string(ct: &CType) -> String {
    match ct {
        CType::Void => "void".to_string(),
        CType::I64 => "i64".to_string(),
        CType::U64 => "u64".to_string(),
        CType::I32 => "i32".to_string(),
        CType::U32 => "u32".to_string(),
        CType::I16 => "i16".to_string(),
        CType::U16 => "u16".to_string(),
        CType::I8 => "i8".to_string(),
        CType::U8 => "u8".to_string(),
        CType::F32 => "f32".to_string(),
        CType::F64 => "f64".to_string(),
        CType::Bool => "bool".to_string(),
        CType::Str => "str".to_string(),
        CType::Ptr => "ptr".to_string(),
        CType::Struct(name) => name.clone(),
    }
}

fn ctype_size(ct: &CType, structs: &IndexMap<String, StructLayout>) -> Result<usize, String> {
    Ok(match ct {
        CType::Void => 0,
        CType::I32 | CType::U32 => 4,
        CType::I8 | CType::U8 | CType::Bool => 1,
        CType::I16 | CType::U16 => 2,
        CType::I64 | CType::U64 => 8,
        CType::F32 => 4,
        CType::F64 => 8,
        CType::Str | CType::Ptr => mem::size_of::<usize>(),
        CType::Struct(n) => {
            structs
                .get(n)
                .ok_or_else(|| format!("unknown struct type '{}'", n))?
                .size
        }
    })
}

fn ctype_align(ct: &CType, structs: &IndexMap<String, StructLayout>) -> Result<usize, String> {
    Ok(match ct {
        CType::Struct(n) => {
            structs
                .get(n)
                .ok_or_else(|| format!("unknown struct type '{}'", n))?
                .align
        }
        other => ctype_size(other, structs)?.max(1),
    })
}

fn ctype_to_middle(ct: &CType, structs: &IndexMap<String, StructLayout>) -> Result<Type, String> {
    Ok(match ct {
        CType::Void => Type::void(),
        CType::I32 => Type::i32(),
        CType::U32 => Type::u32(),
        CType::I8 => Type::i8(),
        CType::U8 | CType::Bool => Type::u8(),
        CType::I16 => Type::i16(),
        CType::U16 => Type::u16(),
        CType::I64 => Type::i64(),
        CType::U64 => Type::u64(),
        CType::F32 => Type::f32(),
        CType::F64 => Type::f64(),
        CType::Str | CType::Ptr => Type::pointer(),
        CType::Struct(n) => {
            let layout = structs
                .get(n)
                .ok_or_else(|| format!("unknown struct type '{}'", n))?
                .clone();
            let field_types: Vec<Type> = layout
                .fields
                .iter()
                .map(|f| ctype_to_middle(&f.ty, structs))
                .collect::<Result<_, _>>()?;
            Type::structure(field_types)
        }
    })
}

#[derive(Clone)]
pub struct StructField {
    pub name: String,
    pub ty: CType,
    pub offset: usize,
}

#[derive(Clone)]
pub struct StructLayout {
    pub name: String,
    pub fields: Vec<StructField>,
    pub size: usize,
    pub align: usize,
}

fn build_struct_layout(
    name: &str,
    field_defs: &IndexMap<String, Value>,
    structs: &IndexMap<String, StructLayout>,
) -> Result<StructLayout, String> {
    let mut fields = Vec::new();
    let mut offset = 0usize;
    let mut max_align = 1usize;

    for (fname, fty_val) in field_defs.iter() {
        let ty_str = match fty_val {
            Value::String(s) => s.clone(),
            other => {
                return Err(format!(
                    "struct field '{}' type must be a string, got '{}'",
                    fname, other
                ));
            }
        };
        let ty = parse_ctype(&ty_str)?;
        let align = ctype_align(&ty, structs)?;
        let size = ctype_size(&ty, structs)?;

        if align > 0 && offset % align != 0 {
            offset += align - (offset % align);
        }
        max_align = max_align.max(align);

        fields.push(StructField {
            name: fname.clone(),
            ty,
            offset,
        });
        offset += size;
    }

    if max_align > 0 && offset % max_align != 0 {
        offset += max_align - (offset % max_align);
    }

    Ok(StructLayout {
        name: name.to_string(),
        fields,
        size: offset.max(1),
        align: max_align,
    })
}

pub struct FfiFunction {
    pub name: String,
    pub arg_types: Vec<CType>,
    pub ret_type: CType,
    pub symbol: *const c_void,
}

pub struct FfiState {
    pub _lib: Library,
    pub structs: RefCell<IndexMap<String, StructLayout>>,
    pub functions: RefCell<HashMap<String, FfiFunction>>,
}

fn expect_num(v: &Value) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        other => Err(format!("expected number, got '{}'", other)),
    }
}

fn expect_string(v: &Value) -> Result<String, String> {
    match v {
        Value::String(s) => Ok(s.clone()),
        other => Err(format!("expected string, got '{}'", other)),
    }
}

fn expect_bool(v: &Value) -> Result<bool, String> {
    match v {
        Value::Boolean(b) => Ok(*b),
        other => Err(format!("expected boolean, got '{}'", other)),
    }
}

fn write_value_to_buffer(
    val: &Value,
    ty: &CType,
    buf: &mut [u8],
    structs: &IndexMap<String, StructLayout>,
    keep_alive: &mut Vec<CString>,
) -> Result<(), String> {
    match ty {
        CType::Void => {}
        CType::I32 => buf[..4].copy_from_slice(&(expect_num(val)? as i32).to_ne_bytes()),
        CType::U32 => buf[..4].copy_from_slice(&(expect_num(val)? as u32).to_ne_bytes()),
        CType::I8 => buf[0] = (expect_num(val)? as i8) as u8,
        CType::Bool => buf[0] = expect_bool(val)? as u8,
        CType::U8 => buf[0] = expect_num(val)? as u8,
        CType::I16 => buf[..2].copy_from_slice(&(expect_num(val)? as i16).to_ne_bytes()),
        CType::U16 => buf[..2].copy_from_slice(&(expect_num(val)? as u16).to_ne_bytes()),
        CType::I64 => buf[..8].copy_from_slice(&(expect_num(val)? as i64).to_ne_bytes()),
        CType::U64 => buf[..8].copy_from_slice(&(expect_num(val)? as u64).to_ne_bytes()),
        CType::F32 => buf[..4].copy_from_slice(&(expect_num(val)? as f32).to_ne_bytes()),
        CType::F64 => buf[..8].copy_from_slice(&(expect_num(val)?).to_ne_bytes()),
        CType::Str => {
            let s = expect_string(val)?;
            let c = CString::new(s).map_err(|e| e.to_string())?;
            let ptr = c.as_ptr() as usize;
            keep_alive.push(c);
            buf[..std::mem::size_of::<usize>()].copy_from_slice(&ptr.to_ne_bytes());
        }
        CType::Ptr => {
            let ptr = match val {
                Value::Null => 0usize,
                other => expect_num(other)? as usize,
            };
            buf[..std::mem::size_of::<usize>()].copy_from_slice(&ptr.to_ne_bytes());
        }
        CType::Struct(name) => {
            let layout = structs
                .get(name)
                .ok_or_else(|| format!("unknown struct type '{}'", name))?
                .clone();

            let map = match val {
                Value::Map(m) => m.borrow().clone(),
                other => {
                    return Err(format!(
                        "expected map for struct '{}', got '{}'",
                        name, other
                    ));
                }
            };

            if let Some(Value::String(tag)) = map.get(STRUCT_TAG_KEY) {
                if tag != name {
                    return Err(format!("expected struct '{}', got struct '{}'", name, tag));
                }
            }

            for field in &layout.fields {
                let fval = map
                    .get(&field.name)
                    .cloned()
                    .ok_or_else(|| format!("struct '{}' missing field '{}'", name, field.name))?;
                let fsize = ctype_size(&field.ty, structs)?;
                write_value_to_buffer(
                    &fval,
                    &field.ty,
                    &mut buf[field.offset..field.offset + fsize],
                    structs,
                    keep_alive,
                )?;
            }
        }
    }
    Ok(())
}

fn read_value_from_buffer(
    ty: &CType,
    buf: &[u8],
    structs: &IndexMap<String, StructLayout>,
) -> Result<Value, String> {
    Ok(match ty {
        CType::Void => Value::Null,
        CType::I32 => Value::Number(i32::from_ne_bytes(buf[..4].try_into().unwrap()) as f64),
        CType::U32 => Value::Number(u32::from_ne_bytes(buf[..4].try_into().unwrap()) as f64),
        CType::I8 => Value::Number(buf[0] as i8 as f64),
        CType::U8 => Value::Number(buf[0] as f64),
        CType::Bool => Value::Boolean(buf[0] != 0),
        CType::I16 => Value::Number(i16::from_ne_bytes(buf[..2].try_into().unwrap()) as f64),
        CType::U16 => Value::Number(u16::from_ne_bytes(buf[..2].try_into().unwrap()) as f64),
        CType::I64 => Value::Number(i64::from_ne_bytes(buf[..8].try_into().unwrap()) as f64),
        CType::U64 => Value::Number(u64::from_ne_bytes(buf[..8].try_into().unwrap()) as f64),
        CType::F32 => Value::Number(f32::from_ne_bytes(buf[..4].try_into().unwrap()) as f64),
        CType::F64 => Value::Number(f64::from_ne_bytes(buf[..8].try_into().unwrap())),
        CType::Str => {
            let ptr = usize::from_ne_bytes(buf[..std::mem::size_of::<usize>()].try_into().unwrap());
            if ptr == 0 {
                Value::Null
            } else {
                let cstr = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) };
                Value::String(cstr.to_string_lossy().to_string())
            }
        }
        CType::Ptr => {
            let ptr = usize::from_ne_bytes(buf[..std::mem::size_of::<usize>()].try_into().unwrap());
            Value::Number(ptr as f64)
        }
        CType::Struct(name) => {
            let layout = structs
                .get(name)
                .ok_or_else(|| format!("unknown struct type '{}'", name))?;
            let mut map = IndexMap::new();
            map.insert(STRUCT_TAG_KEY.to_string(), Value::String(name.clone()));
            for field in &layout.fields {
                let fsize = ctype_size(&field.ty, structs)?;
                let fval = read_value_from_buffer(
                    &field.ty,
                    &buf[field.offset..field.offset + fsize],
                    structs,
                )?;
                map.insert(field.name.clone(), fval);
            }
            Value::Map(Rc::new(RefCell::new(map)))
        }
    })
}

fn call_ffi_function(
    state: &Rc<FfiState>,
    name: &str,
    call_args: Vec<Value>,
) -> Result<Value, String> {
    let functions = state.functions.borrow();
    let f = functions.get(name).ok_or_else(|| {
        format!(
            "ffi function '{}' is not declared, call lib.fun() first",
            name
        )
    })?;

    if call_args.len() != f.arg_types.len() {
        return Err(format!(
            "{}(): expects {} args but got {}",
            name,
            f.arg_types.len(),
            call_args.len()
        ));
    }

    let structs = state.structs.borrow();

    let mut keep_alive: Vec<CString> = Vec::new();
    let mut arg_buffers: Vec<Vec<u8>> = Vec::new();
    for (val, ty) in call_args.iter().zip(f.arg_types.iter()) {
        let size = ctype_size(ty, &structs)?;
        let mut buf = vec![0u8; size.max(1)];
        write_value_to_buffer(val, ty, &mut buf, &structs, &mut keep_alive)?;
        arg_buffers.push(buf);
    }

    let arg_middle_types: Vec<Type> = f
        .arg_types
        .iter()
        .map(|ty| ctype_to_middle(ty, &structs))
        .collect::<Result<_, _>>()?;
    let ret_middle_type = ctype_to_middle(&f.ret_type, &structs)?;

    let cif = Cif::new(arg_middle_types.into_iter(), ret_middle_type);

    let arg_refs: Vec<Arg> = arg_buffers.iter().map(|b| Arg::new(b.as_slice())).collect();

    let ret_size = ctype_size(&f.ret_type, &structs)?.max(1);
    let mut ret_buf = vec![0u8; ret_size];

    let code_ptr = CodePtr::from_ptr(f.symbol);

    unsafe {
        if matches!(f.ret_type, CType::Void) {
            cif.call_return_into(code_ptr, &arg_refs, Ret::void());
        } else {
            cif.call_return_into(code_ptr, &arg_refs, Ret::new(ret_buf.as_mut_slice()));
        }
    }

    read_value_from_buffer(&f.ret_type, &ret_buf, &structs)
}

pub fn make_ffi_module(root_dir: Rc<str>) -> IndexMap<String, Value> {
    let mut map = IndexMap::new();
    map.insert("load".to_string(), ffi_load(root_dir));
    map
}

fn ffi_load(root_dir: Rc<str>) -> Value {
    Value::native_fun(
        "load".to_string(),
        1,
        Rc::new(move |args| -> Result<Value, String> {
            let path_arg = match &args[0] {
                Value::String(s) => s.clone(),
                other => {
                    return Err(format!(
                        "ffi.load(): argument must be a string, got '{}'",
                        other
                    ))
                }
            };

            let full_path = std::path::Path::new(root_dir.as_ref()).join(&path_arg);
            let full_path_str = full_path.to_string_lossy().to_string();

            let lib = unsafe {
                Library::new(&full_path).map_err(|e| {
                    format!(
                        "ffi.load(): failed to load library at '{}' (resolved from '{}'): {}",
                        full_path_str, path_arg, e
                    )
                })?
            };

            let state = Rc::new(FfiState {
                _lib: lib,
                structs: RefCell::new(IndexMap::new()),
                functions: RefCell::new(HashMap::new()),
            });

            let class = build_ffi_class();
            let instance = Rc::new(InstanceData {
                class,
                fields: RefCell::new(IndexMap::new()),
                native: RefCell::new(Some(state as Rc<dyn Any>)),
            });

            Ok(Value::Instance(instance))
        }),
    )
}

fn get_state(inst: &InstanceData) -> Result<Rc<FfiState>, String> {
    inst.native
        .borrow()
        .as_ref()
        .ok_or_else(|| "this instance has no native ffi state".to_string())?
        .clone()
        .downcast::<FfiState>()
        .map_err(|_| "instance native state is not an FfiState".to_string())
}

fn build_ffi_class() -> Rc<ClassData> {
    let mut methods = IndexMap::new();

    methods.insert(
        "struct".to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: "struct".to_string(),
            arity: 2,
            fun: Box::new(|self_val, args| {
                let inst = match &self_val {
                    Value::Instance(i) => i.clone(),
                    _ => return Err("struct(): called on non-instance".to_string()),
                };
                let state = get_state(&inst)?;

                let name = match &args[0] {
                    Value::String(s) => s.clone(),
                    other => {
                        return Err(format!(
                            "struct(): first argument must be a string, got '{}'",
                            other
                        ));
                    }
                };
                let field_defs = match &args[1] {
                    Value::Map(m) => m.borrow().clone(),
                    other => {
                        return Err(format!(
                            "struct(): second argument must be a map, got '{}'",
                            other
                        ));
                    }
                };

                let layout = {
                    let structs = state.structs.borrow();
                    build_struct_layout(&name, &field_defs, &structs)?
                };
                state.structs.borrow_mut().insert(name, layout);

                Ok(Value::Null)
            }),
        })),
    );
    methods.insert(
        "declare".to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: "declare".to_string(),
            arity: 3,
            fun: Box::new(|self_val, args| {
                let inst = match &self_val {
                    Value::Instance(i) => i.clone(),
                    _ => return Err("declare(): called on non-instance".to_string()),
                };
                let state = get_state(&inst)?;

                let name = match &args[0] {
                    Value::String(s) => s.clone(),
                    other => {
                        return Err(format!(
                            "declare(): first argument must be a string, got '{}'",
                            other
                        ));
                    }
                };
                let arg_type_vals = match &args[1] {
                    Value::Array(a) => a.borrow().clone(),
                    other => {
                        return Err(format!(
                            "declare(): second argument must be an array, got '{}'",
                            other
                        ));
                    }
                };
                let ret_type_str = match &args[2] {
                    Value::String(s) => s.clone(),
                    other => {
                        return Err(format!(
                            "declare(): third argument must be a string, got '{}'",
                            other
                        ));
                    }
                };

                let mut arg_types = Vec::new();
                for v in &arg_type_vals {
                    let s = match v {
                        Value::String(s) => s.clone(),
                        other => {
                            return Err(format!(
                                "declare(): argument type must be a string, got '{}'",
                                other
                            ));
                        }
                    };
                    arg_types.push(parse_ctype(&s)?);
                }
                let ret_type = parse_ctype(&ret_type_str)?;

                let symbol = unsafe {
                    let sym: libloading::Symbol<*const c_void> = state
                        ._lib
                        .get(format!("{}\0", name).as_bytes())
                        .map_err(|e| format!("declare(): symbol '{}' not found: {}", name, e))?;
                    *sym
                };

                state.functions.borrow_mut().insert(
                    name.clone(),
                    FfiFunction {
                        name: name.clone(),
                        arg_types,
                        ret_type,
                        symbol,
                    },
                );

                Ok(Value::Null)
            }),
        })),
    );

    methods.insert(
        "listFunctions".to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: "listFunctions".to_string(),
            arity: 0,
            fun: Box::new(|self_val, _args| {
                let inst = match &self_val {
                    Value::Instance(i) => i.clone(),
                    _ => return Err("listFunctions(): called on non-instance".to_string()),
                };
                let state = get_state(&inst)?;

                let functions = state.functions.borrow();
                let mut result = Vec::new();
                for f in functions.values() {
                    let mut m = IndexMap::new();
                    m.insert("name".to_string(), Value::String(f.name.clone()));
                    let args: Vec<Value> = f
                        .arg_types
                        .iter()
                        .map(|ty| Value::String(ctype_to_string(ty)))
                        .collect();
                    m.insert(
                        "args".to_string(),
                        Value::Array(Rc::new(RefCell::new(args))),
                    );
                    m.insert(
                        "returns".to_string(),
                        Value::String(ctype_to_string(&f.ret_type)),
                    );
                    result.push(Value::Map(Rc::new(RefCell::new(m))));
                }

                Ok(Value::Array(Rc::new(RefCell::new(result))))
            }),
        })),
    );

    methods.insert(
        "listStructs".to_string(),
        MethodKind::Native(Rc::new(NativeMethodData {
            name: "listStructs".to_string(),
            arity: 0,
            fun: Box::new(|self_val, _args| {
                let inst = match &self_val {
                    Value::Instance(i) => i.clone(),
                    _ => return Err("listStructs(): called on non-instance".to_string()),
                };
                let state = get_state(&inst)?;

                let structs = state.structs.borrow();
                let mut result = Vec::new();
                for layout in structs.values() {
                    let mut m = IndexMap::new();
                    m.insert("name".to_string(), Value::String(layout.name.clone()));

                    let mut fields_map = IndexMap::new();
                    for field in &layout.fields {
                        fields_map.insert(
                            field.name.clone(),
                            Value::String(ctype_to_string(&field.ty)),
                        );
                    }
                    m.insert(
                        "fields".to_string(),
                        Value::Map(Rc::new(RefCell::new(fields_map))),
                    );
                    m.insert("size".to_string(), Value::Number(layout.size as f64));

                    result.push(Value::Map(Rc::new(RefCell::new(m))));
                }

                Ok(Value::Array(Rc::new(RefCell::new(result))))
            }),
        })),
    );

    let native_get: Rc<dyn Fn(&InstanceData, &str) -> Option<Value>> =
        Rc::new(|inst: &InstanceData, key: &str| -> Option<Value> {
            let state = get_state(inst).ok()?;
            if !state.functions.borrow().contains_key(key) {
                return None;
            }
            let state = state.clone();
            let fname = key.to_string();
            Some(Value::native_fun(
                fname.clone(),
                -1,
                Rc::new(move |call_args| call_ffi_function(&state, &fname, call_args)),
            ))
        });

    let mut class = ClassData::new("FFI".to_string(), methods, IndexMap::new(), None);
    class.native_get = Some(native_get);
    Rc::new(class)
}
