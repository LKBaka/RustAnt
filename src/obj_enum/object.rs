#![allow(unused_imports)]
use std::any::Any;
use std::hash::Hash;
use std::hash::Hasher;
use enum_dispatch::enum_dispatch;

use crate::object::ant_array::AntArray;
use crate::object::ant_hash_map::AntHashMap;
use crate::object::ant_boolean::AntBoolean;
use crate::object::ant_class::AntClass;
use crate::object::ant_closure::Closure;
use crate::object::ant_compiled_function::CompiledFunction;
use crate::object::ant_double::AntDouble;
use crate::object::ant_error::AntError;
use crate::object::ant_int::AntInt;
use crate::object::ant_method::Method;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::ant_none::AntNone;
use crate::object::ant_string::AntString;
use crate::object::ant_uninit::AntUninit;
use crate::object::object::AsAnyMut;
use crate::object::object::ObjectType;
use crate::object::object::IAntObject;


#[enum_dispatch(IAntObject)]
#[derive(Debug, Clone)]
pub enum Object {
    AntArray,
    AntHashMap,
    AntBoolean,
    AntClass,
    Closure,
    CompiledFunction,
    Method,
    AntDouble,
    AntError,
    AntInt,
    AntNativeFunction,
    AntNone,
    AntString,
    AntUninit,
}

impl AsAnyMut for Object {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::AntInt(ant_int) => {
                // 使用AntInt特定的哈希逻辑
                ant_int.value.hash(state);
            },
            Object::AntString(ant_string) => {
                // 使用AntString特定的哈希逻辑
                ant_string.value.hash(state);
            },
            _ => {
                // 其他类型使用基于ID的哈希
                let mut x = self.get_id() as u64;
                x = x.wrapping_mul(0x9e3779b97f4a7c15);
                x ^= x >> 32;
                x.hash(state);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use colored::Colorize;
    use hashbrown::HashMap;

    use crate::{obj_enum::object::Object, object::{ant_int::AntInt, ant_string::AntString, id_counter::next_id}, utils::assert_eq};

    fn test_hash_map_get_obejct(m: &HashMap<Object, Object>, key: &Object, expected_value: &Object) {
        let result = if let Some(v) = m.get(key) {
            v
        } else {
            panic!("{}", format!(
                "expected an object. keys: {:#?}, values: {:#?}", m.keys(), m.values()
            ).red())
        };

        assert_eq(
            result, expected_value,
            || println!("{}", format!(
                "expected an object. keys: {:#?}, values: {:#?}", m.keys(), m.values()
            ).red())
        );

        println!("{}", format!(
r#"
Test passed. HashMap: keys: {:#?}, values: {:#?}. 
Key {:#?}. 
Expected Value {:#?} Got {:#?}
"#, 
            m.keys(), m.values(), key, expected_value, result
        ).green())
    }

    #[test]
    fn test_hash_map_get_objects() {
        // 创建几个对象干扰一下
        {
            next_id();
            next_id();
            let _ = AntInt::from(0xdeadbeef as usize);
            let _ = AntInt::from(0x33550336);

            next_id();
            next_id();
            next_id();

            AntString::new(String::from("oh dear i am the firefly"));
            next_id();

            let _ = AntInt::from(0b1111111111111000000000000);
            next_id();
        };

        let mut tests = vec![
            (Object::AntInt(AntInt::from(0)), Object::AntInt(AntInt::from(0))),
            (Object::AntString(AntString::new(String::from("ciallo"))), Object::AntInt(AntInt::from(0b110))),
        ];

        let mut m = HashMap::new();

        for (k, v) in &tests {
            m.insert(k.clone(), v.clone());
        }

        for (k, v) in &mut tests {
            // 变更id
            match k {
                Object::AntInt(int) => int.id = next_id(),
                Object::AntString(s) => s.id = next_id(),
                _ => {}
            }

            test_hash_map_get_obejct(&m, k, v);
        }

        println!("{}", format!(
            "All test passed. HashMap: keys: {:#?}, values: {:#?}", m.keys(), m.values()
        ).green())
    }
}