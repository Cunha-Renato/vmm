macro_rules! first {
    () => {
        #[inline(always)]
        pub const fn x(&self) -> T {
            self.0[0][0]
        }

        #[inline(always)]
        pub const fn r(&self) -> T {
            self.x()
        }
    };
}

macro_rules! second {
    () => {
        #[inline(always)]
        pub const fn y(&self) -> T {
            self.0[0][1]
        }

        #[inline(always)]
        pub const fn g(&self) -> T {
            self.y()
        }

        #[inline(always)]
        pub const fn xy(&self) -> [T; 2] {
            [self.x(), self.y()]
        }

        #[inline(always)]
        pub const fn rg(&self) -> [T; 2] {
            [self.x(), self.y()]
        }
    };
}

macro_rules! third {
    () => {
        #[inline(always)]
        pub const fn z(&self) -> T {
            self.0[0][2]
        }

        #[inline(always)]
        pub const fn b(&self) -> T {
            self.z()
        }

        #[inline(always)]
        pub const fn xyz(&self) -> [T; 3] {
            [self.x(), self.y(), self.z()]
        }

        #[inline(always)]
        pub const fn rgb(&self) -> [T; 3] {
            self.xyz()
        }
    };
}

macro_rules! fourth {
    () => {
        #[inline(always)]
        pub const fn w(&self) -> T {
            self.0[0][3]
        }

        #[inline(always)]
        pub const fn a(&self) -> T {
            self.w()
        }

        #[inline(always)]
        pub const fn xyzw(&self) -> [T; 4] {
            [self.x(), self.y(), self.z(), self.w()]
        }

        #[inline(always)]
        pub const fn rgba(&self) -> [T; 4] {
            self.xyzw()
        }
    };
}
