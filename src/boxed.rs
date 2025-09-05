use std::any::Any;

impl <T: 'static> Boxed for T {}

pub trait Boxed: Sized + 'static {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    fn boxed_any(self) -> Box<dyn Any> {
        self.boxed() as Box<dyn Any>
    }
}