use std::{cell::RefCell, collections::HashMap, io};

fn main() {
    let v = vec![2, 2, 3, 4, 5, 6, 7, 1, 3, 2, 1];
    let mut ordered_v = v.clone();
    ordered_v.sort();

    // Exercise 1
    // Given a list of integers, use a vector and return the median 
    // (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    println!("Exercise 1");
    println!("List: {:?}", v);
    println!("Ordered List: {:?}", ordered_v);
    println!("Median: {}", median(&v));
    println!("Mode: {}", mode(&v));
    println!("\n");

    //Exercise 2
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
    // Keep in mind the details about UTF-8 encoding!

    println!("Exercise 2");
    let word_array = vec!["Hello", "Testing", "Ola", "Island", "Based"]; 
    println!("Word Array: {:?}", word_array);
    println!("Pigged Array: {:?}", pig_latin(&word_array));
    println!("\n");

    //Exercise 3
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department 
    // or all people in the company by department, sorted alphabetically.

    println!("Exercise 3");
    company_department();

}

fn median(list: &Vec<i32>) -> f32{
    let mut sorted_list = list.clone();
    sorted_list.sort();
    let count = sorted_list.len();
    let median: f32;
    if count % 2 == 0 {
        let first_half_option = sorted_list.get((count / 2)-1);
        let second_half_option = sorted_list.get(count / 2);
        let first_half: i32;
        let second_half: i32;
        match first_half_option {
            Some(found_median) => {
                first_half = *found_median; 
            }
            None => {
                first_half = 0;
            }
        }
        match second_half_option {
            Some(found_median) => {
                second_half = *found_median; 
            }
            None => {
                second_half = 0;
            }
        }
        median = (first_half as f32 + second_half as f32) / 2.0
    } else {
        let median_option = sorted_list.get(count / 2);
        match median_option {
            Some(found_median) => {
                median = *found_median as f32; 
            }
            None => {
                median = 0.0;
            }
        }
    }
    median
}


fn mode(list: &Vec<i32>) -> i32{
    
    let mut map = HashMap::new();

    for number in list {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }


    struct Mode {
        highest: i32,
        count: i32
    }

    let mut mode = Mode {
        highest: 0,
        count: 0
    };


    for (key, value) in &map {
        if*value > mode.count{
            mode.highest = **key;
            mode.count = *value;
        }
    }

    mode.highest
}

fn pig_latin(strings: &Vec<&str>) -> Vec<String>{
    let vowels = ['a','e','i','o','u', 'A', 'E', 'I', 'O', 'U'];
    let mut pigged_strings: Vec<String> = Vec::new();

    for item in strings {
        let first_char = item[0..1].chars().next().expect("Expected a char"); 
        if !vowels.contains(&first_char){
            let mut pigged_string = String::new();
            let second_half = &item[1..item.len()];
            pigged_string.push_str(second_half);
            pigged_string.push_str(first_char.to_string().as_str());
            pigged_string.push_str("-ay");
            pigged_strings.push(pigged_string);
        }else{
            let mut pigged_string = String::new();
            pigged_string.push_str(*item);
            pigged_string.push_str("-ay");
            pigged_strings.push(pigged_string);
        }
    }
    pigged_strings

}


fn company_department() {
    let mut guess = String::new();
    let mut departments_and_employees: HashMap<String, RefCell<Vec<String>>> = HashMap::new();

    loop{
        println!("Hello user, what do you wish to do?");
        println!("1-Create a Department");
        println!("2-Add employeee to department");
        println!("3-Retrieve all people in department");
        println!("4-Retrieve all people in the company by department alphabetically");
        println!("5-Leave");

        // println!("Departments: {:?}",departments_and_employees);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read");

        match guess.as_str().trim()  {
            "1" => create_department(&mut departments_and_employees),
            "2" => add_employee_to_department(&mut departments_and_employees),
            "3" => retrieve_people_in_department(&mut departments_and_employees),
            "4" => retrieve_people_in_company_alphabetically(&mut departments_and_employees),
            "5" => break,
            _ => {}
        }

        guess = String::from("");
        print!("{}[2J", 27 as char);
    }
}

fn create_department(departments_and_employees: &mut HashMap<String, RefCell<Vec<String>>>){
    print!("{}[2J", 27 as char);
    println!("What's gonna be the name of the department?");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to Read");
    department_name = String::from(department_name.trim());

    departments_and_employees.insert(department_name, RefCell::new(vec![]));

    println!("Department inserted!");

    press_enter();
}

fn add_employee_to_department(departments_and_employees: &mut HashMap<String, RefCell<Vec<String>>>){
    print!("{}[2J", 27 as char);
    let department_option =  get_department_by_user(departments_and_employees);

    match department_option {
        Option::None => {
            println!("Department does not exist!");
            press_enter();
            return;
        },
        Option::Some(registered_employees) => {
            println!("Tell the name of the employee:");

            let mut employee = String::new();

            io::stdin()
                .read_line(&mut employee)
                .expect("Failed to Read");

            employee = String::from(employee.trim());

            let mut vector = registered_employees.borrow_mut();
            vector.push(employee);


            println!("Employee inserted!");

            press_enter();
        }   
    }

}

fn retrieve_people_in_department(departments_and_employees: &mut HashMap<String, RefCell<Vec<String>>>){
    print!("{}[2J", 27 as char);
    let department_option =  get_department_by_user(departments_and_employees);

    match department_option {
        Option::None => {
            println!("Department does not exist!");
            press_enter();
        },
        Option::Some(registered_employees) => {
            let vector = registered_employees.borrow();

            println!("The Employess of this department are: ");
            for element in vector.iter() {
                println!("-{}", element);
            }

            press_enter();
        }   
    }
}   

fn retrieve_people_in_company_alphabetically(departments_and_employees: &mut HashMap<String, RefCell<Vec<String>>>){
    print!("{}[2J", 27 as char);
    for (key, value) in departments_and_employees{
        println!("*Department: {}", key);
        let mut vector = value.borrow_mut();
        vector.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        for element in vector.iter(){
            println!(" -{}", element);
        }

    }

    press_enter();
}   

fn get_department_by_user(departments_and_employees: &mut HashMap<String, RefCell<Vec<String>>>) -> Option<&RefCell<Vec<String>>>{
    println!("Tell the name of the department:");

    let mut department_name = String::new();
    
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to Read");

    department_name = String::from(department_name.trim());

    let department_option =  departments_and_employees.get(&department_name);
    department_option
}

fn press_enter(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
