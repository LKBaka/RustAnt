#[macro_export] 
macro_rules! impl_object {
    ($struct_name:ident) => {
        impl EnvGetter for $struct_name {
            fn get_env(&self) -> Environment {
                return self.env.clone()
            }

            fn get_env_ref(&mut self) -> &mut Environment {
                return &mut self.env
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.equals(*Box::new(other))
            }
        }        

        unsafe impl Send for $struct_name {}
        unsafe impl Sync for $struct_name {}
    };
}

#[macro_export] 
macro_rules! extract_arg {
    ($env:expr, $key:expr => $type:ty) => {
        {
            let obj = $env.get($key).expect(&format!("cannot find '{}'. arg_env: {}", $key, $env.to_string()));
        
            let return_value: Option<$type>;
        
            return_value = if let Some(it) = obj.as_any().downcast_ref::<$type>() {
                Some(it.to_owned())
            } else {
                Option::None
            };

            return_value
        }
    };
}

#[macro_export] 
macro_rules! impl_plus_func {
    ($func_name:ident, $me_type:ty, $other_type:ty, $result_type:ty) => {
        fn $func_name(me: $me_type, other: $other_type) -> Option<Object> {
            Some(
                <$result_type>::new_with_native_value(Box::new(other.value + me.value))
            )
        }
    };
}

#[macro_export] 
macro_rules! impl_minus_func {
    ($func_name:ident, $me_type:ty, $other_type:ty, $result_type:ty) => {
        fn $func_name(me: $me_type, other: $other_type) -> Option<Object> {
            Some(
                <$result_type>::new_with_native_value(Box::new(me.value - other.value))
            )
        }
    };
}

#[macro_export] 
macro_rules! impl_multiply_func {
    ($func_name:ident, $me_type:ty, $other_type:ty, $result_type:ty) => {
        fn $func_name(me: $me_type, other: $other_type) -> Option<Object> {
            Some(
                <$result_type>::new_with_native_value(Box::new(other.value * me.value))
            )
        }
    };
}
