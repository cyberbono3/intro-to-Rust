
// we can implement traits for any type 
// in any OOP programming you can not extend i32 type
impl Animal for i32 { 
    fn eat(&mut self, _food: Food) { 
    *self = *self + 1 
    } 
}