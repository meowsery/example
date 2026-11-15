// rules
// 1. anything that does not have "new" in front of it is located on the stack, if something cant be stored on the stack it will cause a compile error if you dont use "new" in front of it
// 2. anything stored on the heap must be manually free'd
// 3. every variable is immutable by default, use "mut" in front of the variable to make it mutable
// 4. no variable or function can have the same name
// 5. functions always pass a immuteable reference to the variable, unless it is a primitive
// 6. pointers automatically get the address that its assigned to
// 7. you have to copy a variable if you want to use it on another variable unless it is located on the heap

// variables
int var = 5 // creates an immutable variable
// int newowner = var // breaks rule 7 
int clone = !var // clones "var"
float fvar = 7 // a float can be a "integer", but it will be stored as a float, so 7.0
int mut mutvar = 4 // creates a mutable variable, this allows you to change the value of the variable
mutvar = 2
let dynamic = "Hello, world!" // creates a variable that can be any type, the type is determined by the value assigned to it
// dynamic = 5 // doesnt work because the type of the variable is already set to a string
print(dynamic.type()) // gets the variable type at compile time, meaning if the type changes, using the "type" function will not return the new type
let mut dynamic2 = new 15 // creates a variable that can change into any type, because of this the size is unknown so you will have to use "new" in front of it
int a, b = 1, 2 // sets 2 variables inside 1
str interpolated = $"This is {var}" // creates a string with a variable in it, the variable is replaced with its value
str concat = "This is " + var // same as above but uses concatenation instead of interpolation
string dynamicstr = new string("Cool string") // creates a string on the heap, this allows you to run the standard string functions on it
dynamicstr.insert(5, "awesome ") // inserts a string into the string at the specified index
print(dynamicstr) // -> Cool awesome string

// statements
int a = 1
int b = 2
if int c = !a + !b == 3 { // creates a variable thats assigned to a + b then checks if its equal to 3
    c += 1
}
print(c) // -> 4
bool divisible = !a % 4 == 0
print(divisible) // -> false
if c ?= divisible { // if "c" does not equal "divisible"
    c += 1
}

// pointers
int* mut dynamicptr = 1 // creates a pointer
print(dynamicptr) // -> 1
dynamicptr = 2 // changes the value of the pointer to 2
// as soon as the address of the pointer is free'd, the pointer will also be free'd
let* mut nullptr // creates a null/nil pointer

// arrays and vectors
int array = [1, 2, 3] // creates an array of integers
int[3] array -> fn(int i) { // runs the function for each element
    return i // "i" is the index of the array
}
// the same thing can be done with a vector
int[10] array2 // creates an array of 10 integers, each element is initialized to 0
int::simd simd = [1, 2, 3, 4] // creates a vector of integers, the size of the vector is determined by the amount of numbers in the array
int::simd[4] simd2 // creates a vector of 4 integers, each element is initialized to 0

// loops
for int i = 1, 10 {
    print(i)
    sleep(1) // sleeps for 1 second
}
for int mut i = 1, 10 { // since "i" is created every iteration unless its muteable, adding "mut" makes it so you can change the value of the interation
    if i == 5 {
        i = 10 // this will make the loop run 5 times instead of 10
    }
}
for 1, 5 { // since nothing gets the index inside this loop you can remove the variable and save performance
    
}
int n = 1
match numbers == n any numbers as [1,2,3,4] { // checks if any number in that array equals to 1
    print(numbers) // prints the number that worked
}
match numbers == n all numbers as [1,1,0] { // checks if all the numbers in that array equals to 1
    print(numbers) // prints the whole array
}
async while true { // a loop that runs asynchronously forever
    sleep(1)
}
int[25] elements
for int i = 0, elements.len(), 2 { // the third argument is the iteration step, "elements.len()" gets the length of the array
    def element -> elements[i] // makes "element" run "elements[i]", think of it as a function that returns "elements[i]"
    element = i // sets the value of each element to its index
    println(element) // prints the value of each element in the same line
    sleep(0.5)
}

// enum
enum Cat { // enums are a very simple way to store data in an efficient way
    // what a enum tells you is that a "Cat" has to be one of these types
    Black
    White
    Orange
}
Cat cat = Cat.Orange
if cat == Cat.Orange {
    print("Cat is orange")
}
enum Stats {
    Player {
        Heal(int)
        Damage(int)
    }
}
int health = 100
Stats stateffect = Stats.Player.Heal(10)
// stateffect.value = 30 // values are immuteable so you can not change it later
if stateffect == Stats.Player.Heal {
    health += stateffect.value
}
print(health) // -> 110

// define
def int2 -> int mut
for int2 i = 1, 10 { // this is the same as "for int mut i = 1, 10" because "def" replaces whatever the name is with the code after the arrow
    println(i)
}
def{int start = 0, int max = 10, int step = 1} loop -> for ref i = start, max, step // uses variables with default values to help the define function
loop { // defaults to "for ref i = 0, 10, 1"
    println(i)
}
loop(1, 5) { // makes "start" 1, "max" 5 and "step" defaults to 1
    println(i)
}

// hashmaps
hashmap::open openhash = { // creates a open addressing hashmap
    "key" = 151
}
hashmap::closed closedhash = { // creates a closed addressing hashmap
    "key" = 151
}
hashmap mut hash = new { // creates a hashmap (automatically closed)
    "key0" = 123
    "key1" = 542
    "key2" = 984
}
for key, value in hash { // gets every bucket in the hashmap in a random order
    if value == 123 {
        break
    }
}
int mut i = hash.len() - 1
for .value in hash { // only gets the value, if you only wanted to get the key then you would do "for key in hash"
    i += 1
    hash[$"key{value}"] = value * 2 // adds a new bucket
}

// functions
fn increment(int* mut i) { // makes "i" a pointer, which is known as a reference
    i += 1 // handle it as a normal pointer
}
// increment(1) // "1" is automatically immuteable meaning it isnt a variable either and there is not point in changing it
int mut number = 123
increment(*number) // "*" at the start to tell its a reference, which sends the address of "number"
print(number) // -> 124
fn functionhook(fn hook, args) { // "args" is every argument that is passed to the function after "hook"
    print(args)
    return hook(args)
}
fn add(int a, int b) {
    return a + b
}
functionhook(add, 5, 10) // -> (5, 10)
// "(5, 10)" is a tuple, a tuple is a collection of values that can be of different types, the values are ordered and can be accessed by their index
// basically an array but splits the values into different variables when passed to a function

// error handling
fn errortest(str hi) {
    if str == "hi" {
        error -> 123
    }
    return "no error"
}
str error = errortest("hi") // stops the program and prints the first element returned
str errorhandled = errortest("hi")? // doesnt stop the program but prints the first element returned
str errorfunction = errortest("hi") -> fn(int code) { // runs the function when an error happens
    if code == 123 {
        print("'hi' isnt allowed")
    }
}
fn check(str[] strings) {
    for int i = 0, strings.len() {
        assert(strings[i] ?= "1", $"String {i} is a invalid string") // if the statement is false then it errors with the error message
    }
}
str[5] strings = [
    "a0"
    "b1"
    "c2"
    "d3"
    "1"
]
check(strings)

// structs and classes
struct uselessdata {
    int a // creates a interger variable in the struct with a default value of 0
    float b = 1 // creates a float variable in the struct with a default value of 1
    int& c = 123 // "&" makes it so its read only, after you make this struct you will no longer be able to change this value
}
class player { // classes are like structs but they can have anything in them, while also having special functions as shown below, but they will always be located on the heap
    int hp = 100
    bool alive = true
    uselessdata data
    fn _init() { // this function is automatically called when the class is created
        print("Player joined!") // prints a message when the player is created
        self.changed(fn(str name, let value) {
            print($"Changed {name} to {value}") // prints a message when a variable in the class is changed
        })
    }
    fn _op(fn O) { // this function is automatically called when the class is used in a math operation
        get int A -> "hp" // gets the "hp" variable from the first player
        get int B -> "hp" // gets the "hp" variable from the second player
        return O(A, B) // applies the operation to the two hp values and returns the result
    }
}
player plr = new player() // -> Player joined!
player plr2 = new player(hp = 50)
print(plr + plr2) // -> 150

// libraries
// __ inside a new script __
int status = 0
lib Status { // basically a class but it can be used in any script
    fn getstatus() {
        return status
    }
}
// __ back to the main script __
import Status // imports everything in the "Status" library
// import Status::getstatus // would only import the "getstatus" function from the "Status" library
// import explicit Status // would import everything but everything inside would need to be called with "Status::" in front of it, good if you want to avoid name conflicts
// import Status as StatusLib // would rename it to be StatusLib instead of Status
print(getstatus()) // -> 0

// standard library manipulation
// __ inside a new script __
lib StringFunction {
    mix string.hello() {
        return $"Hello {self}"
    }
}
// __ back to the main script __
mix StringFunction
string String = new string("World")
print(String.hello()) // -> Hello World

// parallel
import explicit os::availableCores // how much cores are available to the program
int shared iterations = 0 // allows it to be used in parallel tasks, since its shared its automatically muteable 
ptask task -> parallel[os::availableCores()] for 0, 100 { // splits the load to each core
    atomic_add(*iterations, 1) // adds 1 to "iterations" in a safe way
    // iterations += 1 isnt safe so it cant be used
    // you can use atomic_sub for subtraction
}
print(iterations)
task.completed(fn(float t) {
    print($"Parallel task done, took: {t}s")
})

// misc
fn main() { // the start of the program

}
