NIM (NIMBLE) PROGRAMMING LANGUAGE

CHAPTER 1: INTRODUCTION AND FUNDAMENTALS
----------------------------------------------------------------------------
Version: 1.0 (Runtime and Generics Supported, Consistent Syntax)
Philosophy: Procedural, Functional, Modular, Safe Control.

1.1 BASIC SYNTAX
NIM adopts C-style syntax.

Comments: Single: `// single-line comment`. Multiple: `/* multi-line comment */`
All blocks are defined with `{}`, and statements are terminated with a semicolon (`;`).

1.2 VARIABLES
General rules for naming variables are as follows:
Names can contain letters, numbers, and underscores.
Names must begin with a letter or an underscore (_).
Names are case-sensitive (myVar and myvar are different variables).
Names cannot contain spaces or special characters such as !, #, %, etc.
Reserved words (e.g., int, arr, char) cannot be used as names.

def variableName:type = value;

`var` Local or global variable. Not required.
`const` Constant value. Required.

Examples:
```
Nim

// Assign i32 with type inference.
// local, global variable
count = 10; // count:i32 = 10;

// Explicit type specification.
// local, global variable
var name:str = "NIM Language";
var ver:str = "ver: 0.1.5";

// Constant definition
const PI_VALUE:f64 = 3.14159;

// Multivariable definition
x,y,z:i32; // definition
x = 5, y = 6, z = 50; // assignment

(x = 5, y = 6, z = 50):i32; // definition and assignment

// Decimal separation
const PI:f32[4] = 3.14159;
// PI = 3.1415 / takes 4 digits after the decimal separation,
// if it is less than 4, it adds 0.
// example a:f32[4] = 1.25; // a = 1.2500.

// Tuple definition
position:(i32, i32) = (100, 250); // Two variables of type i32

position:(i32...) = (100, 250); // Multiple variables of type i32

// any: Single or multiple variables of any type.

post:any = ("ok", 200); // variable of any type (e.g.: int, arr, str, char, ptr)

```

CHAPTER 2: DATA TYPES
----------------------------------------------------------------------------

2.1 Basic and Special Types
Numeric: `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`, `f128`
`u8`, `u16`, `u32`, `u64`, `u128`

`int` = `i32`
`float` = `f32`
`void` = fn no return.
`bool` = `true` or `false`
`char` = 1 character `char[8]` 8 characters
`bit` = 1bit `bit[16]` 16 bits
`byte` = 8bit `byte[4]` 4 bytes
`null` = Null / Failed functions return null.
`ptr` = pointer pointer
`ref` = reference
`[]` = array `cars[];`, `cars:arr;` defines an empty array.
`arr` = array `cars:arr = ("Volvo", "BMW", "Toyota"); cars[0]; // Volvo`
`arr` = Generic Array. `cars:arr;` represents a generic array type.
`[]` = Dynamic Array. `cars[];` defines an empty, dynamic array.

`str` = Text: `str` (Default UTF-8). Absolute encoding: `var msg:str[utf16] = "Data";`
`str[30];` All str "strings" in the block are limited to 30 characters.

`...` = Variable Number of Parameters Declaration (Homogeneous Type).

The `add(a:i32...)` syntax tells the function to pass only a variable number of parameters `(a[0], a[1],...)` of type `i32`.

`any` = Variable/Mixed Type (Heterogeneous). `str` can be any one of `int`, `float`, `arr`, etc., or multiple mixed types.
When used as a Multiple Return Type, `(return myret:any(a:i32, b:str);)` returns values ​​from different types together and can be accessed as an array (`myret[0]` returns i32).

2.2 The bit width of basic types is strict. Explicit Casting is mandatory to prevent data loss.


Nim

var large_num:i64 = 9000000000;

// Floating-point usage
var ratio:f64 = 1.25;

// Pointer type
var ptr_data:ptr = null;



2.3 Numbers: This supports the use of the letter (e or E) to indicate "10 to the power of 10."
`35000 = 35e3`

2.4 Type Conversion: Explicit Casting rule.
The `types` module contains conversion methods for all number types.
```
// Type Conversion
large_num:i64 = 9000000000;
//......
small_num:i32 = to_i32(large_num); // Converts to i32. The to_i32() function performs the conversion.
```
`to_i64(value_to_convert)`, etc. The `types` module contains the same information for all number types...

CHAPTER 3: OPERATORS
------------------------------------------------------------------------------------------------

3.1 Arithmetic operators
`+` Addition
`-` Subtraction
`*` Multiplication
`/` Division
`%` Modulus
`++` Increment `x++`
`--` Decrement `x--`

3.2 Assignment operators
List of all assignment operators:
`=` `x = 5` `x = 5`
`+=` `x += 3` `x = x + 3`
`-=` `x -= 3` `x = x - 3`
`*=` `x *= 3` `x = x * 3`
`/=` `x /= 3` `x = x / 3`
`%=` `x %= 3` `x = x % 3`
`&=` `x &= 3` `x = x & 3`
`|=` `x |= 3` `x = x | 3`
`^=` `x ^= 3` `x = x ^3`
`>>=` `x >>= 3` `x = x >> 3`
`<<=` `x <<= 3` `x = x << 3`

3.3 Comparison operators
`==`Equal
`===` Identical
`!=` Not equal
`<>` Not equal
`!==` Not identica
`>` Greater than
`<` Less than
`>=` Greater than or equal to
`<=` Less thanor equal to

3.4 Logical operators
`and` And
`&&` And
`or` Or
`||` Or
`xor` Xor
`!` Not

3.5 Bitwise operators
Bitwise operators operate on individual bits of integers
`&` : AND -
`|` : OR -
`^` : XOR -
`~` : NOT -
`<<`: Left shift
`>>`: Right shift

CHAPTER 4: BUILT-IN FUNCTIONS CONTROL STATEMENTS
--------------------------------------------------------------------------------------

4.1 if, if else, if else if else :
if
```
if (condition) {
// code to be executed if condition is true;
}
```
if else
```
if (condition) {
// code to be executed if condition is true;
} else {
// code to be executed if condition is false;
}
```
if elseif else
```
if (condition) {
code to be executed if this condition is true;
} elseif (condition) {
// code to be executed if the first condition is false and this condition is true;
} else {
// code to be executed if all conditions are false;
}
```
if else shorthand
`b:str = (a < 10) ? "Hello" : "Good Bye";`

4.2 Pattern Matching The `match` expression and the `def` default case.
Match Expression (Pattern Matching)
Checks a value against multiple patterns.
```
Nim

fn CheckStatus(code: int):void {
match (code) {
200: {
echo("Operation Successful.");
}
404: {
// Multiple patterns in a single block
echo("Page not found.");
}
def: { // Default case
echo("Unknown HTTP code.");
}
}
}
```

4.3 Controlled Jump `rolling:TAG` mechanism and `$rolling` counter.
On error, rolls the stream back to a previous tag in the same scope. $rolling provides an automatic counter (starting at 0).
Example:
```
Nim

const MAX_RETRIES = 3;

REQUEST_BLOCK: { // Where $rolling = 0

connect = network.HTTP.connect(url);
// error checks....
if(connect){
/* success...*/

}
elseif ($rolling < MAX_RETRIES) {
echo("Try {$rolling}...");
// Increments the counter by 1 and returns to the beginning.
rolling:REQUEST_BLOCK; // Increments $rolling by 1 and returns to the beginning.
}
else {
// Failed to connect after 3 retries.
echo("Permanent Connection Error.");
}
}
```

I. Rolling Mechanism
The `rolling`: statement is a control flow command, not a function that returns a value.
Therefore, having the `$rolling` variable only hold the counter value clarifies the structure.
On an error, it returns the flow to the beginning of the label within the same scope.
This gives the programmer control over the specified number of retries, rather than the classic `try/catch` structure.
`$rolling` provides the label's automatic counter (starting at 0).
This has no direct return value.
Carrying the Counter Value: `$rolling` only holds the number of iterations.
This prevents the variable from appearing "magical" and directly ties it to the `rolling` mechanism.
Variable Declaration Location Task `$rolling` is implicitly defined within the block containing the `rolling` label.

4.4 Group Structure:
group definition: labeled entry points and chained access.
It is a modular container with labeled entry points. `def` is the default label.
```
Nim

group TaskGroup(x:i32, y:i32, z:i32) {
//
val:i32 = 10;

// Labeled function
double: fn (x:i32):i32 -> x * 2;

cube: { return x * x * x; }
// Subgroup
sub_group: group(x) {
// Subgroup label
square: { return x * x; }
// Subgroup default, not required.
def: { echo("Subgroup default. Enrollment: {val}"); }
}
// group Default. Not required.
def: { echo("Main Group Default. Enrollment: {val}"); }
}

void fn Main() {
TaskGroup(); // calls the def block. // output: "Main Group Default. Enrollment: 10"
TaskGroup.cube(3); // calls the cube block. // output: "27"
TaskGroup.double(3); // calls the subgroup square block. // output: "6"
TaskGroup.sub_group.square(3); // calls the subgroup block. // output: "9"
TaskGroup.sub_group(); // calls the subgroup def block if it exists. // output: "Subgroup default. Enrollment."
}
```
4.5 While Loop:
```
// while
i:i8 = 0;

while (i < 5) {
echo("\n{i}");
i++;
}
```
4.6 For Loop:
`for (i, i < 5, i++) {...}` // i starts from 0
`for (i=1, i <5, i++) {...}` // i starts from 1

(For operation on arrays) `for (myArr in val) {...}` // assigns the elements in an array (myArr) to (x) in order, starting from the first element. It stops when all elements in the array are exhausted.
```
colors:arr = ("red", "green", "blue", "yellow");
for (colors in x) {
echo(x);
}

```
4.7 Break, continue, return statements:
Even if the end of the loop has not been reached, we can stop the loop with the following statement: `break`.
```
colors:arr = ("red", "green", "blue", "yellow");
for (colors in color) {
if (color == "blue") break;
echo("\n{color}");
}
```

When a specified condition is met, the statement `continue` breaks an iteration (in the loop) and continues with the next iteration in the loop.
```
for (i, i < 10, i++) {
if (i == 4) {
continue;
}
echo("\n{i}");
}
```

4.8 Native I/O Functions echo, Input, len(data), and variable interpolation. These functions do not require an external module.
I/O and Other Native Functions
echo can also be used locally without any modules.requests.

`echo("Text {var}")`: Prints output to the console, supports variable interpolation. `{val}` cannot have spaces on the right or left of curly brackets.
`echo("Text { var }")`: `{ var }`, `{ var}`, `{var }`, `\{var}` are not variables, but strings.
`echo(var)`: var is a variable here. When written without "", only variable names can be used to print.

`prompt:i32 = input("Enter a number?")`: Takes input from the user. //print, in input module: io
`len(data)`: Returns the length of the collection/string.
Escape characters such as `\` are supported.
```
Nim

// Supports Variable Interpolation
var user_name:str = input("Enter your name: ");
echo("Welcome, {user_name}");

list nums:i32 = [10, 20, 30];
var count = len(nums); // len(data) syntax
```

CHAPTER 5: FUNCTIONS AND SCOPE
----------------------------------------------------------------------

5.1 Function Definition: `fn` syntax, void non-return functions.
fn Function Definition.
Function Parameters and Return: `fn <Name>(<param:type>,...):<Return Type>{...} `
```
Nim:

fn addNumbers(a:f32, b:f32):f32 {
return a + b;
}

echo(addNumbers(1.2, 5.2));

```

`fn func_name(...):i32{//...}` // multiple parameter acceptance.
`fn func_name(myarr[]):arr{//...}` // takes an array as a parameter and returns an array.
`fn func_name():void{//...}` // void non-return functions.
`fn func_name(){//...}` // void non-return functions.
`fn (x:i32):i32 -> x * 2;` // Lambda/Anonymous Functions
`echo("Square a Number: {square(input("Enter a number:"))}");` // function use for parameters.

5.2 Optional Arguments (param:type = value) and Named Argument Calling.
Named and Optional Arguments
Makes function calls more flexible and improves readability.
Optional Argument: Created by assigning a default value to a parameter in the function definition.
Syntax: `param:type = <default_value>`
Named Arguments: Explicitly specifying parameter names during the function call.
This allows for positional ordering.
Example Usage:
```
Nim

void fn Render:void(x:i32, color:str = "blue", size:i32 = 10) {
echo("X:{x}, Color:{color}, Size:{size}");
}

void fn Main() {
// 1. Use defaults (Positional)
Render(5); // Output: X:5, Color:blue, Size:10

// 2. Replace only one default (Named)
Render(10, size: 20); // Output: X:10, Color:blue, Size:20

// 3. Shuffle the Order (Named)
Render(color: "red", x: 15, size: 30); // Output: X:15, Color:red, Size:30
}
```

5.3 Callback
A callback function is a function passed as an argument to another function.
You can also pass functions that take parameters - just make sure the function pointer type matches:
```
fn addNumbers(a:int , b:int ) {
printf("The sum is: {a + b}");
}

fn calculate((*callback)(int, int), x:int , y:int ) {
callback(x, y);
}

int main() {
calculate(addNumbers, 5, 3);
return 0;
}
```

5.4 Asynchronous Programming:
`async` (asynchronous function definition) and `await` (asynchronous wait for a result).

This enables Coroutine-based concurrency in system programming and network operations.

5.5 Error Path: Using `Option<T>` and `Result<T, E>`.
Error Path: Types representing the operation result:
`Option<T>`: Indicates whether the value is present or absent (null).
`Result<T, E>`: Returns a success value (`T`) or an error value (`E`).
Instead of throwing an error, it forces an error-free path by embedding the result in the return type.
```
Nim

// Returns an int on success and a string on error.

fn SafeDivide:any(a:int, b:int) {
if (b == 0) {
return (null, "Division by zero"); // Error path (Result: E)
}
return (a / b, null); // Success path (Result: T)
}

echo(SafeDivide(9,0)); // Output: Division by zero
echo(SafeDivide(9,3)); // Output: 3

```

5.6 Inline function
This is a small function that asks the compiler to insert its code directly into the call location, rather than jumping directly into the code.
```
inline fn square(x:int ):int {
return x * x;
}
```

5.7 Recursion
Recursion is the technique of performing a function call on itself.

This technique provides a way to break complex problems into simpler ones that are easier to solve.

Example
```

fn factorial(n:int):int {
if (n > 1){
return n * factorial(n - 1);
}else{
return 1;
}
}

int main() {
echo("Factorial of 5 is {factorial(5)}");
return 0;
}

```
Factorial means multiplying a number by all the numbers following it up to 1.

For example, the factorial of 5 is: 5 * 4 * 3 * 2 * 1 = 120.

CHAPTER 6: MEMORY ADDRESS AND POINTER
---------------------------------------------------
6.1 Memory Address
When we assign a value to a variable, this value is stored at this memory address.
To access this, use the reference operator `&`, and the result will show where the variable is stored:

```
myAge:i32 = 42;
echo(&myAge); // Outputs 0x7ffe5367e044
```
Note that it is also often called a "pointer."
`&myAge` A pointer essentially stores the memory address of a variable as its value.

6.2 Pointer Pointer
bildWhen used in a unit, it creates a pointer variable `*ptr:i32`.

When not used in a declaration, it acts as a dereference operator.

`myAge:i32 = 42; // Variable declaration`
`*ptr:i32 = &myAge; // Pointer declaration`

// Reference: Output the memory address of myAge with the pointer (0x7ffe5367e044)
`echo(ptr);`

// Dereference: Output the value of myAge with the pointer (42)
`echo(*ptr);`

6.3 Pointers and Arrays
You can also use pointers to access arrays.
```
myNumbers[4]:int = {25, 50, 75, 100};

for (i, i < 4, i++) {
echo("\n{&myNumbers[i]}");
}
```
```
Result:

0x7ffe70f9d8f0
0x7ffe70f9d8f4
0x7ffe70f9d8f8
0x7ffe70f9d8fc
```
Note that the last number in the memory address of each element is different and is added by 4.

Because the size of an int(i32) type is 4 bytes.

The name of an array is actually a pointer to the first element of the array.
The memory address of the first element is the same as the name of the array:
```
myNumbers[4]:i32 = {25, 50, 75, 100};
// Get the memory address of the myNumbers array
echo(myNumbers);
// Get the memory address of the first array element
echo("\n {&myNumbers[0]}");
```
```
Result:

0x7ffe70f9d8f0
0x7ffe70f9d8f0
```
This basically means we can work with arrays using pointers!
Since `myNumbers` is a pointer to the first element in `myNumbers`, you can use the `*` operator to access it.

6.3 Pointer Arithmetic:
Pointer Arithmetic Depends on the Type. Not all pointers behave the same.
When you add to a pointer, the pointer moves by the size of the object it points to; not just by 1 byte.

For example:

A *pointer:i32 moves by the size of an integer (4 bytes).
A *pointer:char moves by the size of a character (1 byte).
A *pointer:str moves the size of one character (4 bytes). UTF8 characters occupy 4 bytes.

So, if both pointers start at memory address 1000:
int *→ p + 1 would move to address 1004
char *→ p + 1 would move to address 1001
str *→ p + 1 would move to address 1004
This shows that pointer movement depends on the data type it points to, not the number you insert:

6.4 Function Pointer
A function pointer is like a regular pointer, but instead of pointing to a variable, it points to a function.
This means it stores the address of a function and allows you to call that function using the pointer.
Function pointers allow you to decide which function to execute while the program is running, or when you want to pass a function as an argument to another function.
They are useful for callbacks, menus, and flexible program design.

`(*pointerName):returnType (parameterType1, parameterType2, ...);`
`(*ptr):int(i32...);`
You can assign a function to a pointer in two ways:
`ptr = add;`
`ptr = &add;`
Both are the same, because the function's name already represents its address in memory.

Once the pointer is assigned, you can call the function in two ways:
`ptr(5, 3);`
`(*ptr)(5, 3);`
Both are valid and perform the same function.

A pointer to a function that adds two numbers:
```
add:int( a:int, b:int) {
return a + b;
}

int main() {
(*ptr):int(i32...) = add;
result:int = ptr(5, 3);
echo("Result: {result}");
return 0;
}
```
Note: A function name, by itself, points to the beginning of its code in memory. So, a function name already acts like a pointer!
Declaring a function pointer gives you a variable that can hold only that address; you can modify it or pass it on.

Function pointers can be passed to other functions—this is called a callback.

This allows a function you give as input to call another function.

CHAPTER 7: STRUCTURES (structs)
--------------------------------------------------------------------------------------------
7.1 struct
Structures are a way to group several related variables together.
Each variable in a structure is called a member of the structure.

Unlike an array, a structure can contain many different data types (int, float, char, etc.).
```
// Create a structure called myStructure
struct myStructure { // Structure name
myNum:i32, //
myLetter:char //
}

int main() {
// Create a structure variable of myStructure called s1
myStructure => s1;

// Assign values ​​to members of s1
s1.myNum = 13;
s1.myLetter = "B";

// echo values
echo("My number: {s1.myNum}\n");
echo("My letter: {s1.myLetter}");

return 0;
}
```
A structure can also contain another structure as a member.
This is called a nested structure and is useful when you want to group related data into layers:
```
// NIM type
struct Owner {
firstName:str[30],
lastName:str[30]
}

struct Car {
brand:str[30],
year:i32,
Owner => owner // Nested structure
}

int main() {
Owner => person = {"John", "Doe"};
Car => car1 = {"Toyota", 2010, person};

echo("Car: {car1.brand} : {car1.year}\n");
echo("Owner: {car1.owner.firstName} {car1.owner.lastName}\n");

return 0;
}
```

'Extend' to define methods

```
// 1. struct definition
struct Point {
x: i32,
y: i32
}

// 2. 'Extend' block to define methods
extend Point {
// Constructor function
fn new(x: i32, y: i32): Point {
return { x: x, y: y }; // Return with the struct literal
}

// Immutable method (self: reference)
fn get_x(self: ref): i32 {
return self.x;
}

// Immutable method (self: ptr - pointer)
fn move_to(self: ptr, new_x: i32, new_y: i32): void {
self.x = new_x;
self.y = new_y;
}
}

// Usage:
void fn Main() {
var p1 = Point.new(10, 20); // Calls the constructor
echo("X: {p1.get_x()}"); // Method call
p1.move_to(50, 60); // Replaceable method call
}
```

7.2 typedef
The keyword allows you to create a new name (an alias typedef) for an existing type.

This makes complex declarations easier to read and your code easier to maintain.

For example, instead of always writing float, to make the code more understandable, we can create a new type called: Temperature
```
typedef Temperature:f32;

int main() {
today:Temperature = 25.5;
tomorrow:Temperature = 18.6;

echo("Today: {today}C\n");
echo("Tomorrow: {tomorrow}C\n");

return 0;
}
```

7.3 Enum Enumeration
An enum is a special type that represents a group of constants (immutable values).
To create an enum, use the enum keyword, followed by the enum name and separate the enum elements with commas:
```
enum Level {
LOW,
MEDIUM,
HIGH
}
```
Note that the comma is not needed in the last item.
Using uppercase letters is not mandatory, but is often considered good practice.

By default, the first element ( LOW ) has a value of 0, the second element ( MEDIUM ) has a value of 1, and so on.
Now, if you try to print myVar, you will get output representing MEDIUM:
```
int main() {
// Create an enum variable and assign a value to it
Level myVar = MEDIUM;
// echo the enum variable
echo(myVar);

return 0;
}
```

As you know, the first element of an enum has a value of 0. The second element has a value of 1, and so on.
You can easily change the values ​​to better understand them:
```
enum Level {
LOW = 25,
MEDIUM = 50,
HIGH = 75
}

echo(myVar); // Now outputs 50
```

If you assign a value to a specific element, the numbers of subsequent elements are updated accordingly:
```
enum Level {
LOW = 5,
MEDIUM, // Now 6
HIGH // Now 7
}
```

CHAPTER 8: MEMORY / memory.nim module. See memory.md
-------------------------------------------------------------------
Default Memory Model Ownership Ownership rules and automatic management.

Automatic Management: Dynamic types (`list`, `string`), etc., are automatically freed according to ownership rules.

8.1 Low-level memory control functions.
`memory` Group: Manual memory control (`memory.Alloc`, `memory.Calloc`, `memory.reAlloc`, `memory.Free`, etc.) is used by the programmer when necessary.

Dynamic memory is not owned by a variable; it can only be accessed through pointers.

You can use the `memory.Alloc` or `memory.Calloc` functions to allocate dynamic memory.

You must include the header to use these. Both allocate a specified amount of memory and return a pointer to its address.

*ptr1:int = memory.Alloc(size);
*ptr2:int = memory.Calloc(amount, size);


The function determines how much memory, measured in bytes, is available. There is a parameter called `memory.Alloc(size)`, which specifies the allocated memory.

The function `memory.Calloc(amount, size)` has two parameters:
amount - Specifies the amount of elements to be allocated.
size - Specifies the size of each element, measured in bytes.

The best way to allocate the correct amount of memory for a data type is to use the `sizeof` operator:
```
*ptr1, *ptr2:int;
ptr1 = memory.Alloc(sizeof(*ptr1));
ptr2 = memory.Calloc(1, sizeof(*ptr2));
```

As mentioned earlier, we cannot use `sizeof` to measure how much memory is allocated; we must calculate it by multiplying the number of elements by the size of the data type:
Example
```
*students:int;
numStudents:int = 12;
students = memory.Calloc(numStudents, sizeof(*students));
echo(numStudents * sizeof(*students)); // 48 bytes
```

8.2 Accessing Dynamic Memory
Dynamic memory behaves like an array whose data type is specified by the type of the pointer.

As with arrays, to access an element in dynamic memory, look at its index number:

`ptr[0] = 12;`
You can also dereference the pointer to access the first element:

`*ptr = 12;`

Example
Reading and writing from dynamic memory:
```
// Allocate memory
*ptr:int;
ptr = memory.Calloc(4, sizeof(*ptr));

// Write to the memory
*ptr = 2;
ptr[1] = 4;
ptr[2] = 6;

// Read from the memory
echo(*ptr);
echo(ptr[1], ptr[2], ptr[3]);
```

8.3 Reallocate Memory
If the amount of memory you've allocated isn't enough, make it largerYou can reallocate it to resize it.
Reallocation allocates a different (usually larger) amount of memory while preserving the data stored therein.

You can change the size of the allocated memory using the function `memory.reAlloc()`.
The function `memory.reAlloc()` takes two parameters:

`*ptr2:int = memory.reAlloc(ptr1, size);`
The first parameter is a pointer to the resized memory.
The second parameter specifies the new size of the allocated memory in bytes.

Increase the size of the allocated memory:
Example
```
*ptr1, *ptr2, size:int;

// Allocate memory for four integers
size = 4 * sizeof(*ptr1);
ptr1 = memory.Alloc(size);

echo("{size} bytes allocated at address {ptr1} \n");

// Resize the memory to hold six integers
size = 6 * sizeof(*ptr1);
ptr2 = memory.reAlloc(ptr1, size);

printf("{size} bytes reallocated at address {ptr2} \n");
```

Note: You should always include error checking (if ptr2 == NULL) when allocating memory.

Note: You should also always free the allocated memory when you're done.

This is important to ensure your program behaves as expected,
but it also makes it more maintainable and efficient.

8.4 To free memory, simply use the `memory.free()` function:

Example
Free allocated memory:
```
// Free allocated memory
memory.free(ptr1);
ptr1 = null; // Set the pointer to `null` after freeing the memory.
```

CHAPTER 9: MODULARITY AND SYSTEM
---------------------------------------------------

9.1 Module System `export` and `use` (Selective/Full Import) rules.
How the Module System Works
Default Visibility (Hidden)
In NIM, all variables, constants, functions, and structures defined within a module (usually a file or a group block) are private by default.
No other module can directly access these elements.

9.2. Export (`export`)
The `export` keyword explicitly marks elements that a module wants to be accessible by other modules.
Rule: For an element to be used elsewhere, it must be marked with export in the module in which it is defined.

Example (in the math.nim file):
```
Nim
:math;
// This can only be used within the math module, as it is not exported.
var PI_INTERNAL = 3.14;

// It can be used in another module because it is exported.
export const PI = 3.14159;

// Exported function
export fn Add:i32(a:i32, b:i32) {
return a + b;
}
```
9.3. Importing (`use`)
The `use` keyword is used to include elements exported from another module into the current module. There are two main usage patterns:

9.3 -1 Full Module Import (Prefixed Usage)
The entire module is imported. The module name is used as a prefix to avoid naming conflicts during usage.

Syntax: `use <module_name>;`

Example (in the `main.nim` file):
```
Nim

use math; // The math module is imported.

void fn Main() {
var result = math.Add(5, 3); // The math. prefix is ​​required.
echo("Result: {result}");
}
```

9.3 -2 Selective Import (Direct Usage)
Only certain elements from the module are imported by specifying them in curly brackets `{}`.
Imported elements are directly prefixed with the `:module_name;` defined at the beginning of the module. This prevents name conflicts.

Modules written as groups can have functions and variables with the same name tag.

Syntax: `use <module_name> { <item_1>, <item_2> }`

Example (in main.nim):
```
Nim

// Only the Add function and the PI constant are imported from the math module.
use math { Add, PI }

void fn Main() {
var result = math.Add(10, 5);
echo("Result: {result}");
echo("PI Constant: {math.PI}");
}
```

9.4 Module Writing:
Application Example: Using `group` within a `Module`
When designing a module, grouping all related operations into a single group significantly organizes that module's API (application programming interface).

For example, you write a module called `network.nim`:
Grouping Within a Module: You group all HTTP-related operations into a group.

```
//--------------------------------------------------------------------------------------
// Nim
//
// The network.nim module file name may differ from the name used as the prefix for the module.
// Multiple modules can be in a single file. Modules are wrapped in groups.
// In the example, there are two modules. HTTP and SOCKET, but in a single file.
// Both are accessed with the network. prefix. They can be defined separately if necessary.
//--------------------------------------------------------------------------------------

:network; // Module name to use as prefix.
export group HTTP(any...){ // Exporting group
connect: { /* Connection code */ }

export get_data: { /* ... */ } // Can also be exported from within the group, preferable to the entire group.

dc: { /* Close connection .... */ }

def: { HTTP.connect(); echo("HTTP Group Started."); }
}

export group SOCKET(any...){ // Exporting group
connect: { /* Connection code */ }......
}
```

Focused Access: When using this module in your main module (`main.nim`), you benefit from the focus of both the module system and the group structure:

```
Nim

// main.nim
use network; // import the entire network module

void fn Main() {
// Direct focus access to group and its tag
network.HTTP.get_data();

// if there is a single import within the group, not the entire group
network.get_data(); // etc...

// Or you can call the default tag.
network.HTTP();
}
```

Conclusion: The group structure is an excellent choice for writing modules because it allows you to break down large and complex APIs (data structures, I/O, memory) into logical units and allow the user to focus on the exact function they want through chained access.
This maximizes both the modularity and readability of the language.

10 CPU AND PERFORMANCE CONTROL
-------------------------------------------------------------------

10.1 Register Access cpu.nim
Provides direct access to CPU registers so the programmer can optimize speed in performance-critical algorithms.

Type Safety: Functions are implemented as `Generic (<T>)`. The type of the value must be specified for read and write operations.
Architectural Dependency: Register IDs `(id)` are semantically mapped to the compile target architecture `(x86, x64, ARM, etc.)`.
Syntax Description: `cpu.set_reg(id: i32, value: T)` Writes the value value to the specified id (register number) location.
`T fn cpu.get_reg(id: i32)` Reads the value from the specified register and converts it to the type `T`.

10.2 Fast Execution Scope
`(fast_exec: LABEL)` Indicates to the compiler that register allocation for variables and operations within a given code block should be prioritized.

Rule: For variables within this block, the compiler attempts to use registers instead of stacks.

Syntax:
```
Nim
fast_exec: TEST{
// Generates virtual registers. The compiler sets these registers first, based on code integrity.
// The code in this block takes priority in register optimization.
// Speed-critical algorithms (Sorting, Searching) are written here.

cpu.set_reg(0, 0); // Initialize R0

while (cpu.get_reg(0) < 10) {
// ...
cpu.set_reg(0, cpu.get_reg(0) + 1);
}
}
```


## SECTION 11.0: Assembly (ASM) Integration

NIMBLE allows assembly (ASM) code to be embedded directly into the language for performance-critical or hardware-specific operations.

This mechanism bypasses the language's abstraction layer and allows direct use of the processor's instruction set.

It is generally considered an extension of the **`cpu`** module.

### 1. `asm:TAG { ... }` Block

The `asm` block instructs the NIMBLE compiler to process the enclosed text as the assembly syntax of the target architecture.

| Structure | Purpose | Description |
| :--- | :--- | :--- |
| **`asm:TAG { ... }`** | Inline Assembly Block. Allows direct writing of assembly code. | The tag is required but is generally not used as a control flow tag; it is for identification purposes only. |

**Example: Simple Assembly Operation (Assuming x86-64)**

```nim
Nim

var addition_result: i64;

asm: ADDITION_CODE {
// Move the value 5 to rax
mov rax, 5
// Add 10 to rax
add rax, 10
// Store the result from rax into the NIMBLE variable (see 19.1)
mov %addition_result, rax
}
```

1.1 ASM Variable Access
Assembly code within the asm block can access local variables of the surrounding NIMBLE function.
This is the basic mechanism for exchanging data between Assembly and NIMBLE code.
```
Syntax, Purpose, Description
%variable_name, Reference to the NIMBLE variable.
"This syntax indicates that the compiler will replace the variable with the register or stack address when the assembly code is running."
(%variable_name), Access the value via the variable's address. "Depending on the assembly syntax, it may be necessary to use the address of the variable itself (%variable_name) or the contents of the address ((%variable_name))."
```

2. Micro-Optimization: Scope of fast_exec
Because the asm block is used for performance-critical purposes by nature, it is automatically considered within the scope of the fast_exec micro-optimization.
```
Structure, Purpose, Description
fast_exec, Micro Optimization Block.,"Instructs the compiler to apply the most aggressive speed optimizations for code in this scope.

Usually used for NIMBLE code surrounding assembly blocks."
```

Rule: The asm block itself is always in the fast_exec scope. If the surrounding NIMBLE code is also expected to receive the same optimization, it must be explicitly wrapped in a fast_exec { ... } block.

Example: fast_exec Usage
```
Nim

void fn Calc() {
// Apply aggressive optimization to this NIMBLE code
fast_exec {
var a = 10;
var b = 20;

// This asm block is already in the fast_exec scope.
asm: CRITICAL_ADD { 
mov rax, %a 
add rax, %b 
mov %total, rax 
} 
}
}

```

## SECTION 11.1: `cpu` Module

CPU Processor Control and Performance Tools.

Provides functions and tools to interact directly with kernels, registers, and processor instructions.

Designed for high-performance system programming, embedded systems, and critical algorithms.

The CPU module is critical to the high-performance and low-level system programming goals of the NIMBLE language.

The fast_exec and asm blocks in your document already address this area, so the module's function should be to further systematize the behavior and capabilities of these blocks.

CPU Module Detailing
The CPU module allows you to interact directly with the hardware (CPU), fine-tune performance, and combine high-level features of the language with low-level Assembly (ASM) programming.

CPU Processor Control and Performance Tools.
Provides functions and tools to interact directly with kernels, registers, and processor instructions. Designed for high-performance system programming, embedded systems, and critical algorithms.

1. Performance and Control Functions
These functions allow the programmer to manipulate and query the CPU environment in which code is running.
FunctionPurposeSyntax and Description

`reg_get()` Returns the current value of the specified register.
`var rax_value = cpu.reg_get(RAX);` Note: Registers such as RAX, RBX, and RCX are defined as constants in the CPU module.

`reg_set()` Sets the value of the specified register.
`cpu.reg_set(RBX, 500);`

`core_count()` Returns the number of physical or logical cores in the current system.
`var num_cores = cpu.core_count();`

`pause()` Optimizes performance in hyper-threading (HT) and spin-lock situations by sending a short pause (similar to NOP) command to the processor.
`cpu.pause();`

`clflush()` Clears the data at the specified memory address from the CPU cache. It is critical for hardware synchronization.
`cpu.clflush(ptr_adres);`

The cpu module is critical for the high-performance and low-level system programming goals of the NIMBLE language.
The fast_exec and asm blocks in your document already address this area, so the module's function should be to more systematically define the behavior and capabilities of these blocks.

2. Timing and Measurement Tools: Retrieves data directly from the CPU to measure the performance of critical code blocks.

`rdtsc()` Reads the value of the Timestamp Counter (RDTSC). Provides a very precise time measurement, but one that can be affected by frequency changes.
`var t1 = cpu.rdtsc();`

`get_freq()` Returns the current CPU operating frequency (in MHz).
`var freq = cpu.get_freq();`

3. Assembly (ASM) Integration
The `asm:TAG { ... }` construct, already present in the documentation, should be considered a natural extension of the cpu module.

`asm:TAG { ... }` Inline Assembly Block. Allows writing assembly code directly.
These blocks are assumed to be within the cpu module scope to improve performance.
ASM Variable Access: NIM variables are referenced as register/stack addresses with special syntax (`%variable_name`).

`fast_exec` Micro-Optimization Block.
Instructs the compiler to apply the most aggressive speed optimizations for code within this scope.
It is typically used for code surrounding asm blocks. It can be assumed that this construct is automatically applied to functions within the module.


## SECTION 11.2: The `file` Module

| Module Name | Description |
| :--- | :--- |
| **`file`** | **File System and I/O Operations.** Provides tools for performing basic file system operations such as opening, reading, writing, deleting, and copying files. All I/O functions generally return the type **`Result<T, E>`** for safe error handling. |

### 1. Basic Constructs and Types

The `file` module requires special types to manage operations:

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`FileHandle`** | Pointer (handle) representing an open file. | The successful return type of the `file.open()` function. |
| **`FileMode`** | Constants that determine how the file is opened. | `file.READ`, `file.WRITE`, `file.APPEND`, `file.READ_WRITE` |
| **`FileError`** | Lists possible errors (error codes) that may occur during file operations. | `ERR_NOT_FOUND`, `ERR_ACCESS_DENIED`, `ERR_IO_FAILED` |

### 2. File Management Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`open()`** | Opens the file at the specified path in the specified mode. | `fn open(path: str, mode: FileMode): Result<FileHandle, FileError>` |
| **`close()`** | Closes the open `FileHandle` and releases its resources. | `fn close(handle: FileHandle): void` |
| **`read_all()`** | Reads the entire contents of the file as a string. | `fn read_all(handle: FileHandle): Result<str, FileError>` |
| **`write()`** | Writes the specified data to the file. Returns the number of bytes successfully written. | `fn write(handle: FileHandle, data: str): Result<i32, FileError>` |
| **`delete()`** | Permanently deletes the file at the specified path. | `fn delete(path: str): Result<void, FileError>` |
| **`copy()`** | Copies a file from the specified source to the specified destination. | `fn copy(src_path: str, dest_path: str): Result<void, FileError>` |
| **`exists()`** | Checks whether a file exists at the specified path. | `fn exists(path: str): bool` |

### 3. Example Usage: Safe File Reading

```
Nim

void fn Main() {
var fileResult = file.open("settings.cfg", file.READ);

// Using the match statement for error checking
match fileResult {
Ok(handle) => {
var contentResult = file.read_all(handle);

match contentResult {
Ok(content) => {
print("File Content:\n{content}");
},
Err(error) => {
print("Read Error: {error}");
}
}
file.close(handle); // Close the file
},
Err(error) => {
// File not found, access denied, etc.
print("Error Opening File: {error}");
}
}
}
```

```
Nim

void fn WriteExample() {
var fileResult = file.open("log.txt", file.WRITE);

match fileResult {
Ok(handle) => {
var writeResult = file.write(handle, "This is the log message.\n");

if writeResult.is_ok() {
print("Number of bytes written successfully: {writeResult.unwrap()}");
} else {
print("Write Error!");
}
file.close(handle);
},
Err(error) => {
print("Error Opening File: {error}");
}
}
}
```

## SECTION 11.3: `array` Module

| Module Name | Description |
| :--- | :--- |
| **`array`** | **Dynamic Array Functions.** Provides helper methods and advanced operations for collections that contain elements of the same type in a sequential order, the size of which can be changed at runtime. |

### 1. Declaration Syntax (Simplified)

There is no need for a special `array.new()` function for dynamic arrays. The array type is defined by a `[]` following the element type and is initially assumed to be empty.

| Syntax | Description |
| :--- | :--- |
| `var myArray: i32[];` | Defines an empty dynamic array of type `i32`. |
| `var stringList: str[];` | Defines an empty dynamic array of type `string`. |
| **Initialization:** `var nums: i32[] = {1, 2, 3};` | Initialize with elements. |

### 2. Basic Management Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`count()`** | Returns the current number of elements in the array. | `fn count<T>(list: arr<T>): i32` |
| **`capacity()`** | Returns the maximum number of elements the array can hold without reallocating memory. (Optimization tool.) | `fn capacity<T>(list: arr<T>): i32` |
| **`new_with_capacity()`** | Creates an empty array with the specified capacity for optimization. | `fn new_with_capacity<T>(size: i32): arr<T>` |
| **`clear()`** | Deletes all elements of the array. | `fn clear<T>(list: arr<T>): void` |
| **`is_empty()`** | Checks if the array is empty. | `fn is_empty<T>(list: arr<T>): bool` |

### 3. Adding and Removing Elements

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`push()`** | Adds the element to the **end** of the array. | `fn push<T>(list: arr<T>, item: T): void` |
| **`pop()`** | Removes and returns the element at the **end** of the array. (On error, `Result<T, Error>`) | `fn pop<T>(list: arr<T>): Result<T, Error>` |
| **`add()` / `insert()`** | Adds the element at the specified index and shifts subsequent elements. | `fn add<T>(list: arr<T>, index: i32, item: T): void` |
| **`remove()`** | Deletes the element at the specified index and compresses the array. (Returns the deleted element.) | `fn remove<T>(list: arr<T>, index: i32): Result<T, Error>` |

### 4. Searching and Sorting

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`find()`** | Searches for the given value and, if found, returns the first index. | `fn find<T>(list: arr<T>, item: T): Option<i32>` |
| **`sort()`** | Sorts the elements of the array using the default or a custom function. | `fn sort<T>(list: arr<T>, [compare_fn]): void` |
| **`reverse()`** | Reverses the order of the elements of the array. | `fn reverse<T>(list: arr<T>): void` |

### 5. Example Usage: Dynamic Array Operations

```
Nim

void fn ArrayExample() {
var numbers: i32[] = {10, 20, 30}; // Dynamic array initialization

// Adding elements with push
numbers.push(40); // [10, 20, 30, 40]

print("Number of Elements: {numbers.count()}"); // 4

// Insert with add
numbers.add(1, 15); // [10, 15, 20, 30, 40]

// Subtract with pop
var lastElement = numbers.pop();
if lastElement.is_ok(): {
print("Removed: {lastElement.unwrap()}"); // 40
}

// Search
var index = numbers.find(20);
if index.is_some(): {
print("Index of 20: {index.unwrap()}"); // 2
}
}

```

## SECTION 11.4: `string` Module

| Module Name | Description |
| :--- | :--- |
| **`string`** | **String Operations.** Provides helper functions for performing common operations on text (string) variables, such as searching, replacing, formatting, case conversion, and splitting. |

### 1. Basic Information and Conversion Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`len()`** | Returns the number of characters (length) in a string. | `fn len(s: str): i32` |
| **`is_empty()`** | Checks if the string is empty. | `fn is_empty(s: str): bool` |
| **`to_upper()`** | Converts all letters in the string to uppercase. | `fn to_upper(s: str): str` |
| **`to_lower()`** | Converts all letters in the string to lowercase. | `fn to_lower(s: str): str` |
| **`trim()`** | Removes whitespace from the beginning and end of the string. | `fn trim(s: str): str` |
| **`word_count()`** | Returns the number of words in the string (separated by spaces). | `fn word_count(s: str): i32` |

### 2. Search and Comparison Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`contains()`** | Checks whether the string contains the given substring. | `fn contains(s: str, substring: str): bool` |
| **`starts_with()`** | Checks if the string starts with the given substring. | `fn starts_with(s: str, prefix: str): bool` |
| **`ends_with()`**** | Checks if the string ends with the given substring. | `fn ends_with(s: str, suffix: str): bool` |
| **`find()`** | Searches for the given substring in the string and returns its **first index** if found. Otherwise, returns `Option<i32>`. | `fn find(s: str, substring: str): Option<i32>` |

### 3. Manipulation and Parsing Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`replace()`** | Replaces all matching substrings in a string with a new string. | `fn replace(s: str, old: str, new: str): str` |
| **`substring()`** | Returns a substring from the specified starting index (inclusive) and length (or ending index). | `fn substring(s: str, start: i32, length: i32): str` |
| **`split()`** | Splits the string according to the given delimiter and returns a dynamic `array<str>`. | `fn split(s: str, delimiter: str): str[]` |
| **`join()`** | Combines elements in a string array with the given delimiter to form a single string. | `fn join(arr: str[], delimiter: str): str` |

### 4. Example Usage: String Operations

```nim
Nim

void fn StringExample() {
var text: str = "Hello World, NIMBLE is awesome!";

// Basics
print("Length: {text.len()}"); // 28

// Conversion
var upper = text.to_upper();
print("Capitalization: {upper}"); // HELLO WORLD, NIMBLE IS AWESOME!

// Search and Replace
if text.contains("NIMBLE") {
print("Found NIMBLE.");
text = text.replace("wonderful", "excellent");
print("New Text: {text}");
}

// Splitting
var words = text.split(" ");
print("Word Count: {words.count()}"); // 4

// Joining
var newText = string.join(words, "_");
print("Joined: {newText}"); // Hello_World,_NIMBLE_perfect!
}
```


## SECTION 11.5: `math` Module

| Module Name | Description |
| :--- | :--- |
| **`math`** | **Mathematical Functions and Constants.**
Provides high-precision functions and constants for basic trigonometric, logarithmic, exponentiation, and rounding operations.
All functions are designed to work with the `f64` (double-precision float) type. |

### 1. Mathematical Constants

The `math` module provides commonly used mathematical constants as `const`:

| Constant Name | Type | Value |
| :--- | :--- | :--- |
| **`PI`** | `f64` | Pi ($\pi$) |
| **`E`** | `f64` | Euler's number ($e$) |
| **`INF`** | `f64` | Positive Infinity |
| **`NAN`** | `f64` | Not a Number |

### 2. Basic Arithmetic and Exponentiation

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`abs()`** | Returns the absolute value of a number. | `fn abs(x: f64): f64` |
| **`pow()`** | Returns the value of a number (base) raised to another number (power) ($x^y$). | `fn pow(x: f64, y: f64): f64` |
| **`sqrt()`** | Returns the square root of a number ($\sqrt{x}$). | `fn sqrt(x: f64): f64` |
| **`exp()`** | Returns the constant $e$ raised to the given power ($e^x$). | `fn exp(x: f64): f64` |
| **`log()`** | Returns the natural logarithm (base $e$) of a number ($\ln x$). | `fn log(x: f64): f64` |
| **`log10()`** | Returns the logarithm of a number to base 10 ($\log_{10} x$). | `fn log10(x: f64): f64` |

### 3. Rounding and Truncation

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`round()`** | Rounds a number to the nearest integer. (Standard methods, such as rounding to the nearest even number, apply in the case of 0.5). | `fn round(x: f64): f64` |
| **`floor()`** | Rounds a number to the nearest integer less than or equal to itself (round down). | `fn floor(x: f64): f64` |
| **`ceil()`** | Rounds a number to the nearest integer greater than or equal to itself (round up). | `fn ceil(x: f64): f64` |
| **`trunc()`** | Truncates the decimal part of a number (trims it towards zero). | `fn trunc(x: f64): f64` |

### 4. Trigonometry

All trigonometric functions accept input in **radians**.

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`sin()`** | Calculates the sine value. | `fn sin(x: f64): f64` |
| **`cos()`** | Calculates the cosine value. | `fn cos(x: f64): f64` |
| **`tan()`** | Calculates the tangent value. | `fn tan(x: f64): f64` |
| **`atan2()`** | Calculates the arctangent of two variables ($atan2(y, x)$). Used to find the angle on the coordinate plane. | ​​`fn atan2(y: f64, x: f64): f64` |
| **`deg_to_rad()`** | Converts degrees to radians. | `fn deg_to_rad(deg: f64): f64` |
| **`rad_to_deg()`** | Converts radians to degrees. | `fn rad_to_deg(rad: f64): f64` |

### 5. Example Usage: Mathematical Calculations

```nim
Nim

void fn MathExample() {
var radius: f64 = 5.0;

// Constant Usage
var area = math.PI * math.pow(radius, 2.0);
print("Area: {area}"); // ?rn: 78.5398...

// Trigonometry
var degrees: f64 = 45.0;
var radians = math.deg_to_rad(degrees);
var sin_value = math.sin(radian);
print("Sin(45): {sin_value}"); // ?rn: 0.707...

// Square Root and Absolute Value
var val1: f64 = math.sqrt(81.0); // 9.0
var val2: f64 = math.abs(-15.5); // 15.5
print("Square root: {val1}, Absolute: {val2}");

// Rounding
var sqrt: f64 = 4.7;
print("Rounding (ceil): {math.ceil(sqrt)}"); // 5.0
print("Rounding (floor): {math.floor(sqrt)}"); // 4.0
}
```


## SECTION 11.6: `json` Module

| Module Name | Description |
| :--- | :--- |
| **`json`** | **JSON Data Processing.** Provides functions for converting string data in JSON format to NIMBLE's dynamic structures (Parse) and for converting NIMBLE data back to JSON (Stringify). It is critical for communicating with web APIs and managing configuration files. |

### 1. Basic Structures and Types

Because JSON data can be dynamic and complex in nature, the module should use a single dynamic type and handle error conditions with `Result`.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`JsonValue`** | Dynamic type representing any value (object, array, string, number, boolean, null) in JSON. | This is the successful return type of the `json.parse()` function. The content type should be verified by methods such as `as_str()` and `as_i32()`. |
| **`JsonError`** | Lists possible errors that may occur during JSON parsing or stringifying. | `ERR_SYNTAX`, `ERR_INVALID_TYPE` |

### 2. Conversion (Parse and Stringify) Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`parse()`** | Converts a string (str) in JSON format to NIMBLE's dynamic `JsonValue` structure. | `fn parse(json_string: str): Result<JsonValue, JsonError>` |
| **`stringify()`** | Converts a NIMBLE structure (typically `JsonValue`, `map`, or `struct`) to a string in JSON format. | `fn stringify(data: any): Result<str, JsonError>` |
| **`format()`** | Makes the serialized JSON string more readable (indented). | `fn format(json_string: str, indent_spaces: i32): Result<str, JsonError>` |

### 3. `JsonValue` Methods (Data Access)

Since `JsonValue` is a dynamic type, special validation methods (with method chaining support) must be used to access data.

| Function | Purpose | Description |
| :--- | :--- | :--- |
| **`get()`** | Accesses the subvalue by key in a `JsonValue`. (For objects). | `json_val.get("key").as_str().unwrap()` |
| **`at()`** | Accesses the value at the index within a `JsonValue`. (For arrays). | `json_val.at(0).as_i32().unwrap()` |
| **`is_object()`**, `is_array()`, `is_str()`, etc. | Checks the type of the content. | `if json_val.is_object(): ...` |
| **`as_str()`**, `as_i32()`, `as_bool()`, etc. | Converts the content to the specified NIMBLE type (ensuring safety by returning `Option<T>`). | `var name: str = json_val.get("name").as_str().unwrap_or("Unknown");` |

### 4. Example Usage: JSON Parsing

```nim
Nim

void fn JsonExample() {
var json_str: str = "{\"name\":\"NIMBLE\",\"version\":1.0,\"active\":true,\"list\":[10, 20]}";

var parseResult = json.parse(json_str);

match parseResult {
Ok(data) => {
// Safe access to data
var name = data.get("name").as_str().unwrap_or("?");
var version = data.get("version").as_f64().unwrap_or(0.0);

print("Name: {name}"); // NIMBLE
print("Version: {version}"); // 1.0

// Accessing the array
var first_number = data.get("list").at(0).as_i32().unwrap_or(-1);
print("First number in list: {first_number}"); // 10
},
Err(error) => {
print("JSON Parse Error: {error}");
}
}
}
```


## SECTION 11.7: `regex` Module

| Module Name | Description |
| :--- | :--- |
| **`regex`** | **Regular Expressions.** Provides functions and structures for searching, matching, capturing, and replacing complex text patterns. It is built on high-performance and secure pattern matching. |

### 1. Basic Structures and Types

Regular expression operations generally have two stages: compiling the pattern and using the pattern.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Regex`** | Opaque structure representing a compiled regular expression pattern. The pattern is optimized in memory for repeated use. | The successful return type of the `regex.compile()` function. |
| **`Match`** | A structure that holds the result of a successful match and a list of captured groups. | Return type of the `regex.find()` and `regex.match()` functions. |
| **`RegexError`** | Lists syntax errors that may occur while compiling the pattern. | `ERR_SYNTAX`, `ERR_INVALID_FLAG` |

### 2. Basic Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`compile()`** | Compiles the given string pattern into an optimized `Regex` structure. | `fn compile(pattern: str): Result<Regex, RegexError>` |
| **`match()`** | Checks whether the string matches the pattern **from the beginning**. Does not require a full match. | `fn match(r: Regex, text: str): Option<Match>` |
| **`find()`** | Searches for the **first** match of the pattern anywhere in the string. | `fn find(r: Regex, text: str): Option<Match>` |
| **`replace()`** | Replaces the first match of the pattern with the given string and returns the new string. | `fn replace(r: Regex, text: str, replacement: str): str` |
| **`replace_all()`** | Replaces all** matches of the pattern in the string with the given string and returns the new string. | `fn replace_all(r: Regex, text: str, replacement: str): str` |

### 3. Methods of the `Match` Structure (Accessing Captures)

The `Match` structure provides access to the result data when the match is successful.

| Function | Purpose | Description |
| :--- | :--- | :--- |
| **`is_success()`** | Checks if the match was successful. | `if myMatch.is_success(): ...` |
| **`text()`** | Returns the entire text matched. | `myMatch.text()` |
| **`group()`** | Returns the string of the capturing group at the specified index (0: Exact match, 1+: Capturing groups). | `myMatch.group(1)` |
| **`start()`**, **`end()`** | Returns the starting and ending indices of the match in the string. | `print(myMatch.start())` |

### 4. Example Usage: Email Validation and Capture

```nim
Nim

void fn RegexExample() {
// Pattern: Username, @, domain (.extension)
var email_pattern: str = "^([a-zA-Z0-9._%+-]+)@([a-zA-Z0-9.-]+)\\.([a-zA-Z]{2,})$";
var test_email: str = "username@nimble.dev";

// 1. Compile the Pattern
var regexResult = regex.compile(email_pattern);

match regexResult {
Ok(email_regex) => {
// 2. Find a Match
var matchOption = email_regex.find(test_email);

match matchOption {
Some(match_data) => {
print("Match Successful.");
// Access Capture Groups
var user = match_data.group(1);
var domain = match_data.group(2);

print("User Name: {user}"); // username
print("Domain Name: {domain}"); // nimble
},
None => {
print("Email format is incorrect.");
}
}

// 3. Replacement
var old_text: str = "Date 2024-01-01";
var date_regex = regex.compile("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
var new_text = date_regex.replace(old_text, "NEW DATE");
print("Changed: {new_text}"); // Date NEW DATE
},
Err(error) => {
print("Pattern Compilation Error: {error}");
}
}
}
```

## SECTION 11.8: `io` Module

| Module Name | Description |
| :--- | :--- |
| **`io`** | **Standard Input/Output (Console I/O).** Provides basic functions for programs to receive data from the user (`stdin`) and display data to the user (`stdout`, `stderr`) via the terminal/console. |

### 1. Standard Output Functions (`stdout`)

These functions write data to the program's standard output stream (usually the terminal).

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`print()`** | Writes the given text or variable to standard output. **Does not add a newline.** | `fn print(data: any): void` | `io.print("Hello");` |
| **`println()`** | Writes the given text or variable to standard output and **follows it with a line break.** | `fn println(data: any): void` | `io.println("World!");` |
| **`flush()`** | Forces any pending data in the output buffer to be written to the screen immediately. | `fn flush(): void` | Performance is critical. |
| **`err_print()`** | Writes the given text to the **Standard Error Output** (`stderr`) stream. Used for error messages. | `fn err_print(data: any): void` | `io.err_print("An error occurred!");` |

### 2. Standard Input Functions (`stdin`)

These functions are used to receive input from the user.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`input()`** | Reads a line of text from the user and returns it as a string (`str`). If the read fails, returns `Result<str, Error>`. | `fn input(): Result<str, IoError>` | `var input = io.input();` |
| **`prompt()`** | Displays a prompt before taking input from the user and then reads the input. | `fn prompt(message: str): Result<str, IoError>` | `var name = io.prompt("Your name: ");` |
| **`read_char()`** | Reads only a single character from the user. | `fn read_char(): Result<char, IoError>` | `var selection = io.read_char();` |

### 3. Error Management

Because standard I/O operations can also be interrupted, error types are defined.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`IoError`** | Lists errors that may occur in I/O operations. | `ERR_READ_FAILED`, `ERR_EOF` (End of File) |

### 4. Example Usage: Receiving Data from the User

```nim
Nim

void fn ConsoleExample() {
// 1. Receiving input from the user via a prompt
var isimResult = io.prompt("Please enter your name: ");

match isimResult {
Ok(name) => {
io.println("Hello, {name}!");

// 2. Receiving and converting numeric input
var yasResult = io.prompt("Please enter your age: ");

match yasResult {
Ok(yas_str) => {
// Converting a string to an integer
// Assuming there is a parse function in NIMBLE's string module or in the basic language.
var yas: i32 = string.to_i32(yas_str).unwrap_or(0);
io.println("So, you are {yas} years old.");
},
Err(error) => {
io.err_print("An error occurred while reading the input.");
}
}
},
Err(error) => {
io.err_print("Could not read name.");
}
}
}
```

## SECTION 11.9: `types` Module

| Module Name | Description |
| :--- | :--- |
| **`types`** | **Type Conversions and Checking.** Provides tools for safe (explicit) and unsafe (unsafe) conversions between different data types (integer, floating-point, string, etc.). Safe conversions are designed to prevent data loss. |

### 1. Safe Casting

Used when the risk of data loss is high or when the conversion must be explicitly specified. On failure (for example, in a string-to-number conversion), it returns `Result<T, E>` or `Option<T>`.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`to_i32()`** | Converts a value (e.g., `str`, `f64`) to a 32-bit integer (`i32`). | `fn to_i32(val: any): Result<i32, ConvertError>` | Provides error checking for string-to-number conversion. |
| **`to_f64()`** | Converts a value to a 64-bit floating point (`f64`). | `fn to_f64(val: any): Result<f64, ConvertError>` | |
| **`to_str()`** | Converts a value (e.g., `i32`, `f64`, `bool`) to a string (`str`). (Usually done automatically, but provided for manual checking.) | `fn to_str(val: any): str` | This conversion always succeeds. |
| **`parse()`** | Converts from a string to a specified target type. The type is specified with the Generic `<T>`. | `fn parse<T>(s: str): Result<T, ConvertError>` | `var num = types.parse<i32>("123");` |

### 2. Unsafe/Direct Casting

Instructs the compiler to directly convert types, ignoring the risk of data loss or incorrect interpretation. Used in low-level system programming requiring high performance.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`as_type()`** | Force-converts a value to the specified target type. | `fn as_type<T>(val: any): T` | `var val_i32 = val_f64.as_type<i32>();` (Decimal part may be truncated!) |
| **`cast()`** | This allows only the bits of data in memory to be interpreted as a different type (reinterpretation/bitwise cast). This is very dangerous. | `fn cast<T>(val: any): T` | For example, reading the bits of an `i32` as `f32`. |

### 3. Type Checking and Verification

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`is_type()`** | Checks whether the runtime type of a variable is the specified type. | `fn is_type<T>(val: any): bool` | `if myVar.is_type<str>(): ...` |
| **`name_of()`** | Returns the runtime type name of a variable as a string. | `fn name_of(val: any): str` | `print(types.name_of(myVar)); // "i32"` |

### 4. Error Handling

Error type for safe conversion functions:

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`ConvertError`** | Lists errors that may occur during type conversion. | `ERR_INVALID_FORMAT` (String is not a number), `ERR_OVERFLOW` (Does not fit in target type) |

### 5. Example Usage: Safe and Unsafe Conversions

```nim
Nim

void fn TypesExample() {
var float_val: f64 = 123.45;
var string_val: str = "42";
var bad_string: str = "abc";

// Safe Conversion (f64 -> i32)
// Should be explicit due to the risk of data loss, but Result is unnecessary in this case.
var int_val_safe: i32 = types.to_i32(float_val).as_type<i32>(); // 123
print("Safe Conversion (int): {int_val_safe}");

// Safe String Parsing (str -> i32)
var parseResult = types.parse<i32>(string_val);
match parseResult {
Ok(i) => { print("Successful parse: {i}"); }, // 42
Err(e) => { print("Error: {e}"); }
}

// Failed String Parsing
var badParse = types.parse<i32>(bad_string);
if badParse.is_err() {
print("Error: Could not convert 'abc' to a number.");
}

// Unsafe/Forced Conversion (Risk of data loss)
var large_i32: i32 = 500;
// Forced conversion from float_val to i32 (as_type) truncates the decimal.
var forced_i32: i32 = float_val.as_type<i32>();
print("Forced Conversion (truncating): {forced_i32}"); // 123
}
```

## SECTION 11.10: `memory` Module

| Module Name | Description |
| :--- | :--- |
| **`memory`** | **Manual Memory Management.** Provides the programmer with low-level operations such as dynamic memory allocation (similar to `malloc`/`calloc`), resizing, and freeing (`free`).

Critical for scenarios where automatic memory management (GC) is not used or is bypassed. |

### 1. Basic Structures and Types

Only pointers and sizes are used in memory operations.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`ptr`** | An unsafe pointer to memory of any type.
| ​​**`size`** | A platform-specific unsigned integer that holds the memory size (in bytes). | Used in functions such as `alloc` and `copy`. |
| **`MemError`** | Lists errors that may occur during memory allocation. | `ERR_NO_MEMORY` (Insufficient system memory), `ERR_INVALID_PTR` |

### 2. Memory Allocation and Deallocation Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`alloc()`** | Allocates memory of the requested size (in bytes). Returns `ptr` on success; returns `null` or an error code on failure. | `fn alloc(size: size): Result<ptr, MemError>` | `ptr ptr1 = memory.alloc(1024);` |
| **`calloc()`** | Allocates memory for the requested number of elements and fills all allocated memory with zeros. | `fn calloc(amount: size, element_size: size): Result<ptr, MemError>` | `ptr ptr2 = memory.calloc(10, types.sizeof<i32>());` (Reserves space for 10 i32s) |
| **`realloc()`** | Resizes the previously allocated memory. Data is preserved. | `fn realloc(ptr: ptr, new_size: size): Result<ptr, MemError>` | `ptr1 = memory.realloc(ptr1, 2048);` |
| **`free()`** | Frees memory previously allocated by `alloc` or `calloc`. | `fn free(ptr: ptr): void` | After being freed, the pointer becomes invalid. |

### 3. Memory Manipulation Functions

These functions are used to move or fill memory in blocks.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`copy()`** | Copies data of the specified size from the source memory block to the destination memory block. | `fn copy(dest: ptr, src: ptr, size: size): void` | `memory.copy(ptr_target, ptr_source, 500);` |
| **`move()`** | Same as `copy()`, but safe for overlapping memory blocks. | `fn move(dest: ptr, src: ptr, size: size): void` | |
| **`set()`** | Fills the entire memory block with a specified single byte value (e.g., for reset). | `fn set(ptr: ptr, value: u8, size: size): void` | `memory.set(ptr1, 0, 1024);` (Reset memory) |

### Example 4 Usage: Manual Memory Management

```
Nim

// It is assumed that there is a default function in the language to get the size of the i32 in memory.
const I32_SIZE = types.sizeof:i32();

void fn MemoryExample() {
var size_bytes: size = 10 * I32_SIZE; // Space for 10 i32s
var data_ptr: ptr;

// 1. Memory Allocation
var allocResult = memory.calloc(10, I32_SIZE);

match allocResult {
Ok(ptr_val) => {
data_ptr = ptr_val;

// 2. Using Memory (Pointer Arithmetic and Unsafe Access)
// Assign the value 42 to the first i32
(data_ptr as *i32)[0] = 42;

// 3. Resizing Memory (e.g., for 20 i32s)
var reallocResult = memory.realloc(data_ptr, 20 * I32_SIZE);

match reallocResult {
Ok(new_ptr) => {
data_ptr = new_ptr;
print("Memory resized. Initial value: { (data_ptr as *i32)[0] }");
},
Err(e) => {
print("Resizing failed: {e}");
}
}
},
Err(e) => {
print("Memory allocation failed: {e}");
}
}

// 4. Freeing Memory (Absolutely necessary!)
memory.free(data_ptr);
print("Memory freed.");
}
```


## SECTION 11.11: `gpu` Module

| Module Name | Description |
| :--- | :--- |
| **`gpu`** | **GPU Computing and Parallel Processing.** Provides tools and structures for exploiting the highly parallel computing power of Graphics Processing Units (GPUs). Enables the definition and execution of data-parallel algorithms on the GPU. |

### 1. Basic Structures and Types

GPU computing revolves around GPU memory and kernel functions (kernels), which are separate from CPU memory.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Device`** | Represents a physical GPU device in the system. | Obtained with `gpu.select_device()`. |
| **`Kernel`** | Represents a compiled function (kernel) to be run in parallel on the GPU. | Created with `gpu.compile_kernel()`. |
| **`GpuArray<T>`** | An array of the specified type (`T`) allocated in GPU memory. Used for data transfer. | The GPU equivalent of dynamic arrays (`arr<T>`) in the CPU. |
| **`GpuError`** | Lists errors that may occur during GPU initialization, memory allocation, or kernel execution. | `ERR_NO_DEVICE`, `ERR_COMPILATION_FAILED` |

### 2. Management and Compilation Functions

| Function | Purpose | Syntax |
| :--- | :--- | :--- |
| **`select_device()`** | Selects an available GPU device in the system. | `fn select_device(index: i32): Result<Device, GpuError>` |
| **`compile_kernel()`** | Compiles a NIMBLE function into a GPU-executable Kernel. | `fn compile_kernel(func_ref): Result<Kernel, GpuError>` |
| **`sync()`** | Waits for GPU operations to complete and returns control to the CPU. | `fn sync(device: Device): void` |
| **`error_code()`** | Returns the error of the last GPU operation. | `fn error_code(): GpuError` |

### 3. Memory Management and Data Transfer

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`alloc_array()`** | Allocates a new `GpuArray` in GPU memory. | `fn alloc_array<T>(device: Device, size: i32): Result<GpuArray<T>, GpuError>` | `var gpu_data = gpu.alloc_array<f64>(dev, 1000);` |
| **`to_gpu()`** | Copies an array (`arr<T>`) from CPU memory to GPU memory. | `fn to_gpu<T>(device: Device, cpu_array: arr<T>): Result<GpuArray<T>, GpuError>` | |
| **`from_gpu()`** | Copies an array from GPU memory back to CPU memory. | `fn from_gpu<T>(gpu_array: GpuArray<T>): Result<arr<T>, GpuError>` | |
| **`free_array()`** | Frees an array allocated in GPU memory. | `fn free_array(gpu_array: GpuArray): void` | Requires manual memory management. |

### 4. Kernel Definition and Execution

Functions that will run on the GPU are defined with the special keyword `kernel`.

| Structure/Function | Purpose | Description |
| :--- | :--- | :--- |
| **`kernel fn`** | Declaration of a function that will run in parallel on the GPU.
| Can only take `GpuArray` and basic types as parameters. |
| **`launch()`** | Executes (starts) a compiled `Kernel` on the GPU. Grid/Block sizes are specified.
| `fn launch(k: Kernel, grid_size: i32, block_size: i32, ...args): void` |
| **`gpu.thread_id()`** | Returns the parallel index of the current thread within the running kernel.
| Used within kernel functions. |

### 5. Example Usage: Adding Two Arrays

```nim
Nim

// Kernel function to run in parallel on the GPU
kernel fn add_arrays(a: GpuArray<i32>, b: GpuArray<i32>, result: GpuArray<i32>) {
// Get the index of the current thread
var i = gpu.thread_id();

// Parallel operation: result[i] = a[i] + b[i]
if i < a.count() {
result[i] = a[i] + b[i];
}
}

void fn GpuExample() {
var deviceResult = gpu.select_device(0); // Select first GPU 

match deviceResult { 
Ok(dev) => { 
var size = 1024; 
var cpu_a: i32[] = array.new_filled(size, 1); 
var cpu_b: i32[] = array.new_filled(size, 2); 

// 1. Transfer data to GPU 
var gpu_a = gpu.to_gpu(dev, cpu_a).unwrap(); 
var gpu_b = gpu.to_gpu(dev, cpu_b).unwrap(); 
var gpu_result = gpu.alloc_array<i32>(dev, size).unwrap(); 

// 2. Compile Kernel 
var kernel = gpu.compile_kernel(add_arrays).unwrap(); 

// 3. Start the Kernel (e.g., 1024 threads)
gpu.launch(kernel, size, 256, gpu_a, gpu_b, gpu_result);

// 4. Wait for the Process to Complete and Retrieve the Data
gpu.sync(dev);
var final_result = gpu.from_gpu(gpu_result).unwrap();

print("GPU Process Finished. First Element: {final_result[0]}"); // Should be 3

// 5. Free the Memory (Very Important!)
gpu.free_array(gpu_a);
gpu.free_array(gpu_b);
gpu.free_array(gpu_result);
},
Err(e) => {
io.err_print("GPU device not found: {e}");
}
}
}
```

## SECTION 11.12: `net` Module

| Module Name | Description |
| :--- | :--- |
| **`net`** | **Network and Socket Communication.** Provides tools for low-level network communication using TCP (reliable) and UDP (fast, insecure) protocols. All functions return the **`Result<T, E>`** type to handle connection and transmission errors. |

### 1. Basic Structures and Types

Basic structures representing sockets and addresses for network communication.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Socket`** | Opaque structure representing a network port (socket). Similar to a file handle (`FileHandle`). | Created with `net.tcp_open()` or `net.udp_open()`. |
| **`Protocol`** | Constants that specify the network protocol to use. | `net.TCP`, `net.UDP` |
| **`Address`** | A structure representing an IP address and port number. | `net.Address { ip: "127.0.0.1", port: 8080 }` |
| **`NetError`** | Lists errors that may occur during network initialization, connection establishment, sending, or receiving. | `ERR_CONNECTION_REFUSED`, `ERR_TIMEOUT`, `ERR_ADDR_IN_USE` |

### 2. Socket Management Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`tcp_connect()`** | Establishes an outgoing TCP connection to the specified remote address (IP and Port). | `fn tcp_connect(address: Address): Result<Socket, NetError>` | Used by the client. |
| **`tcp_listen()`** | Creates a TCP socket to listen for new connections at the specified local address (Port). | `fn tcp_listen(address: Address): Result<Socket, NetError>` | Used by the server. |
| **`tcp_accept()`** | Accepts a new incoming connection on the listening socket and returns a new data socket. | `fn tcp_accept(listener: Socket): Result<Socket, NetError>` | Blocks (waits) when called by the server. |
| **`udp_open()`** | Opens a UDP socket to send/receive data at the specified address (Port). | `fn udp_open(address: Address): Result<Socket, NetError>` | |
| **`close()`** | Closes an open socket and releases system resources. | `fn close(socket: Socket): void` | Required after socket operations. |

### 3. Data Transfer Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- | :--- |
| **`send()`** | Sends data (byte array or string) over the connected socket. Returns the number of bytes sent. | `fn send(s: Socket, data: any): Result<i32, NetError>` | |
| **`recv()`** | Reads data from the socket. Returns the data (byte array or string) read. Specifies the maximum size to read. | `fn recv(s: Socket, max_size: i32): Result<str, NetError>` | This is a blocking function. |
| **`send_to()`** | Sends data to the specified destination using a UDP socket. | `fn send_to(s: Socket, address: Address, data: any): Result<i32, NetError>` | |
| **`recv_from()`** | Reads data from the UDP socket and returns the address information that sent the data. | `fn recv_from(s: Socket, max_size: i32): Result<Tuple<str, Address>, NetError>` | |

### 4. Example Usage: Simple TCP Client

```nim
Nim

void fn TcpClientExample() {
var server_addr = net.Address { ip: "127.0.0.1", port: 8080 };

// 1. Connect to Server
var connectResult = net.tcp_connect(server_addr);

match connectResult {
Ok(sock) => {
io.println("Successfully connected to the server.");

// 2. Send Data
var data_to_send: str = "Hello Server!";
var sendResult = net.send(sock, data_to_send);

if sendResult.is_ok() {
io.println("{sendResult.unwrap()} bytes sent.");
}

// 3. Receive Data
var recvResult = net.recv(sock, 1024); // Read a maximum of 1024 bytes

if recvResult.is_ok() {
io.println("Received from the server: {recvResult.unwrap()}");
} else {
io.err_print("Error receiving data.");
}

net.close(sock);
},
Err(e) => {
io.err_print("Connection Error: {e}");
}
}
}
```

## SECTION 11.13: `time` Module

| Module Name | Description |
| :--- | :--- |
| **`time`** | **Time and Date Management.**
Provides basic tools for interacting with the system clock, creating timestamps, calculating durations, and formatting date/time.
|

### 1. Basic Structures and Types

Time must be stored in different precision and formats.

| Type Name | Purpose | Description |
| :--- | :--- | :---
|
| **`Timestamp`** | A numeric value representing a specific moment, usually the number of seconds since the Unix Epoch (January 1, 1970).
| Can be stored as `i64` or `f64`.
|
| **`DateTime`** | Structured date and time, divided into components such as year, month, day, hour, minute, second.
| Used for formatting and arithmetic operations.
|
| **`Duration`** | A value representing the duration (difference) between two instants (usually in seconds or milliseconds).
| Used for performance measurements and sleep times.
|

### 2. Instantaneous Time Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :---
|
| **`now()`** | Returns the timestamp (`Timestamp`) of the current instant. High precision.
| `fn now(): Timestamp` |
|
| **`utc_now()`** | Returns the `DateTime` structure of the current instant in UTC (Universal Time Coordinate).
| `fn utc_now(): DateTime` |
|
| **`local_now()`** | Returns the `DateTime` structure of the current moment according to the system's local time zone.
| `fn local_now(): DateTime` |
|
| **`sleep()`** | Pauses thread execution for the specified amount of time (milliseconds).
| `fn sleep(ms: i32): void`
| Often found in the `thread` module, but presented here for basic latency.
|

### 3. Conversion and Formatting Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :---
|
| **`from_timestamp()`** | Converts a `Timestamp` value to a `DateTime` structure.
| `fn from_timestamp(ts: Timestamp): DateTime` |
|
| **`format()`** | Converts a `DateTime` structure to a string (`str`) according to the specified format.
| `fn format(dt: DateTime, format_str: str): str`
| The format string follows standards such as `%Y-%m-%d %H:%M:%S`.
|
| **`parse()`** | Parses a string in the specified format into a `DateTime` structure.
| `fn parse(time_str: str, format_str: str): Result<DateTime, TimeError>` |
|

### 4. Time Measurement Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`measure_start()`** | Starts the high-resolution timer (for time measurement).
| `fn measure_start(): Timestamp` |
|
| **`measure_end()`** | Measures the duration starting with `measure_start()` and returns it as `Duration`.
| `fn measure_end(start_time: Timestamp): Duration` |
|
| **`duration_ms()`** | Returns a `Duration` value in milliseconds as `f64`.
| `fn duration_ms(d: Duration): f64` |
|

### 5. Example Usage: Duration Measurement and Formatting

```nim
Nim

void fn TimeExample() {
// 1. Duration Measurement (Performance Test)
var start_time = time.measure_start();

// Simulated Long Process
var i: i32 = 0;
while i < 10000000 {
i = i + 1;
}

var duration = time.measure_end(start_time);
var duration_ms = time.duration_ms(duration);
io.println("Processing duration: {time_ms} milliseconds.");

// 2. Current Time and Formatting
var now_utc = time.utc_now();

// Year-Month-Day Hour:Minute:Second format
var formatted_time = time.format(now_utc, "%Y-%m-%d %H:%M:%S");
io.println("UTC Time: {formatted_time}");

// 3. Sleep
io.println("Waiting for 200 ms...");
time.sleep(200);
io.println("Waiting is over.");
}
```


## SECTION 11.14: `os` Module

| Module Name | Description |
| :--- | :--- |
| **`os`** | **Operating System Interaction.** Provides functions to perform operating system-specific tasks such as managing environment variables, file path manipulation, directory operations, accessing command-line arguments, and managing external processes. |

### 1. Command Line and Environment Variables

Provides information about how the program is started and its environment configuration.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`args()`** | Returns the **command-line arguments** with which the program is run as a string array (`str[]`). (The first element is the program name.) | `fn args(): str[]` | |
| **`get_env()`** | Returns the value of the environment variable with the specified name as a string. If the variable is not defined, `Option<str>` is returned. | `fn get_env(name: str): Option<str>` | |
| **`set_env()`** | Defines a new environment variable or updates an existing one. | ​​`fn set_env(name: str, value: str): void` | |
| **`pid()`** | Returns the ID of the currently running process. | `fn pid(): i32` | |
| **`exec_path()`** | Returns the full executable path of the running program. | `fn exec_path(): str` | |

### 2. Directory and File Path Operations

The `file` module deals with file contents, while the `os` module deals with file paths and directory hierarchy.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`cwd()`** | Returns the program's current working directory. | `fn cwd(): str` | |
| **`chdir()`** | Changes the program's working directory to the specified path. | `fn chdir(path: str): Result<void, OsError>` | |
| **`mkdir()`** | Creates a new directory. | `fn mkdir(path: str): Result<void, OsError>` | |
| **`rmdir()`** | Deletes an empty directory. | `fn rmdir(path: str): Result<void, OsError>` | |
| **`join_path()`** | Joins path segments using separators appropriate to the operating system. | `fn join_path(segments: str[]): str` | `os.join_path({"user", "data", "file.txt"})` |
| **`list_dir()`** | Returns all file and directory names in the specified directory as a string array. | `fn list_dir(path: str): Result<str[], OsError>` | |

### 3. Process Management and Execution

Used to run and control external programs.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`execute()`** | Executes an external program (command) and **waits for the process to complete** (Blocks). Returns the process's exit code. | `fn execute(command: str, args: str[]): Result<i32, OsError>` | `os.execute("git", {"clone", "repo_url"})` |
| **`spawn()`** | Starts an external program in the background and immediately returns control (non-blocking). | `fn spawn(command: str, args: str[]): Result<ProcessHandle, OsError>` | Used for background services. |
| **`exit()`** | Terminates the program with the specified exit code. | `fn exit(code: i32): void` | |

### 4. Example Usage: Command-Line Arguments and Directory Operations

```nim
Nim

// To run this program: ./programim.nim --input data.txt

void fn OsExample() {
// 1. Accessing Command-Line Arguments
var args = os.args();
io.println("Program Name: {args[0]}");

if args.count() > 1 {
io.println("First Argument: {args[1]}"); // --input
}

// 2. Accessing Environment Variables
var userEnv = os.get_env("USER");
match userEnv {
Some(user) => {
io.println("User: {user}");
},
None => {
io.err_print("USER environment variable not found.");
}
}

// 3. Executing External Commands
var exitCodeResult = os.execute("ls", {"-l"}); // For Unix/Linux
// Or os.execute("cmd", {"/c", "dir"}); // For Windows

if exitCodeResult.is_ok() && exitCodeResult.unwrap() == 0 {
io.println("The directory listing command ran successfully.");
}

// 4. Creating a Directory
var newDir = "test_data";
if os.mkdir(newDir).is_ok() {
io.println("The directory '{newDir}' was created.");
}
}

```

## SECTION 11.15: `log` Module

| Module Name | Description |
| :--- | :--- |
| **`log`** | **In-Application Logging and Tracing.** Provides an interface for logging runtime events, errors, and debug information using structured levels. Log output can be redirected to various destinations such as the console/terminal, file, or network. |

### 1. Basic Structures and Types

Logging requires standard levels to categorize messages.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Level`** | Constants that determine the severity of the log message. | `log.DEBUG`, `log.INFO`, `log.WARN`, `log.ERROR`, `log.FATAL` |
| **`Logger`** | Structure that holds logging settings (level, format, target). Enables logging to multiple targets. | Created with `log.new()`. |
| **`Target`** | Structure that specifies the target (file, console, network) to which logs will be written. | `log.TARGET_CONSOLE`, `log.TARGET_FILE` |

### 2. Configuration and Management Functions

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`new()`** | Creates a new `Logger` instance. | `fn new(target: Target): Logger` | |
| **`set_level()`** | Sets the logger's minimum logging level. Messages with lower severity are ignored. | `fn set_level(level: Level): void` | `log.set_level(log.INFO);` |
| **`set_format()`** | Sets a format template that specifies how log messages will appear in the output. | `fn set_format(format_str: str): void` | E.g.: `"{time} [{level}] {message}"` |
| **`add_target()`** | Adds an additional target (e.g., a file) to which logs will be written. | `fn add_target(l: Logger, target: Target): Result<void, LogError>` | |

### 3. Logging Functions

These functions log messages according to their severity level.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`debug()`** | Logs debugging or fine-grained trace information. | `fn debug(msg: str): void` | Useful only during development. |
| **`info()`** | Records information about the general application flow. | `fn info(msg: str): void` | `log.info("User logged in.");` |
| **`warn()`** | Records potential problems or unexpected situations, but does not prevent the program from running. | `fn warn(msg: str): void` | |
| **`error()`** | Records an application error or a recoverable problem. | `fn error(msg: str): void` | `log.error("Error opening file.");` |
| **`fatal()`** | Records critical, irreversible errors that require the program to terminate. | `fn fatal(msg: str): void` | Usually terminated with `os.exit()`. |

### 4. Example Usage: Structured Logging

```nim
Nim

void fn LogExample() {
// 1. Configuring the Default Logger
log.set_level(log.INFO);
log.set_format("[{time} - {level}] {message}");

// Console Output
log.info("Starting application successfully...");

// Output: [2025-11-08 02:11:00 - INFO] Starting application successfully...

// The DEBUG message is ignored because it is lower than the set level.
log.debug("Database connection attempt.");

// 2. Creating a Custom Logger (Logging to File)
var fileTarget = log.Target { type: log.TARGET_FILE, path: "app.log" };
var fileLogger = log.new(fileTarget);
fileLogger.set_level(log.DEBUG); // Log everything to a file

// 3. Using a Custom Logger
if some_condition.is_false() {
var error_message: str = "Configuration file missing.";

log.warn(error_message); // Writes WARN to the console
fileLogger.error(error_message); // Writes ERROR to the file
}

// Critical Error
log.fatal("Could not allocate memory. Terminating program.");
}
```


## SECTION 11.16: `rand` Module

| Module Name | Description |
| :--- | :--- |
| **`rand`** | **Random Number Generation.** Provides functions and structures for generating both standard (fast, deterministic) and cryptographically secure (non-deterministic) random numbers according to application needs. |

### 1. Basic Structures and Types

A random number generator (RNG) maintains a state, and this state must be managed.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Rng`** | Example of a standard, fast random number generator. Typically implemented with an algorithm (such as Xoroshiro, Mersenne Twister). | Generated with `rand.new_rng()`. |
| **`SecureRng`** | Cryptographically secure random number generator (CSPRNG). Uses operating system resources for security and key generation. | Created with `rand.new_secure()`. |
| **`Seed`** | Value that determines the initial state of the generator. The same seed produces the same random number sequence. | |

### 2. Generator Management

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- | :--- |
| **`new_rng()`** | Creates a new fast `Rng` instance using the system's current time (or a default path). | `fn new_rng(): Rng` | |
| **`new_rng_seeded()`** | Creates a deterministic `Rng` instance using a specified `Seed` value. | `fn new_rng_seeded(seed: Seed): Rng` | Used for repeatable simulations. |
| **`new_secure()`** | Creates a new cryptographically secure `SecureRng` instance. | `fn new_secure(): SecureRng` | |
| **`set_seed()`** | Reset the state of an existing `Rng` instance to a new seed. | `fn set_seed(r: Rng, seed: Seed): void` | |

### 3. Random Value Generation Functions

Generating functions must be callable on both `Rng` and `SecureRng`.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`i32()`** | Returns a random 32-bit integer (`i32`). | `fn i32(r: Rng): i32` | |
| **`f64()`** | Returns a random float (`f64`) between 0.0 (inclusive) and 1.0 (exclusive). | `fn f64(r: Rng): f64` | |
| **`range_i32()`** | Returns a random `i32` between the specified `min` (inclusive) and `max` (exclusive) bounds. | `fn range_i32(r: Rng, min: i32, max: i32): i32` | |
| **`choice()`** | Selects a random element from the given array. | `fn choice<T>(r: Rng, arr: T[]): T` | |
| **`bytes()`** | Returns a random byte array (`u8[]`) of the desired length for cryptographic purposes. (Typically used with `SecureRng`). | `fn bytes(r: SecureRng, size: i32): u8[]` | |

### 4. Example Usage: Simulation and Security

```nim
Nim

void fn RandExample() {
// 1. Standard RNG (Simulation/Game)
var fast_rng = rand.new_rng();

// Generate random numbers from 1 to 100 (excluding 100)
var dice_horse = fast_rng.range_i32(1, 100);
io.println("Random Number: {dice_horse}");

// Floating-point generation
var rate = fast_rng.f64();
io.println("Floating Rate: {rate}");

// Select from array
var names: str[] = {"Ali", "Veli", "Ayşe", "Fatma"};
var lucky_name = fast_rng.choice(names);
io.println("Lucky Name: {lucky_name}");

// 2. Secure RNG (Cryptographic)
var secure_rng = rand.new_secure();

// Generates a 32-byte secure key
var key_bytes = secure_rng.bytes(32);
io.println("Length of Generated Secure Key: {key_bytes.count()}");

// The key is usually processed as a byte array, not a string.
}
```


## SECTION 11.17: `crypto` Module

| Module Name | Description |
| :--- | :--- |
| **`crypto`** | **Cryptography and Security.** Provides basic cryptographic functions such as hashing (digesting) to ensure data integrity, symmetric/asymmetric encryption to ensure data confidentiality, and secure key generation. It uses high-security and industry-standard algorithms. |

### 1. Basic Structures and Types

In the cryptography module, data is typically processed as byte arrays.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Digest`** | Byte array (`u8[]`) holding the output of the hash function (digest). | Used to verify data integrity. |
| **`Cipher`** | Symmetric or asymmetric encryption/decryption key. | Required for encryption algorithms. |
| **`Algorithm`** | Constants specifying the algorithm to be used. | `crypto.SHA256`, `crypto.AES256`, `crypto.RSA_2048` |

### 2. Hashing and Hashing Functions

These functions are used to prove the integrity of data and store passwords.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`hash()`** | Returns the cryptographic digest (`Digest`) of the input data using the given algorithm (e.g., SHA256). | `fn hash(alg: Algorithm, data: u8[]): Digest` | |
| **`hmac()`** | Uses a key and a hash algorithm (Keyed-Hash Message Authentication Code) to verify the integrity of a piece of data. | `fn hmac(alg: Algorithm, key: u8[], data: u8[]): Digest` | |
| **`pbkdf2()`** | Password-based key generation function. Ideal for slow hashing (with salting and iterations) passwords before storing them. | `fn pbkdf2(password: str, salt: u8[], iterations: i32, length: i32): u8[]` | |

### 3. Symmetric Encryption (AES)

Performs both encryption and decryption using a secret key.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`generate_key()`** | Generates a new secure symmetric key (`Cipher`) for the specified algorithm. | `fn generate_key(alg: Algorithm): Cipher` | Usually 128/256 bits long. |
| **`encrypt_aes()`** | Encrypts data with the AES algorithm. A mode like GCM/CBC is required. | `fn encrypt_aes(key: Cipher, iv: u8[], data: u8[]): Result<u8[], CryptoError>` | `iv` (Initialization Vector) is required. |
| **`decrypt_aes()`** | Decrypts data encrypted with the AES algorithm. | `fn decrypt_aes(key: Cipher, iv: u8[], encrypted_data: u8[]): Result<u8[], CryptoError>` | |

### 4. Asymmetric Encryption and Signatures (RSA/ECC)

Public and private key pairs are used.

| Function | Purpose | Syntax | Description |
| :--- | :--- | :--- | :--- |
| **`generate_keypair()`** | Generates a public and private key pair for asymmetric encryption. | `fn generate_keypair(alg: Algorithm): Tuple<Cipher, Cipher>` | Like `crypto.RSA_2048`. |
| **`sign()`** | Signs data with the private key. | `fn sign(private_key: Cipher, data: u8[]): u8[]` | |
| **`verify()`** | Verifies the authenticity of data using the public key and signature. | `fn verify(public_key: Cipher, data: u8[], signature: u8[]): bool` | |

### 5. Example Usage: Data Integrity and Encryption

```nim
Nim

void fn CryptoExample() {
var password = "my_secret_password";
var data: u8[] = string.to_bytes("Secret message."); // Converting data to a byte array

// 1. Hashing (Data Integrity)
var hash_digest = crypto.hash(crypto.SHA256, data);
io.println("SHA-256 Digest: {hash_digest.to_hex()}");

// 2. Password Security (PBKDF2)
var salt = rand.new_secure().bytes(16); // Secure, random salt
var derived_key = crypto.pbkdf2(password, salt, 100000, 32);
io.println("Derived Key (32 bytes): {derived_key.count()}");

// 3. Symmetric Encryption (AES-256)
var aes_key = crypto.generate_key(crypto.AES256);
var iv = rand.new_secure().bytes(16); // Unique IV

// Encryption
var cipher_result = crypto.encrypt_aes(aes_key, iv, data);

match cipher_result {
Ok(encrypted) => {
io.println("Encrypted Data Length: {encrypted.count()}");

// Decryption 
var decrypt_result = crypto.decrypt_aes(aes_key, iv, encrypted); 
if decrypt_result.is_ok() { 
var decrypted_data = decrypt_result.unwrap(); 
io.println("Decrypted Message: {string.from_bytes(decrypted_data)}"); 
} 
}, 
Err(e) => { 
io.err_print("Encryption Error: {e}"); 
} 
}
}
```

## SECTION 11.18: The `map` Module
The Key-Value Map is a `[Dictionary / Associative Array]`.

It is a dynamic collection that stores data of different types for quick access using unique keys `[key]`.

1. Definition Syntax [Using Generics]
The map module uses the Generics `(<K, V>)` to specify which key type `[Key]` and which value type `[Value]` will hold.

Syntax Description
`map<KeyTip, ValueTip>` Defines the generic map type.
The Key Type is usually str or integer, while the Value Type can be anything, including any.

`var myMap: map<str, i32>;`
Defines a map with a string key and i32 value.

`var mixedMap: map<any, any>;`
The most flexible structure. Keys and values ​​can be of mixed types.

2. Basic Functions and Usage The map module provides the following basic functions for managing the map structure:
Function Purpose Syntax and Description

`new()` Creates a new, empty map instance.

`var user_data = map.new();`

`set()` Adds a new key-value pair to the map or updates an existing one.

```
user_data.set["name", "Alp"];
user_data.set[1001, "ID");
```
`get()` Returns the value corresponding to the given key. If the key is not found, it returns `null` or can optionally return `Result<V, Error>`.
`var name = user_data["name"];`

`has()` Checks whether the specified key exists in the map.
`if user_data.has("age"): ...`

`remove()`Removes the element with the specified key from the map.
`user_data.remove("age");`

`count()`Returns the number of key-value pairs in the map.
`var count = user_data.count();`

`clear()`Clears the entire contents of the map.
`user_data.clear();`

`keys()`Returns an `array<KeyTip>` containing all the keys in the map.
`var keys = user_data.keys();`

`values()`Returns an `array<ValueTip>` containing all the values ​​in the map.
`var values ​​= user_data.values();`

3. Easy Access Syntax

Easy Syntax
Basic Function Equivalent

Value Assignment `[Set]`
`user_data["city"] = "Ankara";`

Value Retrieval `[Get]`
`var city = user_data["city"];`


## SECTION 12.0: Error Management Philosophy

NIMBLE uses two main mechanisms to manage problems that may occur at runtime:
1. **Safe Error Handling (Result/Option):** Used for reversible and expected errors. Error checking is enforced by the compiler.
2. **Critical Exceptions (Exception/Panic):** Used for unrecoverable system errors or programmer logic errors.

### 1. Safe Error Handling: `Result<T, E>` and `Option<T>`

NIMBLE encourages using the `Result` or `Option` types in any function where an error result is possible.

| Type Name | Purpose | Description |
| :--- | :--- | :--- |
| **`Result<T, E>`** | Holds either a success value (`T`) or an error value (`E`). | Used for all operations that are likely to fail, such as I/O, networking, and conversion. |
| **`Option<T>`** | Holds either a value (`T`) or "None`." | Used as a safe alternative to lookup operations (e.g., `array.find()`, `os.get_env()`) or `null`. |

#### 1.1 Error Flow Control: `match` and `unwrap`

`Result` and `Option` types should be handled using safe methods such as the `match` statement or `unwrap_or`.

```nim
Nim

// Example: Opening a File
var fileResult = file.open("config.ini", file.READ);

match fileResult {
Ok(handle) => {
// Success: Continue processing
...
},
Err(error) => {
// Error: Take action specific to the error type
io.err_print("File Error: {error}");
}
}

// Safe Unwrapping: Return default if no value
var username = os.get_env("USER").unwrap_or("Anonymous");
```
#### 1.2 Error Type Hierarchy (Main Category)
All module errors are grouped under a single main Error enum/struct.
```
Error Type Name, Category, Covered Fields
IO_Error, I/O Errors,"file.FileError, io.IoError"
NET_Error, Network Errors, net.NetError
CONVERT_Error, Conversion Errors,"types.ConvertError, json.JsonError, regex.RegexError"
SYSTEM_Error, System Errors,"os.OsError, memory.MemError, gpu.GpuError"
RUNTIME_Error, Runtime Errors,"Index bounds exceeded, Division by zero, etc."
```
#### 1.3 Critical Exceptions (Panics)
NIMBLE uses the panic mechanism for unrecoverable situations or logic errors. The panic caller wraps the entire call stack and terminates the program.
```
Function/Structure, Purpose, Syntax, Description
panic(), Immediately terminates the program with a critical error message.,fn panic(msg: str): void,Used typically for internal library errors.
try_catch,An optional block for catching critical exceptions.,try { ... } catch (e) { ... },For rare cases where recovery is possible.
```

```
Nim

void fn process_data(data: i32[]) {
if data.is_empty() {
// Attempting to process an empty array requires a panic.
panic("process_data: The input array cannot be empty.");
}
// ...
}
```


## SECTION 15.0: The Concept and Application of Generics

Generics allow you to create functions and data structures written with type parameters to increase code reusability.

In NIMBLE, generics are defined using the syntax **`<T>`**.

### 1. Generic Functions

Used when a function needs to operate with the same logic with multiple types of data. The type parameter is specified immediately after the function name.

| Syntax | Description |
| :--- | :--- |
| `fn <name><T>(param: T): T` | Takes a parameter of type `T` and returns a value of type `T`. |

**Example: Swapping Two Values**

```nim
Nim

// Generic function that swaps two values ​​of type T.
fn swap<T>(a: *T, b: *T): void {
var temp: T = *a; // read the value of a
*a = *b; // write the value of b to a
*b = temp; // write the value of temp to b
}

void fn GenericSwapExample() {
var x: i32 = 10;
var y: i32 = 20;

swap<i32>(&x, &y); // compiled for type i32

var s1: str = "A";
var s2: str = "B";

swap<str>(&s1, &s2); // compiled for type str
}
```

## SECTION 16.0: An In-Depth Examination of the `group` Structure

The `group` structure not only ensures modularity in NIMBLE but also creates a layer of strict access control between data structures (`struct`) and functions.

### 1. Access Tokens in `group` and `struct`

Strict rules define how members in `group` and `struct` are exposed to the outside world.

| Token | Location | Accessibility | Description |
| :--- | :--- | :--- | :--- |
| **`pub`** | `struct` members | Accessible from outside `group`. | Provides direct field access, such as `user.name`. Private by default. |
| **`pub`** | `group` members | Accessible from other modules via the `module.group.member` chain. | Part of the module's API. |
| **(Default)** | `struct` and `group` members | Accessible only within the `group` or `struct` in which it is defined. | Provides encapsulation. |

**Example: Encapsulation**

````nim
Nim

// User Data Module
export group UserModule {

// The name field is accessible because it is pub, and the id field is private.
pub struct User {
pub name: str,
id: i32 // Default: Accessible only within the UserModule.
}

// Factory function: Creates a new user
pub fn create_user(name: str): User {
// The id field can be set here because we are in a group.
return User { name: name, id: 12345 };
}

// Private helper function
fn generate_id(): i32 {
return 999;
}
}

// Using Another Module:
use UserModule;

void fn TestAccess() {
var u = UserModule.create_user("Bob");

// Success: name pub
io.println("Name: {u.name}");

// ERROR: id private
// io.println("ID: {u.id}"); // COMPILATION ERROR

// ERROR: generate_id private
// UserModule.generate_id(); // COMPILATION ERROR
}

```

2. Nesting Groups and Structs
Complex modular structures can be created by defining groups and structs within each other.

Relationship, Purpose, Rule
Define a struct within a group, a module-specific data structure. If the struct is to be used externally, it must be pub or exported.
Group within a struct: Associating logically related methods with a data structure (similar to the impl block in Rust).
"group contains the methods of the struct. The self keyword points to the struct instance."

Example: Methods (group within a struct)
```
Nim

export struct Vector3 {
pub x: f32,
pub y: f32,
pub z: f32,

// group contains the methods of the Vector3 structure.
pub group Methods {
// Methods take the 'self' parameter.
pub fn length(self: Vector3): f32 {
// Requires access to the math module.
return math.sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
}
}
}

// Usage:
use Vector3;

void fn VectorExample() {
var vec = Vector3 { x: 1.0, y: 2.0, z: 3.0 };

// Methods are called via group.
var len = vec.Methods.length();
io.println("Vector length: {len}");
}
```


## SECTION 18.0: Foreign Function Interface (FFI) and C Bindings

FFI (Foreign Function Interface) is the mechanism that allows NIMBLE programs to interact with external libraries written in C, C++, or other low-level languages. This is especially crucial for system-level access, operating system APIs, and using existing high-performance libraries.

### 1. Basic Syntax: `extern`

The `extern` keyword indicates that a function or variable is provided externally (i.e., from the C library) and not by the current module.

| Syntax | Purpose | Description |
| :--- | :--- | :--- |
| **`extern fn`** | Declaring a C function defined in an external library in NIMBLE. | The function signature must match its C counterpart. |
| **`extern var`** | Declaring a global C variable defined in an external library in NIMBLE. | |
| **`#[link(name="<library_name>")]`** | Instructing the compiler which external library to link with (instructing the linker). |

### 2. Declaring C Functions

To use a C library, the relevant functions and variables must be "shadowed" in NIMBLE.

**Example: Using the `sqrt` Function from the `math.h` Library**

```nim
Nim

// Tell the linker to link with 'm' (the math library).
#[link(name="m")]
export group C_Math {

// We declare the C function double sqrt(double x) with extern.
// In NIMBLE, f64 corresponds to the C double.
extern pub fn sqrt(x: f64): f64;

// extern pub fn sin(x: f64): f64;
}

// Usage
use C_Math;

void fn FFIExample() {
var sqrt: f64 = 16.0;

// The call is directed to the function in the C library.
var coq = C_Math.sqrt(sqrt);
io.println("Square root: {coq}"); // Output: 4.0
}

```

3. Data Type Bridging
NIMBLE uses safe type bridging for FFI.
The programmer is expected to use the NIMBLE types (i32, f64, str, *T) appropriately with the corresponding C types (int, double, char*, void*).
```
NIMBLE Type, C Equivalent, Description
"i32, u64","int, unsigned long long", Size matching is important.
"f32, f64","float, double", Standard floating-point types.
*T, T* (Pointer), Corresponds to raw memory addresses.
str, const char*, For readable string data (conversion may be required if the NIMBLE string format is different).
```

4. Unsafe Area (unsafe)
Because FFI calls inherently allow unsafe operations (working with raw pointers, memory management),
NIMBLE code that calls FFI may need to be enclosed in unsafe blocks.

```
Nim

// May be required for pointer arithmetic or manual memory operations.
unsafe {
var ptr = C_Memory.malloc(1024);
// ...
}
```
