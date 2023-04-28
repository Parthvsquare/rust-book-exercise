use std::collections::HashMap;

pub struct NewCacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> NewCacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> NewCacher<T> {
        NewCacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}
