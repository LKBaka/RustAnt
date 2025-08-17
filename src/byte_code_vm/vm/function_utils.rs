use crate::{
    byte_code_vm::vm::{frame::Frame, vm::Vm},
    object::{ant_closure::Closure, ant_compiled_function::CompiledFunction},
    rc_ref_cell,
};

pub fn call_function(vm: &mut Vm, arg_count: usize) -> Result<(), String> {
    let calling_obj = if let Some(it) = vm.stack[vm.sp - 1 - arg_count]
        .borrow()
        .as_any()
        .downcast_ref::<Closure>()
    {
        it.clone()
    } else {
        return Err(format!("calling non-function"));
    };

    if arg_count != calling_obj.func.borrow().param_count {
        return Err(format!(
            "expected {arg_count} args, got {} args",
            calling_obj.func.borrow().param_count
        ));
    }

    let frame = Frame::new(rc_ref_cell!(calling_obj.clone()), vm.sp - arg_count);
    vm.push_frame(rc_ref_cell!(frame.clone()));

    vm.sp = frame.base_pointer + calling_obj.func.borrow().local_count;

    Ok(())
}

pub fn push_closure(vm: &mut Vm, const_index: u16) -> Result<(), String> {
    let constant = &vm.constants[const_index as usize];

    let func = if let Some(it) = (constant.as_any()).downcast_ref::<CompiledFunction>() {
        it
    } else {
        return Err(format!("not a function: {:?}", constant));
    };

    let closure = Closure {
        func: rc_ref_cell!(func.clone()),
        free: rc_ref_cell!(vec![]),
    };

    vm.push(rc_ref_cell!(Box::new(closure)))
}
