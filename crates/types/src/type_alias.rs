#[macro_export]
macro_rules! type_alias {
    ($name:ident; String) => {
        #[derive(Debug)]
        struct $name(String);

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl Into<String> for $name {
            fn into(self) -> String {
                self.0
            }
        }
    };
    ($name:ident; $type:ty) => {
        #[derive(Debug)]
        struct $name($type);

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl Into<$type> for $name {
            fn into(self) -> $type {
                self.0
            }
        }
    };
}
