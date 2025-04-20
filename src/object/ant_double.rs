use std::any::Any;
use std::ops::Deref;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use num_bigint::BigInt;

use crate::object::object::GetEnv;
use crate::constants::{null_obj, uninit_obj};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::impl_object_get_env_function;
use crate::object::ant_native_function::create_ant_native_function;
use crate::object::object::{IAntObject, ObjectType, DOUBLE};

pub struct AntDouble {
    id: Uuid,
    env: Environment,
    value: BigDecimal,
}

impl Clone for AntDouble {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            env: self.env.clone(),
            value: self.value.clone(),
        }
    }
}

fn init_env(int_obj: &mut AntDouble) {
    fn plus(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn plus_int(me: AntDouble, other: AntDouble) -> Option<Box<dyn IAntObject>> {
            Some(
                AntDouble::new_with_native_value(Box::new(other.value + me.value))
            )
        }

        let me = arg_env.get("me").expect(
            &format!("what the fuck? arg_env: {}", arg_env.to_string())
        ).as_any().downcast_ref::<AntDouble>().cloned().unwrap();

        let value = arg_env.get("value");
        if value.is_none() {
            panic!("what the fuck? arg_env: {}", arg_env.to_string())
        }

        let value = value.unwrap().as_any().downcast_ref::<AntDouble>().cloned();

        if let Some(it) = value {
            return plus_int(me, it)
        }

        None
    }

    fn minus(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn minus_int(me: AntDouble, other: AntDouble) -> Option<Box<dyn IAntObject>> {
            Some(
                AntDouble::new_with_native_value(Box::new(me.value - other.value))
            )
        }

        let me = arg_env.get("me").expect(
            &format!("what the fuck? arg_env: {}", arg_env.to_string())
        ).as_any().downcast_ref::<AntDouble>().cloned().unwrap();

        let value = arg_env.get("value");
        if value.is_none() {
            panic!("what the fuck? arg_env: {}", arg_env.to_string())
        }

        let value = value.unwrap().as_any().downcast_ref::<AntDouble>().cloned();

        if let Some(it) = value {
            return minus_int(me, it)
        }

        None
    }

    fn multiply(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn multiply_int(me: AntDouble, other: AntDouble) -> Option<Box<dyn IAntObject>> {
            Some(
                AntDouble::new_with_native_value(Box::new(me.value * other.value))
            )
        }

        let me = arg_env.get("me").expect(
            &format!("what the fuck? arg_env: {}", arg_env.to_string())
        ).as_any().downcast_ref::<AntDouble>().cloned().unwrap();

        let value = arg_env.get("value");
        if value.is_none() {
            panic!("what the fuck? arg_env: {}", arg_env.to_string())
        }

        let value = value.unwrap().as_any().downcast_ref::<AntDouble>().cloned();

        if let Some(it) = value {
            return multiply_int(me, it)
        }

        None
    }

    fn divide(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
        fn divide_int(me: AntDouble, other: AntDouble) -> Option<Box<dyn IAntObject>> {
            Some(
                AntDouble::new_with_native_value(Box::new(me.value / other.value))
            )
        }

        let me = arg_env.get("me").expect(
            &format!("what the fuck? arg_env: {}", arg_env.to_string())
        ).as_any().downcast_ref::<AntDouble>().cloned().unwrap();

        let value = arg_env.get("value");
        if value.is_none() {
            panic!("what the fuck? arg_env: {}", arg_env.to_string())
        }

        let value = value.unwrap().as_any().downcast_ref::<AntDouble>().cloned();

        if let Some(it) = value {
            return divide_int(me, it)
        }

        None
    }

    let func_param_env = create_env(
        vec![
            ("me".to_string(), Box::new(int_obj.clone())),
            ("value".to_string(), uninit_obj.clone())
        ]
    );

    let plus_int_native_func = create_ant_native_function(func_param_env.clone(), plus);

    let minus_int_native_func = create_ant_native_function(func_param_env.clone(), minus);

    let multiply_int_native_func = create_ant_native_function(func_param_env.clone(), multiply);

    let divide_int_native_func = create_ant_native_function(func_param_env.clone(), divide);

    int_obj.env.create("plus", Data::new(plus_int_native_func, DataInfo::new(false)));
    int_obj.env.create("minus", Data::new(minus_int_native_func, DataInfo::new(false)));
    int_obj.env.create("multiply", Data::new(multiply_int_native_func, DataInfo::new(false)));
    int_obj.env.create("divide", Data::new(divide_int_native_func, DataInfo::new(false)));
}

impl IAntObject for AntDouble {
    fn get_type(&self) -> ObjectType {
        DOUBLE.to_string()
    }

    fn get_value(&self) -> Box<dyn Any> {
        Box::new(self.value.clone())
    }

    fn get_base(&self) -> Option<Box<dyn IAntObject>> {
        None
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn inspect(&self) -> String {
        format!("{}", self.value.to_string())
    }

    fn new(arg_env: Environment) -> Box<dyn IAntObject> {
        let mut value = BigDecimal::from(0);

        let mut new = |obj: Box<dyn IAntObject>| {
            let cast_obj =  obj.as_any().downcast_ref::<AntDouble>().cloned();
            match cast_obj {
                None => {
                    panic!()
                }
                Some(double_obj) => {
                    value = double_obj.value
                }
            }
        };

        let mut env = Environment::new();
        env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

        env.fusion(arg_env);

        if env.get("value").unwrap().eq(null_obj.clone().deref()) {
            panic!()
        }

        new(env.get("value").unwrap());

        let mut obj = Self {
            id: Uuid::new_v4(),
            env: env.clone(),
            value,
        };

        init_env(&mut obj);

        Box::new(obj)
    }

    fn new_with_native_value(mut value: Box<dyn Any>) -> Box<dyn IAntObject> {
        let cast_result = value.downcast_mut::<BigDecimal>().cloned();

        match cast_result {
            None => {
                panic!("value is not BigDecimal")
            }
            Some(big_decimal) => {
                let mut env = Environment::new();
                env.create("value", Data::new(null_obj.clone(), DataInfo::new(false)));

                let mut obj = Self {
                    id: Uuid::new_v4(),
                    env,
                    value: big_decimal.clone(),
                };

                init_env(&mut obj);

                Box::new(obj)
            }
        }
    }

    fn eq(&self, other: &dyn IAntObject) -> bool {
        other.get_id() == self.id || if other.get_type() == DOUBLE {
            other.as_any().downcast_ref::<AntDouble>().unwrap().value == self.value
        } else {false}
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl_object_get_env_function!(AntDouble);
