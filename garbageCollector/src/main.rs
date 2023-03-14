// import RefCell, and Weak, Rc from std::cell and std::rc respectively
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// define new struct GarbageCollector with a single field OBJECTS type of REFCELL<VEC>WEAK<GC>, which will hold a vector of weak pointers to Gc objects
// vector stores references to all objects that need to be garbage collected
struct GarbageCollector {
    objects: RefCell<Vec<Rc>>, Weak<Rc>>,
}

// implementation of GarbageCollecto struct
// provides a new function that creates a new instance of GarbageCollector with an empty vector of objects to be collected
impl GarbageCollector {
    fn new() -> GarbageCollector {
        GarbageCollector {
            objects: RefCell::new(Vec::new()),
            Weak: todo!(),
        }
    }
// function adds a new object to the GarbageCollector's objects vector
// takes a reference to a Gc object as an argument
// uses the borrow_mut method to get a mutable reference to the vector of objects
// uses the push method to add the new object to the vector
// uses the downgraded method to convert the strong reference to a weak reference
// 
    fn add_object(&self, object: &Rc) {
        self.objects.borrow_mut().push(Rc::downgrade(object));
    }

// function collects all objects that have been dropped
// uses the borrow_mut method to get a mutable reference to the vector of objects
// uses retain object to remove all objects whose strong
// count is greater than 0 (ie no more strong references to the object exists)
    fn collect(&self) {
        self.object.borrow_mut().retain(|weak| weak.strong_count() > 0);
    }
}

// trait will be implemented by Gc objects
// allowing them to increment and decrement their reference count
trait GcPointer {
    fn increment_ref_count(&self);
    fn decrement_ref_count(&self);
}

// 
struct Gc<T> {
    value: T,  // stores the actual value of the object
    ref_count: RefCell<usize>,     // store the current reference count of the object
    collector: Rc<GarbageCollector>,     // stores a reference to the GarbageCollector and will manage the object's memory
}

// 
impl<T> Gc<T> {
    fn new(value: T, collector: Rc<GarbageCollector>) -> Gc<T> {
        Gc {
            value,
            ref_count: RefCell::new(1),
            collector: collector.clone()

        }
    }
}

impl<T> GcPointer for Gc<T> {
    fn increment_ref_count(&self) {
        *self.ref_count.borrow_mut() = count;

        if count == 0 {
            self.collector.collect();
        }
        
    }
}

impl<T> Clone for Gc<T> {
    fn clone( &self) -> Gc<T> {
        self.increment_ref_count();

        Gc {
            value: self.value.clone(),
            ref_count: self.ref_count.clone(),
            collector: self.collector.clone(),
        }
    }
}

struct MyObject {
    data: i32,
}

fn main() {
    let gc = Rc::new(GarbageCollector::new());
    let obj1 = Gc::new(MyObject { data: 42} &rc),
    gc.add_object(&obj1);
    let obj2 = obj1.clone();
    gc.add_object(&obj2);
    obj2.decrement_ref_count();
    ge.collect();

}

