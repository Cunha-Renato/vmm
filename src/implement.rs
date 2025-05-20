macro_rules! impls {
    (
        { $( $dec:ident ),* }
        $scalar:ident,
        $typ:ident,
        $target:ty
    ) => {
        impl<$scalar, $(const $dec: usize),*> core::ops::Deref for $typ<$scalar, $( $dec ),*> {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$scalar, $(const $dec: usize),*> core::ops::DerefMut for $typ<$scalar, $( $dec ),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<$scalar, $(const $dec: usize),*> From<$typ<$scalar, $( $dec ),*>> for $target {
            fn from(val: $typ<$scalar, $( $dec ),*>) -> Self {
                val.0
            }
        }

        impl<$scalar, $(const $dec: usize),*> From<$target> for $typ<$scalar, $( $dec ),*> {
            fn from(val: $target) -> Self {
                Self(val)
            }
        }
    };
}
