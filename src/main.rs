#[allow(unused_variables)]
fn main() {
    //let location: [f32;2] = [0.0, 0.0]; // initialization
    let mut big_array = [0.0; 10000]; // default value 10k times

    for i in 0..9999 {
        big_array[i] += 1.0;
        print!("is {}", i);
    }

    // for i in 0..99999 {
    //     let result: i64 = i * (i + i);
    //     println!("{}", result);
    // }

    let location_array: [f64; 2]; // initialization
    location_array = [41.4094069, -81.8546911];

    // accessing indexes on array
    println!("{}/{}", location_array[0], location_array[1]);

    // a tuple
    let location = ("KCLE", 41.4094069, -81.8546911);

    // accessing indexes on on tuple
    println!("{}: {}/{}", location.0, location.1, location.2);

    let (name, lat, lon) = location;

    println!("newline \n {} {} {}", name.to_lowercase(), lat, lon);

    //strings and string slices
    let person_slice = "Berkc Tezc";
    let person_string = person_slice.to_string();
    let person_string = "Berkc Tezc".to_string();
    let person_string = String::from(person_slice);

    let person_slice_again = &person_string;
    let person_slice_again = person_string.as_str();

    // concatenation
    let duck = "Duck";
    let airlines = "Airlines";
    let airline_name = format!("{} {}", duck, airlines);
    println!("{}", airline_name);

    let mut band_name = String::new();
    band_name.push_str("Rage");
    band_name.push(' ');
    band_name.push_str("Against");
    band_name.push(' ');
    band_name.push_str("The Machine");
    band_name += "!!";

    println!("{}", band_name);

    let scope_test = "outer scope";
    println!("1-{}", scope_test);
    // a scope
    {
        // shadowing
        let scope_test = "inner scope";
        println!("2-{}", scope_test);
    }
    println!("3-{}", scope_test);

    let unused_variable = "test";
    let _unused_variable_with_prefix = "test2";
    println!("Hello, world!");
}
