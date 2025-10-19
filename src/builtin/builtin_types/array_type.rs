use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    byte_code_vm::vm::vm::Vm,
    obj_enum::object::Object,
    object::{
        ant_method::{Method, MethodType},
        ant_native_function::create_ant_native_function,
        ant_string::AntString,
        object::IAntObject,
    },
};

fn to_string(
    _vm: &mut Vm,
    args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let o = args[0].borrow();

    let me = match &*o {
        Object::AntInt(int) => int,
        _ => return Err(format!("expected an array (self) got: {}", o.inspect())),
    };

    Ok(Some(Object::AntString(AntString::from(
        me.value.to_string(),
    ))))
}

fn push(
    _vm: &mut Vm,
    args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let mut o = args[0].borrow_mut();
    
    // if someone wants to execute 'arr.push(arr)' i will kill him/her ...
    let push = if args[0] == args[1] {
        o.clone()
    } else {
        args[1].borrow().clone()
    };
    
    let me = match &mut *o {
        Object::AntArray(arr) => arr,
        _ => return Err(format!("expected an array (self) got: {}", o.inspect())),
    };
    
    me.items.push(push);

    Ok(None)
}

fn append_all(
    _vm: &mut Vm,
    args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let mut o = args[0].borrow_mut();
    
    // if someone wants to execute 'arr.append_all(arr)' i will kill him/her ...
    let mut append_binding = if args[0] == args[1] {
        /*
        为什么不做任何事情?

        事实上 不做任何事情和以下步骤等价:

        1. 拿走列表所有元素
        2. 把这些元素再追加到列表中

        你可以看到以上步骤完成之后列表没有任何变化, 所以直接返回与它等价
        */
        return Ok(None)
    } else {
        args[1].borrow_mut()
    };

    let append = match &mut *append_binding {
        Object::AntArray(arr) => arr,
        it => return Err(format!("expected an array to append all, got: {}", it.inspect()))
    };
    
    let me = match &mut *o {
        Object::AntArray(arr) => arr,
        _ => return Err(format!("expected an array (self) got: {}", o.inspect())),
    };
    
    me.items.append(&mut append.items);

    Ok(None)
}

fn copy(
    _vm: &mut Vm,
    args: Vec<std::rc::Rc<std::cell::RefCell<Object>>>,
) -> Result<Option<Object>, String> {
    let o = args[0].borrow();
    
    let me = match &*o {
        Object::AntArray(arr) => arr,
        _ => return Err(format!("expected an array (self) got: {}", o.inspect())),
    };
    
    Ok(Some(Object::AntArray(me.clone())))
}

pub static ARRAY_MEMBERS: Lazy<HashMap<String, Object>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        "to_string".to_string(),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(None, to_string)),
        }),
    );

    m.insert(
        "push".to_string(),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(None, push)),
        }),
    );

    m.insert(
        "copy".to_string(),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(None, copy)),
        }),
    );

    m.insert(
        "append_all".to_string(),
        Object::Method(Method {
            me: None,
            func: MethodType::NativeFunction(create_ant_native_function(None, append_all)),
        }),
    );

    m
});
