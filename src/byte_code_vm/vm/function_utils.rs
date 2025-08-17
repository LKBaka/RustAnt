use crate::{
    byte_code_vm::{constants::UNINIT_OBJ, vm::{frame::Frame, vm::Vm}},
    object::{ant_closure::Closure, ant_compiled_function::CompiledFunction, object::Object},
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

pub fn push_closure(
    vm: &mut Vm, 
    const_index: u16,
    free_count: u16,
) -> Result<(), String> {
    let constant = &vm.constants[const_index as usize];

    let func = if let Some(it) = (constant.as_any()).downcast_ref::<CompiledFunction>() {
        it
    } else {
        return Err(format!("not a function: {:?}", constant));
    };

    let free_count_usize = free_count as usize;

    let uninit_obj: Object = Box::new(UNINIT_OBJ.clone());
    let mut free = vec![uninit_obj; free_count_usize];

    for i in 0..free_count as usize {
        free[i] = vm.stack[vm.sp - free_count_usize + i].borrow().clone();
    }

    vm.sp = vm.sp - free_count_usize;

    let closure = Closure {
        func: rc_ref_cell!(func.clone()),
        free: rc_ref_cell!(free),
    };

    vm.push(rc_ref_cell!(Box::new(closure)))
}
