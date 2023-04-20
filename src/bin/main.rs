use speedy_refs::Cell;

fn main() {
    #[derive(Debug, PartialEq, Eq)]
    struct Data(String, usize, bool, Vec<Self>);
    let data = Data(String::from("Hello, World"), 100, false, vec![]);
    let mut cell = Cell::new(data);
    let mut clone = Cell::clone(&cell);

    cell.0.push('!');
    clone.1 += 55;
    cell.2 = true;
    clone.3.push(Data("".into(), 0, false, Vec::new()));

    // Debug for JavaCell is same as that for Data
    println!("{:?}", clone);
    // Output
    //Data("Hello, World!", 155, true, [Data("", 0, false, [])])
    println!("{:?}", cell);
    // Output
    //Data("Hello, World!", 155, true, [Data("", 0, false, [])])
    

    assert_eq!(*cell, Data(String::from("Hello, World!"), 155, true, vec![Data("".into(), 0, false, vec![])]));
    assert_eq!(*clone, Data(String::from("Hello, World!"), 155, true, vec![Data("".into(), 0, false, vec![])]));
}
