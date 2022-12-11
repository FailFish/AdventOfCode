use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fs,
    io::{self, BufRead, BufReader, Error},
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum Item {
    Dir(Dir),
    File(File),
}
impl Item {
    fn get_mut_dir(&mut self) -> Option<&mut Dir> {
        match self {
            Self::Dir(d) => Some(d),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Dir {
    name: String,
    items: Vec<Rc<RefCell<Item>>>,
    parent: Option<Weak<RefCell<Item>>>,
}
#[derive(Debug)]
struct File {
    size: usize,
    name: String,
    parent: Option<Weak<RefCell<Item>>>,
}

fn main() -> Result<(), Error> {
    let file = fs::File::open("input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let root = Rc::new(RefCell::new(Item::Dir(Dir {
        name: String::from("/"),
        items: vec![],
        parent: None,
    })));
    let mut cwd = Rc::downgrade(&root);
    for line in lines {
        if line.starts_with("$") {
            let v: Vec<&str> = line.split_terminator(' ').collect();
            // assert!(v.len() == 3, "incorrect command usage {v:?}");
            match v[1] {
                "cd" => {
                    cwd = match v[2] {
                        ".." => match &*cwd.upgrade().unwrap().borrow() {
                            Item::Dir(d) => Weak::clone(d.parent.as_ref().unwrap()),
                            _ => panic!("supposed to be directory"),
                        },
                        "/" => Rc::downgrade(&root),
                        _ => {
                            if let Item::Dir(d) = &*cwd.upgrade().unwrap().borrow() {
                                d.items
                                    .iter()
                                    .find_map(|i| match &*i.borrow() {
                                        Item::Dir(d) => {
                                            if d.name == v[2] {
                                                Some(Rc::downgrade(&i))
                                            } else {
                                                None
                                            }
                                        }
                                        Item::File(_) => None,
                                    })
                                    .unwrap()
                            } else {
                                panic!("supposed to be directory")
                            }
                        }
                    };
                }
                "ls" => continue,
                _ => panic!("command cannot be recognized"),
            }
        } else {
            let v: Vec<&str> = line.split_terminator(' ').collect();
            assert!(v.len() == 2, "incorrect output");
            let name = v[1].to_string();
            let ls_dir = &*cwd.upgrade().unwrap();
            let mut ls_dir = ls_dir.borrow_mut();
            let ls_dir = ls_dir.get_mut_dir().unwrap();
            // let ls_dir = match *ls_dir {
            //     Item::Dir(ref mut d) => Some(d),
            //     _ => None,
            // }.unwrap();

            // let ls_dir = ls_dir.borrow_mut().get_mut_dir().unwrap();
            if let Ok(size) = v[0].parse::<usize>() {
                let new_file = File {
                    size,
                    name,
                    parent: Some(Weak::clone(&cwd)),
                };
                ls_dir
                    .items
                    .push(Rc::new(RefCell::new(Item::File(new_file))));
            } else if v[0] == "dir" {
                let new_dir = Dir {
                    name,
                    items: vec![],
                    parent: Some(Weak::clone(&cwd)),
                };
                ls_dir.items.push(Rc::new(RefCell::new(Item::Dir(new_dir))));
            }
        }
    }

    let mut sizes = Vec::new();
    let total = traverse(&root, &mut sizes);
    let freespace = 70000000 - total;
    println!("{}, {}", total, freespace);
    // sizes.sort();
    // println!("{sizes:?}");
    let answer1 = sizes
        .iter()
        .fold(0, |accu, &i| if i < 100000 { accu + i } else { accu });

    let answer2 = sizes
        .into_iter()
        .filter(|&i| i + freespace >= 30000000).min().unwrap();
        // .reduce(|accu, i| if i >= minimum { std::cmp::min(accu, i) } else { accu }).unwrap();

    println!("{answer1}, {answer2}");
    Ok(())
}

fn traverse(root: &RefCell<Item>, sizes: &mut Vec<usize>) -> usize {
    match &*root.borrow() {
        Item::Dir(d) => {
            let mut sum = 0;
            for i in d.items.iter() {
                sum += traverse(&i, sizes);
            }
            sizes.push(sum);
            sum
        }
        Item::File(f) => f.size,
    }
}
