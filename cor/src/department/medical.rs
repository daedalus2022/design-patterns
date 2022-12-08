use super::{Department, into_next};

pub struct Medical{
    next: Option<Box<dyn Department>>,
}

impl Medical{
    pub fn new(next: impl Department + 'static) -> Self{
        Self{
            next: into_next(next),
        }
    }
}

impl Department for Medical{
    fn handle(&mut self, patient: &mut crate::patient::Patient) {
        if patient.medicine_done{
            println!("Medicine is already given to a patient");
        }else{
            println!("Medicine giving medicine to a patient {}", patient.name);
            patient.medicine_done = true;
        }

    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}