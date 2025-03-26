//  Functions without return

pub fn some_function () {
    println!("\nThis is printed by a statement inside the function some_function in lib.rs source code!");
}

pub fn print_value (value: i32) {
    println!("\nThe value of the argument given to this function call is {value}.");
}

pub fn print_is_true (integer: i32, boolean: bool) {
    if (integer != 0) == boolean {
        println!("\nThe value {integer} is equal to {boolean}.");
    }   else {
        println!("\nThe value {integer} is not equal to {boolean}.");
    }
}

//  Functions with return

pub fn five () -> i32 {
    return 5;
}

pub fn plus_one (value: i32) -> i32 {
    return value + 1;
}

//  Functions for functions_demos
pub fn celsius_to_kelvin (temp: f64) -> f64 {
    return temp + 273.15;
}

pub fn kelvin_to_celsius (temp: f64) -> f64 {
    return temp - 273.15;
}

pub fn celsius_to_fahrenheit (temp: f64) -> f64 {
    return 9.0 * temp / 5.0 + 32.0;
}

pub fn fahrenheit_to_celsius (temp: f64) -> f64 {
    return 5.0 * (temp - 32.0) / 9.0;
}

pub fn kelvin_to_fahrenheit (temp: f64) -> f64 {
    return 9.0 * (temp - 273.15) / 5.0 + 32.0;
}

pub fn fahrenheit_to_kelvin (temp: f64) -> f64 {
    return 5.0 * (temp - 32.0) / 9.0 + 273.15;
}

pub fn is_even (value: i64) -> bool {
    return value % 2 == 0;
}

pub fn is_odd (value: i64) -> bool {
    return !(value % 2 == 0);
}

pub fn add_ints (left: i128, right: i128) -> i128 {
    return left + right;
}

pub fn add_floats (left: f64, right: f64) -> f64 {
    return left + right;
}

pub fn sub_ints (left: i128, right: i128) -> i128 {
    return left - right;
}

pub fn sub_floats (left: f64, right: f64) -> f64 {
    return left - right;
}

pub fn mul_ints (left: i128, right: i128) -> i128 {
    return left * right;
}

pub fn mul_floats (left: f64, right: f64) -> f64 {
    return left * right;
}

pub fn div_ints (left: i128, right: i128) -> i128 {
    return left / right;
}

pub fn div_floats (left: f64, right: f64) -> f64 {
    return left / right;
}

pub fn remainder_ints (left: i128, right: i128) -> i128 {
    return left % right;
}

pub fn remainder_floats (left: f64, right: f64) -> f64 {
    return left % right;
}

pub fn power_ints (base: i128, expo: i128) -> i128 {
    if expo < 0 {
        return 0;
    }   else if expo == 0 {
        return 1;
    }   else if expo == 1 {
        return base;
    }   else {
        let mut counter: i128 = 2;
        let mut result: i128 = base;

        while counter <= expo {
            result = result * base;
            counter += 1;
        }

        return result;
    }
}

pub fn power_floats (base: f64, expo: i128) -> f64 {
    if expo == -1 {
        return 1.0 / base;
    }   else if expo == 0 {
        return 1.0;
    }   else if expo == 1 {
        return base;
    }   else {
        let mut counter: i128;
        let mut result: f64;

        result = base;

        if expo < -1 {
            counter = -2;

            while counter >= expo {
                result = result * base;
                counter -= 1;
            }

            return 1.0 / result;
        }   else {
            counter = 2;

            while counter <= expo {
                result = result * base;
                counter += 1;
            }

            return result;
        }
    }
}

pub fn gcd (mut first: i128, mut second: i128) -> i128 {
    let mut remainder: i128;

    while first % second > 0 {
        remainder = first % second;
        first = second;
        second = remainder;
    }

    return second;
}

pub fn lcm (first: i128, second: i128) -> i128 {
    return first / gcd(first, second) * second;
}

pub fn body_mass_index (weight: f64, height: f64) -> f64 {
    return weight / power_floats(height, 2);
}

pub fn factorial (value: i32) -> i128 {
    if value >= 0 && value <= 1 {
        return 1;
    }   else {
        let mut result: i128 = 2;
        let mut counter: i32 = 3;

        while counter <= value {
            result = result * counter as i128;
            counter = counter + 1;
        }

        return result;
    }
}
