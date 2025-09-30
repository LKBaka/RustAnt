use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::{code::code::{OP_RETURN, OP_RETURN_VALUE}, constants::NONE_OBJ, vm::{
        function_utils::{call_closure, call_method, call_native},
        vm::Vm,
    }},
    obj_enum::object::Object,
    object::object::IAntObject, rc_ref_cell,
};

fn next(
    vm: &mut Vm
) -> Result<(), String> {
    let ip;

    let instructions;

    let op;

    while {
        instructions = vm.current_frame().instructions();

        let current_frame = vm.current_frame();

        current_frame.ip < instructions.len() as isize - 1
    } {
        vm.current_frame().ip += 1;

        ip = vm.current_frame().ip as usize;

        op = instructions[ip];

        match op {
            OP_RETURN_VALUE => {
                let return_value = vm.pop();

                let frame = vm.pop_frame(); // 弹出当前帧

                vm.sp = frame.base_pointer - 1;

                if let Some(value) = return_value {
                    if let Err(msg) = vm.push(value) {
                        return Err(format!("error push return value: {msg}"));
                    }
                }

                return Ok(());
            }

            OP_RETURN => {
                let return_value = NONE_OBJ.clone();

                if vm.frame_index == 1 {
                    // 没栈帧可榨了 说明已经到了主栈帧 直接报错
                    return Err(format!("cannot return outside function"));
                }

                let frame = vm.pop_frame(); // 弹出当前帧

                vm.sp = frame.base_pointer - 1;

                if let Err(msg) = vm.push(rc_ref_cell!(return_value)) {
                    return Err(format!("error push return value: {msg}"));
                }

                return Ok(());
            }

            _ => {
                return vm.next(op, ip, instructions);
            },
        }
    }

    Ok(())
}

#[inline]
fn native_to_call_closure(
    vm: &mut Vm,
    cl: Rc<RefCell<Object>>,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<(), String> {
    for arg in args.iter() {
        vm.push(arg.clone())?;
    }

    call_closure(vm, cl, args.len())?;

    next(vm)
}

#[inline]
fn native_to_call_method(
    vm: &mut Vm,
    method: Rc<RefCell<Object>>,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<(), String> {
    for arg in args.iter() {
        vm.push(arg.clone())?;
    }

    call_method(vm, method, args.len())?;

    next(vm)
}

/*
该函数不返回原生函数的值
该函数会自动把返回值 push 到栈上
*/
#[inline]
fn native_to_call_native(
    vm: &mut Vm,
    native_func: Rc<RefCell<Object>>,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<(), String> {
    call_native(vm, native_func, args.len())
}

#[inline]
pub fn native_to_call(
    vm: &mut Vm,
    func: Rc<RefCell<Object>>,
    args: Vec<Rc<RefCell<Object>>>,
) -> Result<(), String> {
    #[repr(u8)]
    enum CallType {
        Closure,
        Native,
        Method,
    }

    let ty = match &*func.borrow() {
        Object::Closure(_cl) => CallType::Closure,
        Object::AntNativeFunction(_native) => CallType::Native,
        Object::Method(_method) => CallType::Method,
        _ => return Err(format!("calling non-function: {}", func.borrow().inspect())),
    };

    match ty {
        CallType::Closure => native_to_call_closure(vm, func, args),
        CallType::Method => native_to_call_method(vm, func, args),
        CallType::Native => native_to_call_native(vm, func, args),
    }
}
