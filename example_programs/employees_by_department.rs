use std::collections::HashMap;

struct Company {
    employees: HashMap<String, Vec<String>>,
}

impl Company {
    fn employees_by_department(&self, department: String) -> Vec<String> {
        let e = self.employees.get(&department);

        match e {
            Some(x) => x.to_vec(),
            None    => vec![],
        }
    }

    fn add_employees_by_department(&mut self, department: String, names: Vec<String>) {
        self.employees.insert(department, names);
    }
}

fn main() {
    let mut my_company = Company { employees: HashMap::new() };
    let department = String::from("engineering");
    let results = my_company.employees_by_department(department.to_string());
    
    println!("Results before any employees added: ");
    for result in results {
        println!("{}", result);
    }

    my_company.add_employees_by_department(
        department.to_string(),
        vec![
            String::from("bradford"),
            String::from("josce"),
            String::from("alex"),
        ]
    );

    let results = my_company.employees_by_department(department.to_string());

    println!("Results after bradford added: ");
    for result in results {
        println!("{}", result);
    }
}

// Using a hash map and vectors, create a text interface to allow a
// user to add employee names to a department in a company. For example,
// “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user
// retrieve a list of all people in a department or all people in the
// company by department, sorted alphabetically.