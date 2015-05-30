#![cfg_attr(feature="unstable", feature(unboxed_closures, core, zero_one))]

#[macro_use] extern crate units;

// It is recommended to put units inside a separate module
pub mod my_units {
    units! {
        // Here we define our unit system with three units
        MyUnits {
            Meter[m],
            Second[s],
            Mile[mile]
        }
    }
}

// Import the unit constants and also the dimensionless unit `one`
use my_units::f64::{one, m, s, mile};

#[cfg(not(feature = "unstable"))]
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

#[cfg(feature = "unstable")]
fn main() {
    let km = m(1000.); // Define the `km` unit as 1000 meter
    let h = s(60. * 60.); // Define an hour as 60 * 60 seconds
    // These units could also have been defined as separate base units,
    // in order to further reduce the risk to mix them up.
    
    // Let's calculate how far a car has moved if it has accelerated
    // uniformely from 0 to 100 km/h in 12 seconds
    let initial_speed = (km/h)(0.);
    let final_speed = (km/h)(100.);
    let time = s(12.);
    let acceleration = (final_speed - initial_speed)/time;
    let result = one(0.5) * acceleration * time * time; // s = a/2 * t^2
    // Here we use debug formatting, which will automatically print the base dimensions
    println!("{:?}", result); // This will print `166.66666666666669 m`
    
    // Let's convert the result to miles
    let meter_per_mile = (m/mile)(1609.344); // This has unit `m/mile`
    let result_in_miles = result / meter_per_mile; // This has unit `mile`
    // Here we get a dimensionless value by eliminating the unit, then use deref (*) to extract the raw f64.
    println!("{} miles", *(result_in_miles/mile)); // This will print `0.103561865372889 miles`
    
    // Now we want to know how long a ball will fall until it reaches the
    // floor if dropped from a height of 1.5 meters
    let height = m(1.5);
    let g = (m/s/s)(9.81); // Use a gravitational constant of 9.81 m/s^2
    let time = (one(2.) * height / g).sqrt(); // sqrt() takes care of the units
    // Print the result in milliseconds and round to 2 digits
    let ms = s(0.001); // 1 ms = 0.001 s
    println!("The ball falls for {:.2} ms until it hits the ground.", *(time/ms));
    // The above will print `The ball falls for 553.00 ms until it hits the ground.`
}