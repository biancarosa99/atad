# atad

# Problem1_SolveMeFirst

solve_me_first Function:
solve_me_first is a function that takes two i32 integers as arguments and returns their sum.

Reading Input:
Inputs are read as strings using io::stdin().read_line().
These strings are trimmed and parsed into i32 using .trim().parse().

Calling the Function:
The solve_me_first function is called with the parsed integers.
The result is printed using println!.

# Problem2_SimpleArraySum

simpleArraySum function:

- The ar parameter is a slice of integers (&[i32]).
- The simpleArraySum function uses the iter() method to create an iterator over the slice and calls sum() to compute the total.

# Problem3_CompareTheTriplets

compareTriplets function:

- Accepts two slices of integers a and b, each of length 3.
- Iterates over the indices 0..3:
- If a[i] > b[i], Alice gets 1 point.
- If a[i] < b[i], Bob gets 1 point.
- Returns a Vec<i32> containing the scores [alice_score, bob_score].
