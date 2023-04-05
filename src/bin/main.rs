use speedy_refs::bench::MyBencher;

pub const LEN: usize = 90_000_000;

fn main() {
    start()
}

fn clone<T: Clone>(mut val: T) {
    for _ in 0..LEN {
        val = val.clone();
    }
}

fn mine() {
    let rc = speedy_refs::RefCell::new(1);

    for i in 0..LEN {
        if i & 1 == 0 {
            rc.borrow_mut();
        } else {
            rc.borrow();
        }
    }
}

fn std() {
    let rc = std::cell::RefCell::new(1);
    for i in 0..LEN {
        if i & 1 == 0 {
            rc.borrow_mut();
        } else {
            rc.borrow();
        }
    }
}

pub fn start() {
    let bcher = MyBencher::new(3);
    println!("Refcells Bench");
    bcher.bench("MINE", || mine());
    bcher.bench("STD", || std());
    println!("\nRc's Bench");
    bcher.bench("MINE", || clone(speedy_refs::Rc::new(5)));
    bcher.bench("STD", || clone(std::rc::Rc::new(5)));
}
