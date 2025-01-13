# atad

# Problem1_SolveMeFirst

Function Definition:
solve_me_first is a function that takes two i32 integers as arguments and returns their sum.

Reading Input:
Inputs are read as strings using io::stdin().read_line().
These strings are trimmed and parsed into i32 using .trim().parse().

Calling the Function:
The solve_me_first function is called with the parsed integers.
The result is printed using println!.

# Problem2_SimpleArraySum

Function Definition:

The ar parameter is a slice of integers (&[i32]).
The simpleArraySum function uses the iter() method to create an iterator over the slice and calls sum() to compute the total.
