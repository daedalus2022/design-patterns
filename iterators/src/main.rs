use crate::users::UserCollection;

mod users;

fn main() {
    print!("Iterators are widely used in the statndard library: ");

    let array = &[1, 2, 3];
    let iterator = array.iter();

    iterator.for_each(|e|{print!("{} ", e)});

    println!("\nLet's test our own iterator.");

    let users = UserCollection::new();

    let mut iterator = users.iter();

    println!("1nd element: {:?}", iterator.next());
    println!("2nd element: {:?}", iterator.next());
    println!("3rd element: {:?}", iterator.next());
    println!("4td element: {:?}", iterator.next());

    print!("All elements in user collection:");
    users.iter().for_each(|e|print!("{} ", e));

    println!();
}
