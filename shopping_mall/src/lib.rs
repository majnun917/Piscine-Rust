pub mod mall;
use crate::mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (&String, &Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .expect("mall no store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let max_salary = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.values())
        .map(|employee| employee.salary)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .expect("mall no employees");

    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|(_, store)| {
            store.employees.iter().filter(move |(_, employee)| {
                (employee.salary - max_salary).abs() < f64::EPSILON
            })
        })
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum::<usize>()
        + mall.guards.len()
}
pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let total_size_floor = mall.floors.values().map(|f| f.size_limit).sum::<u64>();
    let guards_required = (total_size_floor / 200) as usize;
    if mall.guards.len() < guards_required {
        let guards_to_hire = guards_required - mall.guards.len();
        let new_guards = guards.into_iter().take(guards_to_hire);
        mall.guards.extend(new_guards);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors
        .values_mut()
        .flat_map(|floor| floor.stores.values_mut())
        .flat_map(|store| store.employees.values_mut())
        .for_each(|employee| {
            if employee.working_hours.1 - employee.working_hours.0 >= 10 {
                employee.salary *= 1.1;
            } else {
                employee.salary *= 0.9;
            }
        });
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use mall::*;
// use shopping_mall::*;

// fn employees(mall: &mut Mall) -> HashMap<String, &mut Employee> {
//     mall.floors
//         .values_mut()
//         .flat_map(|f| {
//             f.stores
//                 .values_mut()
//                 .flat_map(|s| s.employees.iter_mut().map(|(k, v)| (k.clone(), v)))
//         })
//         .collect::<HashMap<_, _>>()
// }


// #[test]
// fn biggest_store_tests() {
//     let mut mall = mall();

//     assert!(matches!(
//         biggest_store(&mall),
//         (
//             name,
//             Store {
//                 square_meters: 950,
//                 ..
//             }
//         ) if name == "Pretail"
//     ));

//     mall.floors
//         .get_mut("Supermarket")
//         .unwrap()
//         .remove_store("Pretail");

//     assert!(matches!(
//         biggest_store(&mall),
//         (
//             name,
//             Store {
//                 square_meters: 60,
//                 ..
//             }
//         ) if name == "PizBite"
//     ));
// }

// #[test]
// fn highest_paid_test() {
//     let mut mall = mall();

//     let highest_paid = highest_paid_employee(&mall);

//     assert!(matches!(
//         highest_paid[..],
//         [(name, Employee { age: 54, .. })] if name == "Abdallah Stafford"
//     ));

//     let highest_salary = highest_paid[0].1.salary;

//     let mut employees = employees(&mut mall);

//     let another_employee = employees.get_mut("Finbar Haines").unwrap();

//     another_employee.raise(highest_salary - another_employee.salary);

//     let highest_paid = highest_paid_employee(&mall);

//     assert_eq!(2, highest_paid.len());
//     assert!(highest_paid
//         .windows(2)
//         .all(|w| w[0] != w[1] && w[0].1.salary == w[1].1.salary));
//     assert!(highest_paid.into_iter().all(
//         |v| matches!(v, (n, Employee { age: 54, .. }) if n == "Abdallah Stafford")
//             | matches!(v, (n, Employee { age: 36, .. }) if n == "Finbar Haines")
//     ));
// }

// #[test]
// fn nbr_of_employees_test() {
//     let mut mall = mall();

//     assert_eq!(36, nbr_of_employees(&mall));

//     mall.floors
//         .get_mut("Supermarket")
//         .unwrap()
//         .stores
//         .get_mut("Pretail")
//         .unwrap()
//         .employees
//         .drain();

//     assert_eq!(22, nbr_of_employees(&mall));
// }

// #[test]
// fn check_for_securities_test() {
//     let mut mall = mall();

//     assert_eq!(3, mall.guards.len());

//     check_for_securities(
//         &mut mall,
//         [
//             (
//                 "Peter Solomons",
//                 Guard {
//                     age: 45,
//                     years_experience: 20,
//                 },
//             ),
//             (
//                 "William Charles",
//                 Guard {
//                     age: 32,
//                     years_experience: 10,
//                 },
//             ),
//             (
//                 "Leonardo Changretta",
//                 Guard {
//                     age: 23,
//                     years_experience: 0,
//                 },
//             ),
//             (
//                 "Vlad Levi",
//                 Guard {
//                     age: 38,
//                     years_experience: 8,
//                 },
//             ),
//             (
//                 "Faruk Berkai",
//                 Guard {
//                     age: 40,
//                     years_experience: 15,
//                 },
//             ),
//             (
//                 "Chritopher Smith",
//                 Guard {
//                     age: 35,
//                     years_experience: 9,
//                 },
//             ),
//             (
//                 "Jason Mackie",
//                 Guard {
//                     age: 26,
//                     years_experience: 2,
//                 },
//             ),
//             (
//                 "Kenzie Mair",
//                 Guard {
//                     age: 34,
//                     years_experience: 8,
//                 },
//             ),
//             (
//                 "Bentley Larson",
//                 Guard {
//                     age: 33,
//                     years_experience: 10,
//                 },
//             ),
//             (
//                 "Ray Storey",
//                 Guard {
//                     age: 37,
//                     years_experience: 12,
//                 },
//             ),
//         ]
//         .map(|(n, d)| (n.to_owned(), d))
//         .into(),
//     );

//     assert_eq!(9, mall.guards.len());
// }

// #[test]
// fn cut_or_raise_test() {
//     let mut mall = mall();

//     cut_or_raise(&mut mall);

//     {
//         let employees = employees(&mut mall);

//         assert_eq!(585.792, employees.get("Finbar Haines").unwrap().salary);
//         assert_eq!(1100.473, employees.get("Sienna-Rose Penn").unwrap().salary);
//     }

//     cut_or_raise(&mut mall);

//     {
//         let employees = employees(&mut mall);

//         assert_eq!(527.2128, employees.get("Finbar Haines").unwrap().salary);
//         assert_eq!(1210.5203, employees.get("Sienna-Rose Penn").unwrap().salary);
//     }
// }
// }
