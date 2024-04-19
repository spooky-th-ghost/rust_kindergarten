fn primitiveshit() {
    // There are different numeric types for variables with a couple different ways to declare
    // For integers there are the following:
    // Signed (Can be negative or positive):
    //     i8,
    //     i16,
    //     i32,
    //     i64,
    //     i128
    // Unsigned (Can only be positive)
    //     u8,
    //     u16,
    //     u32...

    let my_int8: i8 = 1;
    let my_int32 = 1;
    let my_int128 = 1_i128;
    println!("an i8: {my_int8}\n an i32:{my_int32}\n an i128:{my_int128}\n for integers you usually default to a u32 or an i32, the compiler will easily convert them as well");

    // decimals
    // you will just use f32 most of the time
    // but f64 does exist
    let my_float = 1.0;
    let my_explicit_float_1: f32 = 1.0;
    let my_explicit_float_2 = 1.0_f32;

    println!("these are all f32: {my_float}, {my_explicit_float_1}, {my_explicit_float_2} ");

    // there are a few string types that matter for reasons that will almost never effect you
    // if a string needs to be changed after initialization you use the <String> type
    let some_string = "My other string".to_string();

    // and if it's just a string that will be read and not changed use <&str>
    let static_string: &str = "My Static string";

    println!("My strings: {some_string}, {static_string}");
}

// Structs are rusts versions of classes
struct Person {
    //they have values in them just like classes
    name: String,
    //if something needs to access the value outside of the file it's declared in make it public
    pub age: u32,
}

// to define functions/methods for a struct you write an impl block like this
impl Person {
    // since we are not changing any of our fields with this method we use &self ( a reference to
    // the object the method is being called from)
    fn say_hi(&self) {
        println!("Hi, I'm {} and I'm {} years old", self.name, self.age);
    }

    // simmilar to fields, methods need to be public to be callable in different files
    // it's common practice to define your constructor for a struct as a 'new' function
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

//there is also a simpler type of struct called a tuple struct, which instead of having named
//fields just has a list of the types it contains
struct TupleBoy(pub u32);

fn structshit() {
    let colin = Person::new("Colin", 32);
    // if a struct doesn't have a new method you can create it using struct literal syntax
    let walter = Person {
        name: "Walter".to_string(),
        age: 31,
    };

    // you call methods on an instance of a struct how you would assume
    colin.say_hi();
    walter.say_hi();

    //for tuple structs they can be declared like this:
    let tuple_boy = TupleBoy(16);
    // and they're fields are accessed like this
    println!("Tupleboys value: {}", tuple_boy.0);
}

// the second (and last) complex data type in rust is an enum
enum Size {
    Small,
    Medium,
    Large,
}

// enums can have simple variants as you'd expect
// but they can also hold data like structs

enum AttackType {
    Basic(u32),
    Ranged {
        speed: u32,
        damage: u32,
        effect: String,
    },
}

/// Every Crate (essentially a unit of code, like a library) needs a main function to run
fn main() {
    primitiveshit();
    structshit();
}
