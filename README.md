# units [![crates.io](https://img.shields.io/crates/v/units.svg)](https://crates.io/crates/units) [![Build Status](https://travis-ci.org/Boddlnagg/units.svg?branch=master)](https://travis-ci.org/Boddlnagg/units)

**Units of Measure** for Rust. Easy to use, type-safe and customizable.

## Usage (Example)
```rust
#[macro_use] extern crate units;

// It is recommended to put units inside a separate module
pub mod my_units {
    // Here we define our unit system with three units
    units! {
        MyUnits {
            Meter[m],
            Second[s],
            Mile[mile]
        }
    }
}

// Import the unit constants and also the dimensionless unit `one`
use my_units::f64::{one, m, s, mile};

fn main() {
    let km = 1000.*m; // Define the `km` unit as 1000 meter
    let h = 60.*60.*s; // Define an hour as 60 * 60 seconds
    // These units could also have been defined as separate base units,
    // in order to further reduce the risk to mix them up.

    // Let's calculate how far a car has moved if it has accelerated
    // uniformly from 0 to 100 km/h in 12 seconds
    let initial_speed = 0.*km/h;
    let final_speed = 100.*km/h;
    let time = 12.*s;
    let acceleration = (final_speed - initial_speed)/time;
    let result = 0.5 * acceleration * time * time; // s = a/2 * t^2
    // Here we use debug formatting, which will automatically print the base dimensions
    println!("{:?}", result); // This will print `166.66666666666669 m`

    // Let's convert the result to miles
    let meter_per_mile = 1609.344*m/mile; // This has unit `m/mile`
    let result_in_miles = result / meter_per_mile; // This has unit `mile`
    // Here we get a dimensionless value by eliminating the unit, then use deref (*) to extract the raw f64.
    println!("{} miles", *(result_in_miles/mile)); // This will print `0.103561865372889 miles`

    // Now we want to know how long a ball will fall until it reaches the
    // floor if dropped from a height of 1.5 meters
    let height = 1.5*m;
    let g = 9.81*m/s/s; // Use a gravitational constant of 9.81 m/s^2
    let time = (2. * height / g).sqrt(); // sqrt() takes care of the units
    // Print the result in milliseconds and round to 2 digits
    let ms = 0.001*s; // 1 ms = 0.001 s
    println!("The ball falls for {:.2} ms until it hits the ground.", *(time/ms));
    // The above will print `The ball falls for 553.00 ms until it hits the ground.`
}
```
Use `cargo run --example basics` to run this example.

<sup>There is an alternative, safer syntax for assigning units to values using function call operator overloading, but this is currently only available in Rust nightly. See [examples/basics.rs](examples/basics.rs) for both versions.</sup>


## How does it work?
The macro invocation shown above will generate a struct `MyUnits<U,T=f64>`, where the first type parameter is a special marker to get the dimension right (it contains exponents for every base unit, encoded as [type-level integers](http://github.com/Boddlnagg/tylar)). `Meter`, `Second` and `Mile` will be type aliases for this marker type with the correct exponents.

The second type parameter denotes the wrapped numeric type (defaults to `f64`). The autogenerated child modules `my_units::f64` and `my_units::f32` then both contain the constants `m`, `s` and `mile`, each of the correct dimension and wrapping the value `1.0`. For example, `f64::m` is of type `MyUnits<Meter, f64>`.

Additionally, the type `One` and the constant `one` are provided for dimensionless values, and only those can be unwrapped using the `Deref` trait (this works automatically for method calls, but sometimes needs to be made explicit using the `*` prefix operator, see the example above).
