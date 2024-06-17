pub fn capitalise(mut s: String) -> String {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    s
}

pub fn capitalise_second(mut s: String) -> String {
    if let Some(r) = s.get_mut(1..2) {
        r.make_ascii_uppercase();
    }
    s
}

#[macro_export]
macro_rules! static_gen_fn {
    ($fn_name:ident, $gen_fn_name:ident, $this_type:ty, $ret_type:ty, $body:expr) => {
        impl $this_type {
            fn $gen_fn_name() -> Vec<$ret_type> {
                $body
            }

            pub fn $fn_name() -> &'static [$ret_type] {
                use once_cell::sync::Lazy;
                static RETVAL: Lazy<Vec<$ret_type>> = Lazy::new(|| <$this_type>::$gen_fn_name());
                &RETVAL
            }
        }
    };
}

pub(crate) use static_gen_fn;

#[macro_export]
macro_rules! static_gen_fn_all {
    ($t:ty) => {
        static_gen_fn!(all, generate_all, $t, $t, {
            use strum::IntoEnumIterator;
            Self::iter().collect()
        });
    };
}

pub(crate) use static_gen_fn_all;
