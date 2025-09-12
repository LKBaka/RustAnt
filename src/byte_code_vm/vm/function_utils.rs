use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::{
        constants::{NONE_OBJ, UNINIT_OBJ},
        vm::{frame::Frame, vm::Vm},
    }, obj_enum::object::Object, object::{ant_closure::Closure, object::{IAntObject, CLOSURE, NATIVE_FUNCTION}}, rc_ref_cell
};

pub fn call(vm: &mut Vm, arg_count: usize) -> Result<(), String> {
    let calling_obj = &vm.stack[vm.sp - 1 - arg_count];

    if calling_obj.borrow().get_type() == CLOSURE {
        call_closure(vm, calling_obj.clone(), arg_count)
    } else if calling_obj.borrow().get_type() == NATIVE_FUNCTION {
        call_native(vm, calling_obj.clone(), arg_count)
    } else {
        return Err(format!("calling non-function. obj: {:?}", calling_obj.borrow()))
    }
}

pub fn call_native(vm: &mut Vm, obj: Rc<RefCell<Object>>, arg_count: usize) -> Result<(), String> {
    let obj_borrow = obj.borrow();

    let calling_obj = 
    if let Object::AntNativeFunction(it) = &*obj_borrow
    {
        it
    } else {
        return Err(format!("calling non-native-function"));
    };

    let args = &vm.stack[vm.sp - arg_count..vm.sp];
    let result = (calling_obj.function)(args.to_vec());

    // 调整栈指针以移除 函数对象 + 参数
    let base_pointer_of_function = vm.sp - arg_count - 1;
    vm.sp = base_pointer_of_function;

    // 将返回值放到栈顶（作为函数调用表达式的值）
    if let Some(it) = result? {
        if let Err(msg) = vm.push(rc_ref_cell!(it)) {
            return Err(format!("error push native function result: {msg}"))
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

pub fn push_closure(vm: &mut Vm, const_index: u16, free_count: u16) -> Result<(), String> {
    let constant = vm.constants[const_index as usize].clone();
    let constant_borrow = constant.borrow();

    let func = match &*constant_borrow {
        Object::CompiledFunction(f) => f,
        _ => return Err(format!("not a function: {:?}", constant))
    };

    let free_count_usize = free_count as usize;

    let uninit_obj: Object = Object::AntUninit(UNINIT_OBJ.clone());
    let mut free = vec![uninit_obj; free_count_usize];

    for i in 0..free_count_usize {
        free[i] = vm.stack[vm.sp - free_count_usize + i].borrow().clone();
    }

    vm.sp -= free_count_usize;

    let closure = Closure {
        func: func.clone(),
        free: rc_ref_cell!(free),
    };

    vm.push(rc_ref_cell!(Object::Closure(closure)))
}
