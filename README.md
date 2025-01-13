# atad

# Problem1_SolveMeFirst

solve_me_first Function:

- solve_me_first is a function that takes two i32 integers as arguments and returns their sum.

Reading Input:

- Inputs are read as strings using io::stdin().read_line().
- These strings are trimmed and parsed into i32 using .trim().parse().

Calling the Function:

- The solve_me_first function is called with the parsed integers.
- The result is printed using println!.

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

# Problem4_AVeryBigSum

aVeryBigSum function:

- Takes as input a slice of 64-bit integers (&[i64]).
- Uses the iter method to create an iterator over the slice, and the sum method to compute the sum of all elements.
- Returns the sum of the elements as a 64-bit integer (i64).

Rust's i64 type supports 64-bit signed integers, making it suitable for very large sums.

# Problem5_DiagonalAbsoulteDifference

diagonalDifference function:

- Takes as input a reference to a 2D vector arr, which represents a square matrix of integers.
- primary_diagonal_sum: The sum of the elements on the primary diagonal (from top-left to bottom-right).
- secondary_diagonal_sum: The sum of the elements on the secondary diagonal (from top-right to bottom-left).
- Loop: it iterates through the rows of the matrix:
  - For the primary diagonal, it sums arr[i][i].
  - For the secondary diagonal, it sums arr[i][n - 1 - i], where n is the size of the matrix.
  - Finally, it returns the absolute difference between the two sums.

# Problem6_PlusMinus

plusMinus function:

- Takes a slice of integers as its parameter.
- Calculates the length of the array n as a floating-point number.
- Initializes counters for positive, negative, and zero values.
- Iterates over the array and increments the appropriate counter based on whether the element is positive, negative, or zero.-
- Calculates the ratios of positive, negative, and zero values by dividing the respective counts by n and prints the results.

# Problem7_Staircase

- The main function reads the input from the standard input. It reads the integer n, which denotes the size of the staircase.

stairCase function:

- Takes an integer n as its parameter.
- Iterates from 1 to n (inclusive).
- For each iteration i, it calculates the number of spaces and hashes needed for that line:
  - The number of spaces is n - i.
  - The number of hashes is i.
- Constructs the line by repeating the space character and the hash character the required number of times.
- Prints the constructed line.
