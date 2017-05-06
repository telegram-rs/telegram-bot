macro_rules! integer_id_impls {
    ($name: ident) => {
        impl $name {
            pub fn new(inner: Integer) -> Self {
                $name(inner)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Integer> for $name {
            fn from(inner: Integer) -> Self {
                $name::new(inner)
            }
        }

        impl From<$name> for Integer {
            fn from(from: $name) -> Self {
                from.0
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
                where D: ::serde::de::Deserializer<'de>
            {
                let inner = ::serde::de::Deserialize::deserialize(deserializer)?;
                Ok($name::new(inner))
            }
        }

        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer
            {
                serializer.serialize_i64(self.0)
            }
        }
    };
}
