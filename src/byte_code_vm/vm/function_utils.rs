use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::{
        constants::{NONE_OBJ, UNINIT_OBJECT},
        vm::{frame::Frame, vm::Vm},
    },
    obj_enum::object::Object,
    object::{ant_closure::Closure, ant_method::MethodType},
    rc_ref_cell,
};

pub fn call(vm: &mut Vm, arg_count: usize) -> Result<(), String> {
    let top = vm.sp - 1 - arg_count;
    let obj_tag = {
        let borrow = vm.stack[top].borrow();
        match &*borrow {
            Object::Closure(_) => 0u8,
            Object::AntNativeFunction(_) => 1u8,
            Object::Method(_) => 2u8,
            it => return Err(format!("calling non-function: {it:#?}")),
        }
    };

    match obj_tag {
        0 => call_closure(vm, vm.stack[top].clone(), arg_count),
        1 => call_native(vm, vm.stack[top].clone(), arg_count),
        2 => call_method(vm, vm.stack[top].clone(), arg_count),
        _ => Err(format!("calling non-function")),
    }
}

pub fn call_native(vm: &mut Vm, obj: Rc<RefCell<Object>>, arg_count: usize) -> Result<(), String> {
    let obj_borrow = obj.borrow();

    let calling_obj = if let Object::AntNativeFunction(it) = &*obj_borrow {
        it
    } else {
        return Err(format!("calling non-native-function"));
    };

    let args = &vm.stack[vm.sp - arg_count..vm.sp];
    let result = (calling_obj.function)(vm, args.to_vec());

    // 调整栈指针以移除 函数对象 + 参数
    let base_pointer_of_function = vm.sp - arg_count - 1;
    vm.sp = base_pointer_of_function;

    // 将返回值放到栈顶（作为函数调用表达式的值）
    if let Some(it) = result? {
        if let Err(msg) = vm.push(rc_ref_cell!(it)) {
            return Err(format!("error push native function result: {msg}"));
        }

        return Ok(());
    }

    if let Err(msg) = vm.push(rc_ref_cell!(NONE_OBJ.clone())) {
        return Err(format!("error push none object: {msg}"));
    }

    Ok(())
}

pub fn call_closure(vm: &mut Vm, obj: Rc<RefCell<Object>>, arg_count: usize) -> Result<(), String> {
    let obj_borrow = obj.borrow();

    let calling_obj = if let Object::Closure(it) = &*obj_borrow {
        it.clone()
    } else {
        return Err(format!("calling non-function"));
    };

    let func = calling_obj.func.clone();

    if arg_count != func.param_count {
        return Err(format!(
            "expected {} args, got {arg_count} args",
            func.param_count
        ));
    }

    let local_count = func.local_count;

    let frame = Frame::new(calling_obj, vm.sp - arg_count);

    let frame_base_pointer = frame.base_pointer;

    vm.push_frame(frame);

    vm.sp = frame_base_pointer + local_count;

    Ok(())
}

pub fn call_method(vm: &mut Vm, obj: Rc<RefCell<Object>>, arg_count: usize) -> Result<(), String> {
    let obj_borrow = obj.borrow();

    let calling_obj = if let Object::Method(it) = &*obj_borrow {
        it.clone()
    } else {
        return Err(format!("calling non-method"));
    };

    let mut arg_count = arg_count;

    // 将方法对象替换为 self 对象
    if let Some(o) = calling_obj.me {
        vm.stack.insert(vm.sp - arg_count, o);
        vm.sp += 1;
        arg_count += 1
    }

    match calling_obj.func {
        MethodType::Closure(cl) => call_closure(vm, rc_ref_cell!(Object::Closure(cl)), arg_count),
        MethodType::NativeFunction(f) => {
            call_native(vm, rc_ref_cell!(Object::AntNativeFunction(f)), arg_count)
        }
    }
}

pub fn push_closure(vm: &mut Vm, const_index: u16, free_count: u16) -> Result<(), String> {
    let constant = &vm.constants[const_index as usize];
    let constant_borrow = constant.borrow();

    let func = match &*constant_borrow {
        Object::CompiledFunction(f) => f,
        _ => return Err(format!("not a function: {:?}", constant)),
    };

    let free_count_usize = free_count as usize;

    if free_count_usize == 0 {
        let closure = Closure {
            func: func.clone(),
            free: rc_ref_cell!(vec![]),
        };

        drop(constant_borrow);
        return vm.push(rc_ref_cell!(Object::Closure(closure)));
    }

    let mut free = vec![UNINIT_OBJECT.clone(); free_count_usize];

    for i in 0..free_count_usize {
        free[i] = vm.stack[vm.sp - free_count_usize + i].borrow().clone();
    }

    vm.sp -= free_count_usize;

    let closure = Closure {
        func: func.clone(),
        free: rc_ref_cell!(free),
    };

    drop(constant_borrow);
    vm.push(rc_ref_cell!(Object::Closure(closure)))
}