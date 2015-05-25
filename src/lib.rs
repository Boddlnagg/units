#![cfg_attr(feature="unstable", feature(unboxed_closures,core))]

extern crate tylar;

#[doc(no_inline)]
pub use tylar::{NumType,Zero,P1};
#[doc(no_inline,hidden)]
pub use tylar::Sub as TSub;
#[doc(no_inline,hidden)]
pub use tylar::Add as TAdd;
#[doc(no_inline,hidden)]
pub use tylar::Halve as THalve;

// TODO: try again to make dimension struct generic in underlying number type
// TODO: support documentation comments/attributes for modules and dimensions/units using `$(#[$attr:meta])*` (blocked on rust/#23812)

#[macro_export]
macro_rules! units {( $name:ident { $( $dim:ident => $uname:ident[$unit:ident]),+ } ) => {
    use std::marker::PhantomData;
    use std::ops::{Add,Sub,Mul,Div,Deref};
    use std::fmt::{Debug, Formatter, Result};
    use $crate::{NumType,Zero,P1,TAdd,TSub,THalve};
    
    #[derive(Copy,Clone,PartialEq,PartialOrd)]
    #[allow(non_snake_case)]
    pub struct $name<_N, $($dim:NumType<$dim>=Zero),+> {
        amount: _N,
        $($dim: PhantomData<$dim>),+
    }
    
    // Constructor (private)
    impl<_N, $($dim:NumType<$dim>),+> $name<_N, $($dim),+> {
        fn new(amount: _N)-> $name<_N, $($dim),+> {
            $name::<_N, $($dim),+ > {
                amount: amount,
                $($dim: PhantomData),+
            }
        }
    }
    
    // Implement sqrt function if underlying numeric type is f32 or f64
    // TODO: other functions that could be implemented: min, max, abs(?)
    
    impl<$($dim:NumType<$dim>),+> $name<f32, $($dim),+> {
        #[allow(non_camel_case_types,dead_code)]
        pub fn sqrt<$($unit:NumType<$unit>),+>(self) -> $name<f32, $($unit),+>  /* abuse $unit for Out parameter */
            where $($dim:THalve<$dim,Out=$unit>),+ {
            $name::new(self.amount.sqrt())
        }
    }
    
    impl<$($dim:NumType<$dim>),+> $name<f64, $($dim),+> {
        #[allow(non_camel_case_types,dead_code)]
        pub fn sqrt<$($unit:NumType<$unit>),+>(self) -> $name<f64, $($unit),+>  /* abuse $unit for Out parameter */
            where $($dim:THalve<$dim,Out=$unit>),+ {
            $name::new(self.amount.sqrt())
        }
    }
    
    // Debug formatting (printing units)
    // TODO: maybe implement Display? / use Debug for underlying number
    impl<_N, $($dim:NumType<$dim>),+> Debug for $name<_N, $($dim),+> where _N: ::std::fmt::Display {
        fn fmt(&self, formatter: &mut Formatter) -> Result {
            try!(self.amount.fmt(formatter));
            let mut num: i32;
            $(
                num = $dim::new().into();
                if num != 0 {
                    try!(write!(formatter, " {}^{}", stringify!($unit), num));
                }
            )+
            Ok(())
        }
    }
    
    // Implementations of Deref and Into<f64> that are only valid for dimensionless quantities
    // (all exponents Zero, which are the default typeparameters)
    impl<_N> Deref for $name<_N> {
        type Target = _N;
        fn deref(&self) -> &Self::Target { &self.amount }
    }
    
    // TODO: Add operators for reference types?
    
    // Addition (only with matching dimensions)
    impl<_N, $($dim:NumType<$dim>),+> Add for $name<_N, $($dim),+> where _N:Add<_N,Output=_N>  {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            $name::new(self.amount + rhs.amount)
        }
    }
    
    // Subtraction (only with matching dimensions)
    impl<_N, $($dim:NumType<$dim>),+> Sub for $name<_N, $($dim),+> where _N:Sub<_N,Output=_N> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            $name::new(self.amount - rhs.amount)
        }
    }
    
    // Multiplications (dimension exponents are added)
    #[allow(non_camel_case_types)]
    impl<_N, $($dim:NumType<$dim>),+ ,
         $($uname:NumType<$uname>),+ , /* abuse $uname for RHS parameter */
         $($unit:NumType<$unit>),+> /* abuse $unit for Out parameter */
        Mul<$name<_N, $($uname),+>> for $name<_N, $($dim),+>
            where _N:Mul<_N,Output=_N>, $($dim:TAdd<$dim,$uname,Out=$unit>),+ {
            
        type Output = $name<_N, $($unit),+>;
        fn mul(self, rhs: $name<_N, $($uname),+>) -> Self::Output {
            $name::new(self.amount * rhs.amount)
        }
    }
    
    // Division (dimension exponents are subtracted)
    #[allow(non_camel_case_types)]
    impl<_N, $($dim:NumType<$dim>),+ ,
         $($uname:NumType<$uname>),+ , /* abuse $uname for RHS parameter */
         $($unit:NumType<$unit>),+> /* abuse $unit for Out parameter */
        Div<$name<_N, $($uname),+>> for $name<_N, $($dim),+>
            where _N:Div<_N,Output=_N>, $($dim:TSub<$dim,$uname,Out=$unit>),+ {
            
        type Output = $name<_N, $($unit),+>;
        fn div(self, rhs: $name<_N, $($uname),+>) -> Self::Output {
            $name::new(self.amount / rhs.amount)
        }
    }
    
    // Mul and Div where LHS is a (dimensionless) f64
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Mul<$name<f64, $($dim),+>> for f64 {
        type Output = $name<f64, $($dim),+>;
        fn mul(self, rhs: $name<f64, $($dim),+>) -> Self::Output {
            $name::new(self * rhs.amount)
        }
    }
    
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Div<$name<f64, $($dim),+>> for f64 {
        type Output = $name<f64, $($dim),+>;
        fn div(self, rhs: $name<f64, $($dim),+>) -> Self::Output {
            $name::new(self / rhs.amount)
        }
    }
    
    // Mul and Div where LHS is a (dimensionless) f32
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Mul<$name<f32, $($dim),+>> for f32 {
        type Output = $name<f32, $($dim),+>;
        fn mul(self, rhs: $name<f32, $($dim),+>) -> Self::Output {
            $name::new(self * rhs.amount)
        }
    }
    
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Div<$name<f32, $($dim),+>> for f32 {
        type Output = $name<f32, $($dim),+>;
        fn div(self, rhs: $name<f32, $($dim),+>) -> Self::Output {
            $name::new(self / rhs.amount)
        }
    }
    
    // overload call operator (only if feature = "unstable")
    __dim_fn_call_helper! { [$name], $($dim),+ }
    
    // type alias and constant for the dimensionless type (all exponents are zero)
    pub type One<_N> = $name<_N>;
    
    // generate aliases of the form `pub type $dim1 = $name<Pos1, Zero, Zero, ...>`
    __dim_type_alias_helper! { [$name], $($dim),+ -> P1 }
    
    $(pub type $uname<_N> = $dim<_N>;)+
    
    pub mod f64 {
        use std::marker::PhantomData;
        pub type One = super::$name<f64>;
        $(pub type $dim = super::$dim<f64>;)+
        $(pub type $uname = super::$uname<f64>;)+
        
        // generate constants of the form `pub const $unit1 = $dim1 { amount: 1.0, ... }`
        __dim_constants_helper! { [$name], $($dim -> $unit),+ => $($dim),+ }
    }
    
    pub mod f32 {
        use std::marker::PhantomData;
        pub type One = super::$name<f32>;
        $(pub type $dim = super::$dim<f32>;)+
        $(pub type $uname = super::$uname<f32>;)+
        
        // generate constants of the form `pub const $unit1 = $dim1 { amount: 1.0, ... }`
        __dim_constants_helper! { [$name], $($dim -> $unit),+ => $($dim),+ }
    }
}}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "unstable")]
macro_rules! __dim_fn_call_helper {
    ( [$name:ident], $($dim:ident),+) => (
       impl<_N, $($dim:NumType<$dim>),+> ::std::ops::FnOnce<(_N,)> for $name<_N, $($dim),+> where _N:Mul<_N,Output=_N> {
            type Output = $name<_N, $($dim),+>;
            
            extern "rust-call" fn call_once(self, args: (_N,)) -> Self::Output {
                $name::new(self.amount * args.0)
            }
        }
    );
}

#[macro_export]
#[doc(hidden)]
#[cfg(not(feature = "unstable"))]
macro_rules! __dim_fn_call_helper {
    // Just return nothing (because function call overloading isn't supported in stable Rust)
    ( [$name:ident], $($dim:ident),+) => ()
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dim_type_alias_helper {
    ( [$name:ident], $dim:ident -> $($types:ty),+ ) => (
        pub type $dim<_N> = $name<_N, $($types),+>;
    );
    
    ( [$name:ident], $dim:ident, $($dims:ident),* -> $($types:ty),+ ) => (
        pub type $dim<_N> = $name<_N, $($types),+>; __dim_type_alias_helper!( [$name], $($dims),* -> Zero, $($types),+);
    )
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dim_constants_helper {
    ( [$name:ident], $dim:ident -> $unit:ident => $($phantom:ident),+ ) => (
        #[allow(dead_code,non_upper_case_globals)]
        pub const $unit: $dim = $dim {
            amount: 1.0,
            $($phantom: PhantomData),+
        };
        
        // finally (in the last recursive iteration) also create a constant for dimensionless `one`
        #[allow(dead_code,non_upper_case_globals)]
        pub const one: One = One {
            amount: 1.0,
            $($phantom: PhantomData),+
        };);
    
    ( [$name:ident], $dim:ident -> $unit:ident, $($dims:ident -> $units:ident),* => $($phantom:ident),+ ) => (
        #[allow(dead_code,non_upper_case_globals)]
        pub const $unit: $dim = $dim {
            amount: 1.0,
            $($phantom: PhantomData),+
        };
        __dim_constants_helper!( [$name], $($dims -> $units),* => $($phantom),+);
    )
}

pub mod si {
    units! {
        SI {
            Length => Metre[m],
            Mass => Kilogram[kg],
            Time => Second[s],
            Current => Ampere[A],
            Temperature => Kelvin[K],
            Amount => Mole[mol],
            LuminousIntensity => Candela[cd]
        }
    }
}