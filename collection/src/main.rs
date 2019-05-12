use std::collections::HashMap;
use std::cmp::Ordering;
use std::io;

fn main() {
    company_tui();
}

enum Action {
    Add {name: String, department: String},
    List {department: String},
    ListAll,
}

fn company_tui() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("unable to read input");
        println!("input: {}", input);

        let mut word_iter = input.trim().split_whitespace();
        let action = word_iter.next().unwrap_or_default();
        let action = match action {
            "Add" => {
                let name = word_iter.next().unwrap();
                let to = word_iter.next().unwrap();
                if to != "to" {
                    println!("error syntax");
                    continue;
                }
                let department = word_iter.next().unwrap();
                Action::Add {name: name.to_string(), department: department.to_string()}
            },
            "List" => {
                let param = word_iter.next();
                match param {
                    Some(department) =>  Action::List {department: department.to_string()},
                    None => Action::ListAll,
                }
            },
            unknown => {
                println!("unknown command: {}", unknown);
                continue
            }
        };
        match action {
            Action::Add {name, department} => {
                let employees = map.entry(department.clone()).or_insert(Vec::new());
                employees.push(name);
                println!("After Add map is: {:?}", map);
            },
            Action::List {department} => {
                println!("List {}: {:?}", department, map.get(&department));
            },
            Action::ListAll => {
                for (dep, employees) in map.iter() {
                    let mut employees_sorted = employees.clone();
                    employees_sorted.sort();
                    println!("department: {}, employees: {:?} employees_sorted: {:?}", dep, employees, employees_sorted);
                }
            }
        }
    }
}

fn experimentation() {
    let v = vec![1,2,3,4,5];
    let first = &v[0];
    // v.push(6);
    for i in &v {
        println!("The element is: {}", i);
    }
    let mut vm = v;
    for i in &mut vm {
        *i += 10;
        println!("The element is: {}", i);
    }

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let mut s = "initial contents".to_string();
    s.push_str(" and next contents");
    s.push('o');

    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s = s1 + &s2;
    println!("{}",s);

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");
    let ss = format!("{}-{}-{}", ss1,ss2,ss3);
    println!("{}",ss);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    scores.insert(blue, 10);
    scores.insert(String::from("Yellow"), 50);

    // println!("{}", blue);
    let yellow = String::from("Yellow");
    let value = scores.get(&yellow);
    println!("value is: {:?}", value);
    for (k,v) in scores.iter() {
        println!("k: {:?} v: {:?}", k, v);
    }

    scores.entry(String::from("Yellow")).or_insert(25);
    println!("or_insert is : {:?}", scores.get(&yellow));
    println!("scores: {:?}", scores);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("insert_and_modify is: {:?}", map);

}

fn list_of_integers() {
    let integers = [3, 5, 2, 6, 5];
    let sum: i32 = integers.iter().sum();
    let mean :f64 = sum as f64 / integers.len() as f64;
    println!("sum: {} mean: {}", sum, mean);

    let mut sorted_integers = integers;
    sorted_integers.sort();
    let half_index = (sorted_integers.len() - 1) / 2;
    let median = sorted_integers[half_index];
    println!("sorted_integers: {:?} median: {}", sorted_integers, median);

    let mut map :HashMap<i32,i32> = HashMap::new();
    for i in integers.iter() {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let max = map.iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(&v2))
        .map(|(k,v_)| k);
    println!("max: {:?}", max);
}