#![cfg_attr(feature="unstable", feature(unboxed_closures,core))]

extern crate tylar;

#[macro_use]
extern crate lazy_static;

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
    pub struct $name<$($dim:NumType<$dim>=Zero),+> {
        amount: f64,
        $($dim: PhantomData<$dim>),+
    }
    
    // Constructor (private) and some functions
    impl<$($dim:NumType<$dim>),+> $name<$($dim),+> {
        fn new(amount: f64)-> $name<$($dim),+> {
            $name::< $($dim),+ > {
                amount: amount,
                $($dim: PhantomData),+
            }
        }
        
        // TODO: other functions that could be implemented: min, max, abs(?)
        #[allow(non_camel_case_types,dead_code)]
        pub fn sqrt<$($unit:NumType<$unit>),+>(self) -> $name<$($unit),+>  /* abuse $unit for Out parameter */
            where $($dim:THalve<$dim,Out=$unit>),+ {
            $name::new(self.amount.sqrt())
        }
    }
    
    // Debug formatting (printing units)
    impl<$($dim:NumType<$dim>),+> Debug for $name<$($dim),+> {
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
    impl Deref for $name {
        type Target = f64;
        fn deref(&self) -> &Self::Target { &self.amount }
    }
    
    impl Into<f64> for $name {
        fn into(self) -> f64 { self.amount }
    }
    
    // TODO: Add operators for reference types?
    
    // Addition (only with matching dimensions)
    impl<$($dim:NumType<$dim>),+> Add for $name<$($dim),+> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            $name::new(self.amount + rhs.amount)
        }
    }
    
    // Subtraction (only with matching dimensions)
    impl<$($dim:NumType<$dim>),+> Sub for $name<$($dim),+> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            $name::new(self.amount - rhs.amount)
        }
    }
    
    // Multiplications (dimension exponents are added)
    #[allow(non_camel_case_types)]
    impl<$($dim:NumType<$dim>),+ ,
         $($uname:NumType<$uname>),+ , /* abuse $uname for RHS parameter */
         $($unit:NumType<$unit>),+> /* abuse $unit for Out parameter */
        Mul<$name<$($uname),+>> for $name<$($dim),+>
            where $($dim:TAdd<$dim,$uname,Out=$unit>),+ {
            
        type Output = $name<$($unit),+>;
        fn mul(self, rhs: $name<$($uname),+>) -> Self::Output {
            $name::new(self.amount * rhs.amount)
        }
    }
    
    // Division (dimension exponents are subtracted)
    #[allow(non_camel_case_types)]
    impl<$($dim:NumType<$dim>),+ ,
         $($uname:NumType<$uname>),+ , /* abuse $uname for RHS parameter */
         $($unit:NumType<$unit>),+> /* abuse $unit for Out parameter */
        Div<$name<$($uname),+>> for $name<$($dim),+>
            where $($dim:TSub<$dim,$uname,Out=$unit>),+ {
            
        type Output = $name<$($unit),+>;
        fn div(self, rhs: $name<$($uname),+>) -> Self::Output {
            $name::new(self.amount / rhs.amount)
        }
    }
    
    // Mul and Div where one operand is a (dimensionless) float
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Mul<$name<$($dim),+>> for f64 {
        type Output = $name<$($dim),+>;
        fn mul(self, rhs: $name<$($dim),+>) -> Self::Output {
            $name::new(self * rhs.amount)
        }
    }
    
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Div<$name<$($dim),+>> for f64 {
        type Output = $name<$($dim),+>;
        fn div(self, rhs: $name<$($dim),+>) -> Self::Output {
            $name::new(self / rhs.amount)
        }
    }

    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Mul<f64> for $name<$($dim),+> {
        type Output = $name<$($dim),+>;
        fn mul(self, rhs: f64) -> Self::Output {
            $name::new(self.amount * rhs)
        }
    }
    
    #[cfg(not(feature = "unstable"))]
    impl<$($dim:NumType<$dim>),+> Div<f64> for $name<$($dim),+> {
        type Output = $name<$($dim),+>;
        fn div(self, rhs: f64) -> Self::Output {
            $name::new(self.amount / rhs)
        }
    }
    
    __dim_fn_call_helper! { [$name], $($dim),+ }
    
    // type alias and constant for the dimensionless type (all exponents are zero)
    pub type One = $name;
    
    #[allow(dead_code,non_upper_case_globals)]
    pub const one: One = One {
        amount: 1.0,
        $($dim: PhantomData),+
    };
    
    // generate aliases of the form `pub type $dim1 = $name<Pos1, Zero, Zero, ...>`
    __dim_type_alias_helper! { [$name], $($dim),+ -> P1 }
    
    $(pub type $uname = $dim;)+
    
    // generate constants of the form `pub const $unit1 = $dim1 { amount: 1.0, ... }`
    __dim_constants_helper! { [$name], $($dim -> $unit),+ => $($dim),+ }
}}

#[macro_export]
#[doc(hidden)]
#[cfg(feature = "unstable")]
macro_rules! __dim_fn_call_helper {
    ( [$name:ident], $($dim:ident),+) => (
       impl<$($dim:NumType<$dim>),+> ::std::ops::FnOnce<(f64,)> for $name<$($dim),+> {
            type Output = $name<$($dim),+>;
            
            extern "rust-call" fn call_once(self, args: (f64,)) -> Self::Output {
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
        pub type $dim = $name<$($types),+>;
    );
    
    ( [$name:ident], $dim:ident, $($dims:ident),* -> $($types:ty),+ ) => (
        pub type $dim = $name<$($types),+>; __dim_type_alias_helper!( [$name], $($dims),* -> Zero, $($types),+);
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
        }; );
    
    ( [$name:ident], $dim:ident -> $unit:ident, $($dims:ident -> $units:ident),* => $($phantom:ident),+ ) => (
        #[allow(dead_code,non_upper_case_globals)]
        pub const $unit: $dim = $dim {
            amount: 1.0,
            $($phantom: PhantomData),+
        };
        __dim_constants_helper!( [$name], $($dims -> $units),* => $($phantom),+);
    )
}

#[allow(non_upper_case_globals)]
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
    
    // TODO: Either remove these or add more derived units.
    //       The complexity of this implementation (lazy_static, etc) might not be justifiable
    //       given that derived units can easily be defined at use-site (using `let`-constructs)
    pub mod derived {
        use tylar::{Zero,P1,N1,N2};
        use super::{SI,one,m,kg,s,Length};
    
        // We need lazy_static! here, because overloaded operators/function calls can't be evaluated in static/const
        
        #[cfg(feature = "unstable")]
        lazy_static! {
            pub static ref cm: Length = m(1./100.);
            pub static ref mm: Length = m(1./1000.);
            pub static ref km: Length = m(1000.);
        }
        
        #[cfg(not(feature = "unstable"))]
        lazy_static! {
            pub static ref cm: Length = 1./100.*m;
            pub static ref mm: Length = 1./1000.*m;
            pub static ref km: Length = 1000.*m;
        }
        
        lazy_static! {
            pub static ref N: SI<P1,P1,N2> = kg*m/s/s;
            pub static ref Hz: SI<Zero,Zero,N1> = one/s;
        }
    }
}