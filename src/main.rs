type AgeResult = Result<String, String>;
type AgeError = Result<i32, String>;

fn too_young(age: i32) -> AgeError {
    if age <= 0 {
        Err("too young".to_string())
    } else {
        Ok(age)
    }
}

fn too_old(age: i32) -> AgeError {
    if age > 150 {
        Err("too old".to_string())
    } else {
        Ok(age)
    }
}

fn check_age(age: i32) -> AgeError {
    let age = too_young(age)?;
    let age = too_old(age)?;
    Ok(age)
}

fn age_group(age: i32) -> AgeResult {
    let validated_age = check_age(age)?;


    let result = match validated_age {
        _ if age < 10 => "child".to_string(),
        _ if age >= 18 => "adult".to_string(),
        a => format!("teenager of {} years old", a),
    };

    Ok(result)
}


//can go with panic:
//fn age_group(age: i32) -> AgeResult {
//        if age < 0 || age > 150 {
//    panic!("age is out of range".to_string());
//}


fn main() {
    let age = 15;
    match age_group(age) {
        Ok(description) => println!("{}", description),
        Err(err) => println!("Error: {}", err),
    }

    let zero = 0;
    match age_group(zero) {
        Ok(description) => println!("{}", description),
        Err(err) => println!("Error:{}", err),
    }

}