use crate::Borrow;

#[derive(Debug, PartialEq, Eq)]
struct Data(String, usize, bool, Vec<Self>);

#[test]
fn test() {
    // Create a new variable
    let data = Data(String::from("Hello, World"), 100, false, vec![]);
    // Create a Borrow (a reference) with the variable
    let mut data_ref = Borrow::new(data);
    // Create another reference to the same variable
    let mut clone = Borrow::clone(&data_ref);
    // Use the Borrow seemlessly
    data_ref.0.push('!');
    clone.1 += 55;
    data_ref.2 = true;
    clone.3.push(Data("".into(), 0, false, Vec::new()));

    // Debug for JavaCell is same as that for Data
    println!("{:?}", clone);
    // Output
    //Data("Hello, World!", 155, true, [Data("", 0, false, [])])
    println!("{:?}", data_ref);
    // Output
    //Data("Hello, World!", 155, true, [Data("", 0, false, [])])

    assert_eq!(
        *data_ref,
        Data(
            String::from("Hello, World!"),
            155,
            true,
            vec![Data("".into(), 0, false, vec![])]
        )
    );
    assert_eq!(
        *clone,
        Data(
            String::from("Hello, World!"),
            155,
            true,
            vec![Data("".into(), 0, false, vec![])]
        )
    );
    // Borrow implements AsRef and Deref of T
    print(&data_ref);
}

fn print<T: std::fmt::Debug>(data: &T) {
    // do something
    println!("{:?}", data);
}
