#[macro_export]
macro_rules! impl_object {
    ($struct_name:ident) => {
        impl crate::object::object::AsAnyMut for $struct_name {
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.equals(*Box::new(other))
            }
        }

        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "[Debug] TypeName: {}, Inspect: {}",
                    self.get_type(),
                    self.inspect()
                )
            }
        }

        unsafe impl Send for $struct_name {}
        unsafe impl Sync for $struct_name {}
    };
}

#[macro_export]
macro_rules! extract_arg {
    ($env:expr, $key:expr => $type:ty) => {{
        let obj = $env.get($key).expect(&format!(
            "cannot find '{}'. arg_env: {}",
            $key,
            $env.to_string()
        ));

        let return_value: Option<$type>;

        return_value = if let Some(it) = obj.as_any().downcast_ref::<$type>() {
            Some(it.clone())
        } else {
            Option::None
        };

        return_value
    }};
}
