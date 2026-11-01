# Lab Tasks

Lab 1
- completed

Lab 2
1. Run both examples
2. integer examples compile because i32 has copy trait implemented so the the value copy to other variable rather that move. but the String type has no copy trait implemented, so the value move directly to other variable and the previous varibale freed up from memory. now when you try to print the value which is already freed up from memory the rust find it before compilation and through error.
3. To print the both `model` and `selected_model` you have to clone the `model` value to the `selected_model`. It has some performance issue and also there will be unused value on the programmer which will takes up teh memory.
