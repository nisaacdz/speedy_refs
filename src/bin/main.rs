use speedy_refs::bench::MyBencher;

pub const LEN: usize = 30_000_000;

fn main() {
    start()
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
    let bcher = MyBencher::new(2);
    println!("Refcells Bench\n");
    bcher.bench("MINE", || mine());
    bcher.bench("STD", || std());
}
