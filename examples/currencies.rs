#![cfg_attr(feature="unstable", feature(unboxed_closures,core,zero_one))]

#[macro_use]
extern crate units;

mod currencies {
    units! {
        Currency {
            Euro[euro],
            Dollar[dollar]
        }
    }
}

use currencies::Dim;
use currencies::Euro;
use currencies::f64::{euro, dollar};

#[cfg(not(feature = "unstable"))]
fn main() {
    // the conversion factor could also be retrieved from a database
    let dollar_per_euro = 1.1006*dollar/euro;
    
    let a = 2.0*euro;
    let b = a*dollar_per_euro;
    println!("{:?} = {:?}", a, b);
    
    let euro2dollar = |x: Dim<Euro,_>| x * dollar_per_euro;
    println!("{:?} = {:?}", 4.0*euro, euro2dollar(4.0*euro));
    
    let c = 3.0*dollar;
    let d = c/dollar_per_euro;
    println!("{:?} = {:?}", c, d);
}

#[cfg(feature = "unstable")]
fn main() {
    let dollar_per_euro = (dollar/euro)(1.1006);
    
    let a = euro(2.0);
    let b = a*dollar_per_euro;
    println!("{:?} = {:?}", a, b);
    
    let euro2dollar = |x: Dim<Euro,_>| x * dollar_per_euro;
    println!("{:?} = {:?}", euro(4.0), euro2dollar(euro(4.0)));
    
    let c = dollar(3.0);
    let d = c/dollar_per_euro;
    println!("{:?} = {:?}", c, d);
}
