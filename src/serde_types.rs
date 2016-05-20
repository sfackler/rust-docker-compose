use std::collections::HashMap;

pub struct Container {
    pub id: String,
    pub config: Config,
    pub network_settings: NetworkSettings,
}
impl ::serde::de::Deserialize for Container {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Container, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __field2, __ignore, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_usize<E>(&mut self, value: usize)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                0usize => { Ok(__Field::__field0) }
                                1usize => { Ok(__Field::__field1) }
                                2usize => { Ok(__Field::__field2) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "Id" => { Ok(__Field::__field0) }
                                "Config" => { Ok(__Field::__field1) }
                                "NetworkSettings" => { Ok(__Field::__field2) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                b"Id" => { Ok(__Field::__field0) }
                                b"Config" => { Ok(__Field::__field1) }
                                b"NetworkSettings" => {
                                    Ok(__Field::__field2)
                                }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    deserializer.deserialize_struct_field(__FieldVisitor::<D>{phantom:
                                                                                  PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: 


                  ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Container;
                #[inline]
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Container, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field2 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Container{id: __field0,
                                     config: __field1,
                                     network_settings: __field2,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Container, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field1 => {
                                    __field1 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field2 => {
                                    __field2 =
                                        Some(try!(visitor.visit_value()));
                                }
                                _ => {
                                    try!(visitor . visit_value:: < :: serde::
                                         de:: impls:: IgnoredAny > (  ));
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                match visitor.missing_field("id") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                match visitor.missing_field("config") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                match visitor.missing_field("network_settings")
                                    {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        try!(visitor . end (  ));
                        Ok(Container{id: __field0,
                                     config: __field1,
                                     network_settings: __field2,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] =
                &["id", "config", "network_settings"];
            deserializer.deserialize_struct("Container", FIELDS,
                                            __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
pub struct Config {
    pub labels: HashMap<String, String>,
}
impl ::serde::de::Deserialize for Config {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Config, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __ignore, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_usize<E>(&mut self, value: usize)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                0usize => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "Labels" => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                b"Labels" => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    deserializer.deserialize_struct_field(__FieldVisitor::<D>{phantom:
                                                                                  PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Config;
                #[inline]
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Config, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Config{labels: __field0,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Config, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!(visitor.visit_value()));
                                }
                                _ => {
                                    try!(visitor . visit_value:: < :: serde::
                                         de:: impls:: IgnoredAny > (  ));
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                match visitor.missing_field("labels") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        try!(visitor . end (  ));
                        Ok(Config{labels: __field0,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] = &["labels"];
            deserializer.deserialize_struct("Config", FIELDS,
                                            __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
pub struct NetworkSettings {
    pub ports: HashMap<String, Vec<Port>>,
}
impl ::serde::de::Deserialize for NetworkSettings {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<NetworkSettings, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __ignore, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_usize<E>(&mut self, value: usize)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                0usize => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "Ports" => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                b"Ports" => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    deserializer.deserialize_struct_field(__FieldVisitor::<D>{phantom:
                                                                                  PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                NetworkSettings;
                #[inline]
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<NetworkSettings, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(NetworkSettings{ports: __field0,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<NetworkSettings, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!(visitor.visit_value()));
                                }
                                _ => {
                                    try!(visitor . visit_value:: < :: serde::
                                         de:: impls:: IgnoredAny > (  ));
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                match visitor.missing_field("ports") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        try!(visitor . end (  ));
                        Ok(NetworkSettings{ports: __field0,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] = &["ports"];
            deserializer.deserialize_struct("NetworkSettings", FIELDS,
                                            __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
pub struct Port {
    pub host_ip: String,
    pub host_port: String,
}
impl ::serde::de::Deserialize for Port {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Port, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __ignore, }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_usize<E>(&mut self, value: usize)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                0usize => { Ok(__Field::__field0) }
                                1usize => { Ok(__Field::__field1) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "HostIp" => { Ok(__Field::__field0) }
                                "HostPort" => { Ok(__Field::__field1) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                b"HostIp" => { Ok(__Field::__field0) }
                                b"HostPort" => { Ok(__Field::__field1) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    deserializer.deserialize_struct_field(__FieldVisitor::<D>{phantom:
                                                                                  PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Port;
                #[inline]
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Port, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Port{host_ip: __field0, host_port: __field1,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Port, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field1 => {
                                    __field1 =
                                        Some(try!(visitor.visit_value()));
                                }
                                _ => {
                                    try!(visitor . visit_value:: < :: serde::
                                         de:: impls:: IgnoredAny > (  ));
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                match visitor.missing_field("host_ip") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                match visitor.missing_field("host_port") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        try!(visitor . end (  ));
                        Ok(Port{host_ip: __field0, host_port: __field1,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] = &["host_ip", "host_port"];
            deserializer.deserialize_struct("Port", FIELDS,
                                            __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
