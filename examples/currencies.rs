#![cfg_attr(feature="unstable", feature(unboxed_closures,core))]

#[macro_use]
extern crate units;

mod currencies {
    units! {
        Currency {
            CurrencyEurope => Euro[euro],
            CurrencyUS => Dollar[dollar]
        }
    }
    
    #[cfg(not(feature = "unstable"))]
    pub fn euro_to_dollar(a: Euro) -> Dollar {
        // the conversion factor must not be hard-coded,
        // but could be retrieved from a database
        let dollar_per_euro = 1.1006*dollar/euro;
        a * dollar_per_euro
    }
    
    #[cfg(feature = "unstable")]
    pub fn euro_to_dollar(a: Euro) -> Dollar {
        // the conversion factor must not be hard-coded,
        // but could be retrieved from a database
        let dollar_per_euro = (dollar/euro)(1.1006);
        a * dollar_per_euro
    }
}

use currencies::{euro, dollar};

#[cfg(not(feature = "unstable"))]
fn main() {
    let dollar_per_euro = 1.1006*dollar/euro;
    let a = 2.0*euro;
    
    let b = a*dollar_per_euro;
    println!("{:?} = {:?}", a, b);
    
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
    
    let c = dollar(3.0);
    let d = c/dollar_per_euro;
    println!("{:?} = {:?}", c, d);
}
