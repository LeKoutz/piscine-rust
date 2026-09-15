pub mod mall;

use std::{collections::HashMap};
pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let all_employees: Vec<(&String, &Employee)> = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.iter())
        .collect();

    let max_salary = all_employees
        .iter()
        .map(|(_, employee)| employee.salary)
        .fold(f64::MIN, f64::max);

    all_employees
        .into_iter()
        .filter(|(_, employee)| employee.salary == max_salary)
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let guards = mall.guards.len();
    let mut employees = 0;
    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            employees += store.employees.len()
        }
    }
    guards + employees
}

pub fn check_for_securities(mall: &mut Mall, available_sec: HashMap<String, Guard>) {
    let mut total_size: u64 = 0;
    for floor in mall.floors.values() {
        total_size += floor.size_limit;
    }

    let total_areas = total_size / 200;
    let unguarded_areas = (total_areas as usize).saturating_sub(mall.guards.len());

    let mut hired = 0;
    for (name, guard) in available_sec {
        if hired >= unguarded_areas {
            break;
        }
        mall.hire_guard(name, guard);
        hired += 1;
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let shift_hours = employee.working_hours.1 - employee.working_hours.0;
                if shift_hours >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}