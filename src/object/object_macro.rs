#[macro_export] macro_rules! impl_object_get_env_function {
    ($struct_name:ident) => {
        impl GetEnv for $struct_name {
            fn get_env(&self) -> Environment {
                return self.env.clone()
            }

            fn get_env_ref(&self) -> &Environment {
                return &self.env
            }
        }
    };
}