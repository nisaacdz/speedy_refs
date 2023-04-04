use speedy_refs::bench::MyBencher;

pub const LEN: usize = 4_000_000;

fn main() {
    start()
}

fn mine() {
    let rc = speedy_refs::RefCell::new("".to_owned());

    for i in 0..LEN {
        if i & 1 == 0 {
            rc.borrow_mut();
        } else {
            rc.borrow();
        }
    }
}

fn std() {
    let rc = std::cell::RefCell::new("".to_owned());

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
    bcher.bench("MINE", || mine());
    bcher.bench("STD", || std())
}
