macro_rules! downcast_get_type_id {
    () => {
        #[allow(dead_code)]
        fn __private_get_type_id__(&self, _: PrivateHelper) -> (std::any::TypeId, PrivateHelper)
        where
            Self: 'static,
        {
            (std::any::TypeId::of::<Self>(), PrivateHelper(()))
        }
    };
}

macro_rules! downcast_dyn {
    ($name:ident) => {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub struct PrivateHelper(());

        impl dyn $name + 'static {
            #[allow(dead_code)]
            pub fn downcast_ref<T: $name + 'static>(&self) -> Option<&T> {
                if self.__private_get_type_id__(PrivateHelper(())).0 == std::any::TypeId::of::<T>()
                {
                    unsafe { Some(&*(self as *const dyn $name as *const T)) }
                } else {
                    None
                }
            }

            #[allow(dead_code)]
            pub fn downcast_mut<T: $name + 'static>(&mut self) -> Option<&mut T> {
                if self.__private_get_type_id__(PrivateHelper(())).0 == std::any::TypeId::of::<T>()
                {
                    unsafe { Some(&mut *(self as *const dyn $name as *const T as *mut T)) }
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

pub(crate) use downcast_dyn;
pub(crate) use downcast_get_type_id;
pub(crate) use status_codes;

#[cfg(test)]
mod tests {
    #![allow(clippy::upper_case_acronyms)]

    trait MB {
        downcast_get_type_id!();
    }

    downcast_dyn!(MB);

    impl MB for String {}
    impl MB for () {}

    #[actix_rt::test]
    async fn test_any_casting() {
        let mut body = String::from("hello cast");
        let resp_body: &mut dyn MB = &mut body;
        let body = resp_body.downcast_ref::<String>().unwrap();
        assert_eq!(body, "hello cast");
        let body = resp_body.downcast_mut::<String>().unwrap();
        body.push('!');
        let body = resp_body.downcast_ref::<String>().unwrap();
        assert_eq!(body, "hello cast!");
        let not_body = resp_body.downcast_ref::<()>();
        assert!(not_body.is_none());
    }
}
