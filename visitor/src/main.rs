use visitor::Visitor;

mod visitor;

#[derive(Default, Debug)]
pub struct TwoValuesStruct{
    a: i32,
    b: i32,
}

#[derive(Default, Debug)]
pub struct TwoValuesArray{
    ab: [i32; 2],
}

trait Deserializer<V: Visitor>{
    fn create(visitor: V)->Self;

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str>{
        Err("parse_str is unimplemented")
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str>{
        Err("parse_vec is unimplemented")
    }
}

struct StringDeserializer<V: Visitor>{
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V>{
    fn create(visitor: V)->Self {
        Self{visitor}
    }

    fn parse_str(&self, input: &str) -> Result<<V as Visitor>::Value, &'static str> {
        let input_vec = input.split_ascii_whitespace()
        .map(|x|x.parse().unwrap())
        .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}

struct VecDeserializer<V: Visitor>{
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V>{
    fn create(visitor: V)->Self{
        Self{visitor}
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<<V as Visitor>::Value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}

fn main() {
    let deserializer = StringDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValuesArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    println!(
        "Error: {}",
        deserializer.parse_str("123 456").err().unwrap()
    )
}
