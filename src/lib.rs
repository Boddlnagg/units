#![cfg_attr(feature="unstable", feature(unboxed_closures, core, zero_one))]

extern crate tylar;

#[doc(no_inline)]
pub use tylar::{NumType,Zero,P1};
#[doc(no_inline,hidden)]
pub use tylar::Sub as TSub;
#[doc(no_inline,hidden)]
pub use tylar::Add as TAdd;
#[doc(no_inline,hidden)]
pub use tylar::Halve as THalve;

use std::fmt::{Formatter,Result};

// TODO: support documentation comments/attributes for modules and dimensions/units using `$(#[$attr:meta])*` (blocked on rust/#23812)

pub trait UnitFormat { fn fmt(&mut Formatter) -> Result; }
pub trait UnitZero {}
pub trait UnitAdd<RHS> { type Out; }
pub trait UnitSub<RHS> { type Out; }
pub trait UnitMul<RHS> { type Out; }
pub trait UnitDiv<RHS> { type Out; }
pub trait UnitSqrt { type Out; }

#[macro_export]
macro_rules! units {( $name:ident { $( $dim:ident[$unit:ident]),+ } ) => {
    use $crate::{UnitZero,UnitAdd,UnitSub,UnitMul,UnitDiv,UnitSqrt,UnitFormat};
    use std::marker::PhantomData;
    use std::fmt::{Debug,Formatter,Result};
    use std::ops::{Add,Sub,Mul,Div,Deref};
    use $crate::{NumType,Zero,P1,TAdd,TSub,THalve};
    
    // TODO: move as much as possible of the following impls out of the macro (using a helper trait)
    #[derive(Copy,Clone,PartialEq,PartialOrd,Eq,Ord)]    
    pub struct $name<D,N=f64> {
        amount: N,
        phantom: PhantomData<D>
    }

    impl<D,N> $name<D,N> {
        fn new(v: N) -> Self {
            $name { amount: v, phantom: PhantomData }
        }
    }

    impl<D:UnitFormat,N> Debug for $name<D,N> where N:Debug {
        fn fmt(&self, formatter: &mut Formatter) -> Result {
            try!(self.amount.fmt(formatter));
            D::fmt(formatter)
        }
    }

    // TODO: Add operators for reference types?

    impl<D1,D2,N1,N2> Add<$name<D2,N2>> for $name<D1,N1> where D1:UnitAdd<D2>, N1:Add<N2> {
        type Output = $name<D1::Out, N1::Output>;
        
        fn add(self, rhs: $name<D2,N2>) -> Self::Output {
            $name::new(self.amount + rhs.amount)
        }
    }

    impl<D1,D2,N1,N2> Sub<$name<D2,N2>> for $name<D1,N1> where D1:UnitSub<D2>, N1:Sub<N2> {
        type Output = $name<D1::Out, N1::Output>;
        
        fn sub(self, rhs: $name<D2,N2>) -> Self::Output {
            $name::new(self.amount - rhs.amount)
        }
    }

    impl<D1,D2,N1,N2> Mul<$name<D2,N2>> for $name<D1,N1> where D1:UnitMul<D2>, N1:Mul<N2> {
        type Output = $name<D1::Out, N1::Output>;
        
        fn mul(self, rhs: $name<D2,N2>) -> Self::Output {
            $name::new(self.amount * rhs.amount)
        }
    }

    impl<D1,D2,N1,N2> Div<$name<D2,N2>> for $name<D1,N1> where D1:UnitDiv<D2>, N1: Div<N2> {
        type Output = $name<D1::Out, N1::Output>;
        
        fn div(self, rhs: $name<D2,N2>) -> Self::Output {
            $name::new(self.amount / rhs.amount)
        }
    }

    // Implement sqrt function if underlying numeric type is f32 or f64
    // TODO: other functions that could be implemented: min, max, abs(?)

    #[allow(dead_code)]
    impl<D1> $name<D1,f64> where D1:UnitSqrt {
        pub fn sqrt(self) -> $name<D1::Out,f64> {
            $name::new(self.amount.sqrt())
        }
    }

    #[allow(dead_code)]
    impl<D1> $name<D1,f32> where D1:UnitSqrt {
        pub fn sqrt(self) -> $name<D1::Out,f32> {
            $name::new(self.amount.sqrt())
        }
    }

    // Implementation of Deref that is only valid for dimensionless quantities (all exponents Zero)
    impl<D:UnitZero,N> Deref for $name<D,N> {
        type Target = N;
        fn deref(&self) -> &Self::Target { &self.amount }
    }

    // Implementation of Zero (unstable), which is valid for all units, since zero is the only
    // value that is polymorphic in its unit. std::num::One is deliberately NOT implemented.
    #[cfg(feature = "unstable")]
    impl <D,N> ::std::num::Zero for $name<D,N> where N: ::std::num::Zero {
        fn zero() -> Self {
            $name::new(N::zero())
        }
    }

    #[cfg(feature = "unstable")]
    impl<D,N> FnOnce<(N,)> for $name<D,N> where N:Mul<N,Output=N> {
        type Output = $name<D,N>;
        extern "rust-call" fn call_once(self, args: (N,)) -> Self::Output {
            $name::new(self.amount * args.0)
        }
    }

    // One operand is a (dimensionless) float
    #[cfg(not(feature = "unstable"))]
    impl<D> Mul<Dim<D,f64>> for f64 {
        type Output = $name<D,f64>;
        fn mul(self, rhs: $name<D,f64>) -> Self::Output {
            $name::new(self * rhs.amount)
        }
    }

    #[cfg(not(feature = "unstable"))]
    impl<D> Mul<$name<D,f32>> for f32 {
        type Output = $name<D,f32>;
        fn mul(self, rhs: $name<D,f32>) -> Self::Output {
            $name::new(self * rhs.amount)
        }
    }
    
    #[derive(Copy,Clone,PartialEq,PartialOrd,Eq,Ord)]
    #[allow(non_snake_case)]
    pub struct Unit<$($dim:NumType<$dim>=Zero),+> {
        $($dim: PhantomData<$dim>),+
    }
    
    // Debug formatting (printing units)
    // TODO: maybe implement Display?
    impl<$($dim:NumType<$dim>),+> UnitFormat for Unit<$($dim),+> {
        fn fmt(formatter: &mut Formatter) -> Result {
            let mut exp: i32;
            $(
                exp = $dim::new().into();
                match exp {
                    0 => (),
                    1 => try!(write!(formatter, " {}", stringify!($unit))),
                    _ => try!(write!(formatter, " {}^{:?}", stringify!($unit), exp))
                }
            )+
            Ok(())
        }
    }
    
    impl<$($dim:NumType<$dim>),+> UnitAdd<Unit<$($dim),+>> for Unit<$($dim),+> { type Out = Unit<$($dim),+>; }

    impl<$($dim:NumType<$dim>),+> UnitSub<Unit<$($dim),+>> for Unit<$($dim),+> { type Out = Unit<$($dim),+>; }

    // In Mul and Div implementations we abuse $unit for the RHS type parameter name
    
    #[allow(non_camel_case_types)]
    impl<$($dim:NumType<$dim>),+ , $($unit:NumType<$unit>),+> UnitMul<Unit<$($unit),+>> for Unit<$($dim),+>
        where $($dim:TAdd<$dim,$unit>),+ { type Out = Unit<$(<$dim as TAdd<$dim,$unit>>::Out),+>; }
    
    #[allow(non_camel_case_types)]    
    impl<$($dim:NumType<$dim>),+ , $($unit:NumType<$unit>),+> UnitDiv<Unit<$($unit),+>> for Unit<$($dim),+>
        where $($dim:TSub<$dim,$unit>),+ { type Out = Unit<$(<$dim as TSub<$dim,$unit>>::Out),+>; }
        
    impl<$($dim:NumType<$dim>),+> UnitSqrt for Unit<$($dim),+>
        where $($dim:THalve<$dim>),+ { type Out = Unit<$(<$dim as THalve<$dim>>::Out),+>; }
        
    // type alias and `UnitZero` impl for the dimensionless type (all exponents are zero)
    pub type One = Unit;
    impl UnitZero for One {}
    
    // generate aliases of the form `pub type $dim1 = Unit<Pos1, Zero, Zero, ...>`
    __dim_type_alias_helper! { $($dim),+ -> P1 }
    
    pub mod f64 {
        use std::marker::PhantomData;
        use super::{$name,One,$($dim),+};
        
        #[allow(non_upper_case_globals, dead_code)]
        pub const one: $name<One, f64> = $name {
            amount: 1.0,
            phantom: PhantomData
        };
        
        $(
        #[allow(non_upper_case_globals, dead_code)]
        pub const $unit: $name<$dim, f64> = $name {
            amount: 1.0,
            phantom: PhantomData
        };
        )+
    }
    
    pub mod f32 {
        use std::marker::PhantomData;
        use super::{$name,One,$($dim),+};
        
        #[allow(non_upper_case_globals, dead_code)]
        pub const one: $name<One, f32> = $name {
            amount: 1.0,
            phantom: PhantomData
        };
        
        $(
        #[allow(non_upper_case_globals, dead_code)]
        pub const $unit: $name<$dim, f32> = $name {
            amount: 1.0,
            phantom: PhantomData
        };
        )+
    }
}}

#[macro_export]
#[doc(hidden)]
macro_rules! __dim_type_alias_helper {
    ( $dim:ident -> $($types:ty),+ ) => (
        pub type $dim = Unit<$($types),+>;
    );
    
    ( $dim:ident, $($dims:ident),* -> $($types:ty),+ ) => (
        pub type $dim = Unit<$($types),+>; __dim_type_alias_helper!( $($dims),* -> Zero, $($types),+);
    )
}

pub mod si {
    units! {
        SI {
            Length[m],
            Mass[kg],
            Time[s],
            Current[A],
            Temperature[K],
            Amount[mol],
            LuminousIntensity[cd]
        }
    }
}