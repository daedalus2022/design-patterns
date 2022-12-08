use department::{cashier::Cashier, medical::Medical, doctor::{self, Doctor}, reception::{self, Reception}, Department};
use patient::Patient;

mod patient;
mod department;


///
/// Conceptual Example
/// The example demonstrates processing a patient through a chain of departments. The chain of responsibility is constructed as follows:
/// > Patient -> Reception -> Doctor -> Medical -> Cashier
/// The chain is constructed using Box pointers, which means dynamic dispatch in runtime. Why? It seems quite difficult to narrow down implementation to a strict compile-time typing using generics: in order to construct a type of a full chain Rust needs full knowledge of the "next of the next" link in the chain. Thus, it would look like this:
/// 
/// 

fn main() {
    let cashier = Cashier::default();

    let medical = Medical::new(cashier);

    let doctor = Doctor::new(medical);

    let mut reception = Reception::new(doctor);

    let mut patient = Patient{
        name: "John".into(),
        ..Patient::default()
    };

    reception.execute(&mut patient);

    println!("\n The patient has been already handled:\n");

    reception.execute(&mut patient);
}
