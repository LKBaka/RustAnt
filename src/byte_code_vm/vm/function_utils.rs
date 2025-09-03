use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::{
        constants::{NONE_OBJ, UNINIT_OBJ},
        vm::{frame::Frame, vm::Vm},
    }, object::{ant_closure::Closure, ant_compiled_function::CompiledFunction, ant_native_function::AntNativeFunction, object::{Object, CLOSURE, NATIVE_FUNCTION}}, rc_ref_cell
};

pub fn call(vm: &mut Vm, arg_count: usize) -> Result<(), String> {
    let calling_obj = &vm.stack[vm.sp - 1 - arg_count];

    if calling_obj.borrow().get_type() == CLOSURE {
        call_function(vm, calling_obj.clone(), arg_count)
    } else if calling_obj.borrow().get_type() == NATIVE_FUNCTION {
        call_native(vm, calling_obj.clone(), arg_count)
    } else {
        return Err(format!("calling non-function. obj: {:?}", calling_obj.borrow()))
    }
}

pub fn call_native(vm: &mut Vm, obj: Rc<RefCell<Object>>, arg_count: usize) -> Result<(), String> {
    let calling_obj = if let Some(it) = obj
        .borrow()
        .as_any()
        .downcast_ref::<AntNativeFunction>()
    {
        it.clone()
    } else {
        return Err(format!("calling non-native-function"));
    };

    let args = &vm.stack[vm.sp - arg_count..vm.sp];
    let result = (calling_obj.function)(args.to_vec());

    // 调整栈指针以移除 函数对象 + 参数
    let base_pointer_of_function = vm.sp - arg_count - 1;
    vm.sp = base_pointer_of_function;

    // 将返回值放到栈顶（作为函数调用表达式的值）
    if let Some(it) = result {
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

pub fn call_function(vm: &mut Vm, obj: Rc<RefCell<Object>>, arg_count: usize) -> Result<(), String> {
    let calling_obj = if let Some(it) = obj
        .borrow()
        .as_any()
        .downcast_ref::<Closure>()
    {
        it.clone()
    } else {
        return Err(format!("calling non-function"));
    };

    let func = calling_obj.func.clone();
    let func_borrow = func.borrow();

    if arg_count != func_borrow.param_count {
        return Err(format!(
            "expected {arg_count} args, got {} args",
            func_borrow.param_count
        ));
    }

    let local_count = func_borrow.local_count;

    let frame = Frame::new(rc_ref_cell!(calling_obj), vm.sp - arg_count);

    let frame_base_pointer = frame.base_pointer;

    vm.push_frame(rc_ref_cell!(frame));

    vm.sp = frame_base_pointer + local_count;

    Ok(())
}

pub fn push_closure(vm: &mut Vm, const_index: u16, free_count: u16) -> Result<(), String> {
    let constant = &vm.constants[const_index as usize];

    let func = if let Some(it) = constant
        .as_any()
        .downcast_ref::<CompiledFunction>() 
    {
        it
    } else {
        return Err(format!("not a function: {:?}", constant));
    };

    let free_count_usize = free_count as usize;

    let uninit_obj: Object = Box::new(UNINIT_OBJ.clone());
    let mut free = vec![uninit_obj; free_count_usize];

    for i in 0..free_count_usize {
        free[i] = vm.stack[vm.sp - free_count_usize + i].borrow().clone();
    }

    vm.sp -= free_count_usize;

    let closure = Closure {
        func: rc_ref_cell!(func.clone()),
        free: rc_ref_cell!(free),
    };

    vm.push(rc_ref_cell!(Box::new(closure)))
}
