extern crate units;

use units::si::{one,m,kg,s};

#[cfg(feature = "unstable")]
#[allow(non_snake_case)]
fn main() {
    let N = *units::si::derived::N;
    let cm = *units::si::derived::cm;
    
    let acc = m(2.0)/s/s;
    
    println!("{:?}", (acc*m).sqrt() + (m/s)(3.0));
    
    println!("{:?}", (m*m)(2.0) - m(1.0)*cm(3.0));
    
    println!("{:?}", m(2.0) / cm(100.0));
    
    let x = ((kg(3.0) * acc)/N).sin();
    
    println!("{} N", x);
    
    let x = m;
    let y = acc*s*s;
    //println!("{:?} > {:?}: {}", x, y, x > y);
    
    let min = s(60.0);
    let h = min(60.0);
    let km = m(1000.0);
    let l = (m/s)(2.0);
    println!("{} m/s = {} km/h", *(l/m*s), *(l/km*h));
    
    
    let t_1 = s(0.0);
    let v_1 = m(10.0)/s;
    let t_2 = s(10.0);
    let v_2 = m(30.0)/s;
    let a = (v_2-v_1)/(t_2-t_1);
    println!("{} km/h pro s", *(a/(km/h/s)));
}

#[cfg(not(feature = "unstable"))]
#[allow(non_snake_case)]
fn main() {
    let N = *units::si::derived::N;
    let cm = *units::si::derived::cm;
    
    let Hz = one/s;
    
    let acc = 2.0*m/s/s;
    //let cm = 0.01*m;
    
    println!("{:?}", (acc*m).sqrt() + 3.0*m/s);
    
    println!("{:?}", 2.0*m*m - 1.0*m * 3.0*cm);
    
    println!("{:?}", 2.0*m / 100.0*cm);
    
    let x = ((3.0*kg * acc)/N).sin();
    
    println!("{} N", x);
    
    let x = m;
    let y = acc*s*s;
    //println!("{:?} > {:?}: {}", x, y, x > y);
    
    let min = 60.0*s;
    let h = 60.0*min;
    let km = 1000.0*m;
    let l = 2.0*m/s;
    println!("{} m/s = {} km/h", *(l/m*s), *(l/km*h));
    
    
    let t_1 = 0.0*s;
    let v_1 = 10.0*m/s;
    let t_2 = 10.0*s;
    let v_2 = 30.0*m/s;
    let a = (v_2-v_1)/(t_2-t_1);
    println!("{} km/h pro s", *(a/(km/h/s)));
}