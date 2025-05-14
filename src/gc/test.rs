#[test]
pub fn test_gc() {
    use num_bigint::BigInt;

    use crate::environment::environment::Environment;
    use crate::environment::data::Data;
    use crate::environment::data_info::DataInfo;
    use crate::object::ant_int::AntInt;
    use crate::object::object::IAntObject;
    use crate::gc::gc::{collect_all, print_stats, set_threshold};

    println!("=== GC Test Started ===");

    // 设置较小的阈值以便于观察
    set_threshold(5);
    
    // 创建环境
    let mut env = Environment::new();
    
    println!("\n1. Creating objects");
    for i in 0..10 {
        let int_obj = AntInt::new_with_native_value(Box::new(BigInt::from(i)));
        env.create(&format!("x{}", i), Data::new(int_obj, DataInfo::new(false)));
    }
    print_stats();
    
    println!("\n2. Removing partial objects");
    for i in 0..5 {
        if let Some(obj) = env.get(&format!("x{}", i)) {
            env.remove_obj(obj);
        }
    }
    print_stats();
    
    println!("\n3. Manual GC trigger");
    collect_all();
    print_stats();
    
    println!("\n4. Trigger auto GC with new objects");
    for i in 10..20 {
        let int_obj = AntInt::new_with_native_value(Box::new(BigInt::from(i)));
        env.create(&format!("x{}", i), Data::new(int_obj, DataInfo::new(false)));
    }
    print_stats();
    
    println!("\n=== GC Test Completed ===");
}

#[test]
pub fn test_recursion_depth() {
    use num_bigint::BigInt;
    use crate::environment::environment::Environment;
    use crate::environment::data::Data;
    use crate::environment::data_info::DataInfo;
    use crate::object::ant_int::AntInt;
    use crate::object::object::IAntObject;
    use crate::gc::gc::{collect_all, print_stats, set_threshold, set_max_recursion_depth};

    println!("=== Recursion Depth Test Started ===");

    set_threshold(5);
    set_max_recursion_depth(10);
    
    let mut env = Environment::new();
    
    println!("\n1. Creating objects");
    for i in 0..20 {
        let int_obj = AntInt::new_with_native_value(Box::new(BigInt::from(i)));
        env.create(&format!("x{}", i), Data::new(int_obj, DataInfo::new(false)));
    }
    print_stats();
    
    println!("\n2. Manual GC trigger");
    collect_all();
    print_stats();
    
    println!("\n=== Recursion Depth Test Completed ===");
}