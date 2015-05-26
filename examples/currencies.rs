#![cfg_attr(feature="unstable", feature(unboxed_closures,core,zero_one))]

#[macro_use]
extern crate units;

mod currencies {
    units! {
        Currency {
            Euro[eur],
            USDollar[usd]
        }
    }
}

use currencies::{Currency, Euro};
use currencies::f64::{eur, usd};

#[cfg(not(feature = "unstable"))]
fn main() {
    // the conversion factor could also be retrieved from a database
    let usd_per_eur = 1.1006*usd/eur;
    
    let a = 2.0*eur;
    let b = a*usd_per_eur;
    println!("{:?} = {:?}", a, b);
    
    let eur2usd = |x: Currency<Euro,_>| x * usd_per_eur;
    println!("{:?} = {:?}", 4.0*eur, eur2usd(4.0*eur));
    
    let c = 3.0*usd;
    let d = c/usd_per_eur;
    println!("{:?} = {:?}", c, d);
}

#[cfg(feature = "unstable")]
fn main() {
    let usd_per_eur = (usd/eur)(1.1006);
    
    let a = eur(2.0);
    let b = a*usd_per_eur;
    println!("{:?} = {:?}", a, b);
    
    let eur2usd = |x: Currency<Euro,_>| x * usd_per_eur;
    println!("{:?} = {:?}", eur(4.0), eur2usd(eur(4.0)));
    
    let c = usd(3.0);
    let d = c/usd_per_eur;
    println!("{:?} = {:?}", c, d);
}
