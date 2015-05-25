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

pub trait DimFormat { fn fmt(&mut Formatter) -> Result; }
pub trait DimZero {}
pub trait DimAdd<RHS> { type Out; }
pub trait DimSub<RHS> { type Out; }
pub trait DimMul<RHS> { type Out; }
pub trait DimDiv<RHS> { type Out; }
pub trait DimSqrt { type Out; }

#[macro_export]
macro_rules! units {( $name:ident { $( $dim:ident => $uname:ident[$unit:ident]),+ } ) => {
    use $crate::{DimZero,DimAdd,DimSub,DimMul,DimDiv,DimSqrt,DimFormat};
    use std::marker::PhantomData;
    use std::fmt::{Debug,Formatter,Result};
    use std::ops::{Add,Sub,Mul,Div,Deref};
    use $crate::{NumType,Zero,P1,TAdd,TSub,THalve};
    
    // TODO: move as much as possible of the Dim impls out of the macro (using a helper trait)
    #[derive(Copy,Clone,PartialEq,PartialOrd,Eq,Ord)]    
    pub struct Dim<D,N=f64> {
        amount: N,
        phantom: PhantomData<D>
    }

    impl<D,N> Dim<D,N> {
        fn new(v: N) -> Self {
            Dim { amount: v, phantom: PhantomData }
        }
    }

    impl<D:DimFormat,N> Debug for Dim<D,N> where N:Debug {
        fn fmt(&self, formatter: &mut Formatter) -> Result {
            try!(self.amount.fmt(formatter));
            D::fmt(formatter)
        }
    }

    // TODO: Add operators for reference types?

    impl<D1,D2,N1,N2> Add<Dim<D2,N2>> for Dim<D1,N1> where D1:DimAdd<D2>, N1:Add<N2> {
        type Output = Dim<D1::Out, N1::Output>;
        
        fn add(self, rhs: Dim<D2,N2>) -> Self::Output {
            Dim::new(self.amount + rhs.amount)
        }
    }

    impl<D1,D2,N1,N2> Sub<Dim<D2,N2>> for Dim<D1,N1> where D1:DimSub<D2>, N1:Sub<N2> {
        type Output = Dim<D1::Out, N1::Output>;
        
        fn sub(self, rhs: Dim<D2,N2>) -> Self::Output {
            Dim::new(self.amount - rhs.amount)
        }
    }

    impl<D1,D2,N1,N2> Mul<Dim<D2,N2>> for Dim<D1,N1> where D1:DimMul<D2>, N1:Mul<N2> {
        type Output = Dim<D1::Out, N1::Output>;
        
        fn mul(self, rhs: Dim<D2,N2>) -> Self::Output {
            Dim::new(self.amount * rhs.amount)
        }
    }

    impl<D1,D2,N1,N2> Div<Dim<D2,N2>> for Dim<D1,N1> where D1:DimDiv<D2>, N1: Div<N2> {
        type Output = Dim<D1::Out, N1::Output>;
        
        fn div(self, rhs: Dim<D2,N2>) -> Self::Output {
            Dim::new(self.amount / rhs.amount)
        }
    }

    // Implement sqrt function if underlying numeric type is f32 or f64
    // TODO: other functions that could be implemented: min, max, abs(?)

    impl<D1> Dim<D1,f64> where D1:DimSqrt {
        pub fn sqrt(self) -> Dim<D1::Out,f64> {
            Dim::new(self.amount.sqrt())
        }
    }

    impl<D1> Dim<D1,f32> where D1:DimSqrt {
        pub fn sqrt(self) -> Dim<D1::Out,f32> {
            Dim::new(self.amount.sqrt())
        }
    }

    // Implementation of Deref that is only valid for dimensionless quantities (all exponents Zero)
    impl<D:DimZero,N> Deref for Dim<D,N> {
        type Target = N;
        fn deref(&self) -> &Self::Target { &self.amount }
    }

    // Implementation of Zero (unstable), which is valid for all units, since zero is the only
    // value that is polymorphic in its unit. std::num::One is deliberately NOT implemented.
    #[cfg(feature = "unstable")]
    impl <D,N> ::std::num::Zero for Dim<D,N> where N: ::std::num::Zero {
        fn zero() -> Self {
            Dim::new(N::zero())
        }
    }

    #[cfg(feature = "unstable")]
    impl<D,N> FnOnce<(N,)> for Dim<D,N> where N:Mul<N,Output=N> {
        type Output = Dim<D,N>;
        extern "rust-call" fn call_once(self, args: (N,)) -> Self::Output {
            Dim::new(self.amount * args.0)
        }
    }

    // One operand is a (dimensionless) float
    #[cfg(not(feature = "unstable"))]
    impl<D> Mul<Dim<D,f64>> for f64 {
        type Output = Dim<D,f64>;
        fn mul(self, rhs: Dim<D,f64>) -> Self::Output {
            Dim::new(self * rhs.amount)
        }
    }

    #[cfg(not(feature = "unstable"))]
    impl<D> Mul<Dim<D,f32>> for f32 {
        type Output = Dim<D,f32>;
        fn mul(self, rhs: Dim<D,f32>) -> Self::Output {
            Dim::new(self * rhs.amount)
        }
    }
    
    #[derive(Copy,Clone,PartialEq,PartialOrd,Eq,Ord)]
    #[allow(non_snake_case)]
    pub struct $name<$($dim:NumType<$dim>=Zero),+> {
        $($dim: PhantomData<$dim>),+
    }
    
    // Debug formatting (printing units)
    // TODO: maybe implement Display?
    impl<$($dim:NumType<$dim>),+> DimFormat for $name<$($dim),+> {
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
    
    impl<$($dim:NumType<$dim>),+> DimAdd<$name<$($dim),+>> for $name<$($dim),+> { type Out = $name<$($dim),+>; }

    impl<$($dim:NumType<$dim>),+> DimSub<$name<$($dim),+>> for $name<$($dim),+> { type Out = $name<$($dim),+>; }

    // In Mul and Div implementations we abuse $unit for the RHS type parameter name
    
    #[allow(non_camel_case_types)]
    impl<$($dim:NumType<$dim>),+ , $($unit:NumType<$unit>),+> DimMul<$name<$($unit),+>> for $name<$($dim),+>
        where $($dim:TAdd<$dim,$unit>),+ { type Out = $name<$(<$dim as TAdd<$dim,$unit>>::Out),+>; }
    
    #[allow(non_camel_case_types)]    
    impl<$($dim:NumType<$dim>),+ , $($unit:NumType<$unit>),+> DimDiv<$name<$($unit),+>> for $name<$($dim),+>
        where $($dim:TSub<$dim,$unit>),+ { type Out = $name<$(<$dim as TSub<$dim,$unit>>::Out),+>; }
        
    impl<$($dim:NumType<$dim>),+> DimSqrt for $name<$($dim),+>
        where $($dim:THalve<$dim>),+ { type Out = $name<$(<$dim as THalve<$dim>>::Out),+>; }
        
    // type alias and `DimZero` impl for the dimensionless type (all exponents are zero)
    pub type One = $name;
    impl DimZero for One {}
    
    // generate aliases of the form `pub type $dim1 = $name<Pos1, Zero, Zero, ...>`
    __dim_type_alias_helper! { [$name], $($dim),+ -> P1 }
    
    pub mod f64 {
        use std::marker::PhantomData;
        use super::{Dim,One,$($dim),+};
        
        #[allow(non_upper_case_globals, dead_code)]
        pub const one: Dim<One, f64> = Dim {
            amount: 1.0,
            phantom: PhantomData
        };
        
        $(
        #[allow(non_upper_case_globals, dead_code)]
        pub const $unit: Dim<$dim, f64> = Dim {
            amount: 1.0,
            phantom: PhantomData
        };
        )+
    }
    
    pub mod f32 {
        use std::marker::PhantomData;
        use super::{Dim,One,$($dim),+};
        
        #[allow(non_upper_case_globals, dead_code)]
        pub const one: Dim<One, f32> = Dim {
            amount: 1.0,
            phantom: PhantomData
        };
        
        $(
        #[allow(non_upper_case_globals, dead_code)]
        pub const $unit: Dim<$dim, f32> = Dim {
            amount: 1.0,
            phantom: PhantomData
        };
        )+
    }
}}

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