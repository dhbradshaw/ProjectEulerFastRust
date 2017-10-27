# Fast Project Euler Solutions in Rust
Fast and (sometimes) simple solutions to the first 50 Project Euler problems in Rust.

Currently written by a rank Rust beginner, so don't come here for style tips or beautiful Rust, at least not yet.  Instead, maybe try https://github.com/gifnksm/ProjectEulerRust .

## Purpose
In trying to learn to make my Rust code go more quickly, it was extremely helpful to know what was possible in the first place.  Having other people post fast times challenged me to improve my own code.  

Similarly, seeing fast solutions taught me principles that I was then able to apply to speed up the problems associated with those solutions and then also other problems where similar tricks could apply.

While seeing times and solutions helped me a lot, it would have helped me more efficiently if I could have seen fast code gathered in one place.  So here I've gathered a set of solutions in one language in one place and am making them available.

## Usage:
Clone the repo.  Then do
```
$ cargo run --release
```
to compile and run the code.  

It was developed using the stable branch.  For me right now that's 1.21.0:

```
$ rustc --version
rustc 1.21.0 (3b72af97e 2017-10-09)
```  

To look at a given problem, go to `src/problems/rs`.

## Community challenge: 0.1 => 0.01 seconds.
I read a statement on Hacker News that said that skilled programmers can look at the code of a beginner and improve its efficiency by a factor of 10.  Can that happen here?  

As of October 2017, the code in this repository solves the first 50 problems sequentially in under 0.1 seconds on my laptop.  I would like to open this repository to the community with the hope that together we can make it faster by another factor of ten.

Currently, that seems impossible.  But getting it down to under 0.1 seconds also seemed impossible and then I did it.  So I'm learning to reserve judgment on what's possible and what is not.

## The story behind this repository

In July of 2017 I started reading through the new Rust book and solving the first 50 Euler Problems ( https://projecteuler.net/archives ) in Rust.

My goal, of course, was to become capable of doing interesting things in Rust.  To help that learning accelerate, I checked the forums for each problem after solving it to see what others had done.  It was a treat to see many different solutions in many different languages.

One thing that many people did was post the time it took their code to run.  As I compared my times to the times posted, I began to see that if I made a little effort, I could get competitive times using Rust.  Since one of my purposes in learning Rust was to learn to write fast code, I made speed an explicit goal.

After completing the first 50 problems, I logged the time it took to complete all of them sequentially on my laptop.  Initially, that time was almost **2** seconds.

I started going after the slowest algorithms to speed them up.  Since that was fun, I then went back and sped up some of the faster ones.  In doing that I gained insights that helped me speed up the slower ones some more.  Gradually I got the combined sequential time down from 2 seconds to **0.5** seconds, then **0.25** seconds, and finally below **0.1** seconds.

## Times

So here are my times next to the average and fasted posted times for each of the different problems.  The times are on my laptop, a 17 inch $300 HP from 2016.

|  | Title            | Avg`*`  | Fastest`*` | This |
| ------- |:---------------------:| -----:| -----:| -----:|
| 1 | Multiples of 3 and 5        | NA | NA | 2.7e-6 |
| 2 | Even Fibonacci numbers      | NA | NA | 1.9e-7 |
| 3 | Largest prime factor        |  | 1.8e-6 | 1.8e-5 |
| 4 | Largest palindrome product  | Avg | Fastest | 1.8e-5 |
| 5 | Smallest multiple           | Avg | Fastest | 1.3e-7 |
| 6 | Sum square difference       | Avg | Fastest | 3.0e-7 |
| 7 | 10001st prime               | Avg | Fastest | 6.3e-4 |
| 8 | Largest product in a series | Avg | Fastest | 4.3e-5 |
| 9 | Special Pythagorean triplet | Avg | Fastest | 1.8e-6 |
| 10 | Summation of primes        | Avg | Fastest | 1.9e-2 |
| 11 | Largest product in a grid  | Avg | Fastest | 8.6e-5 |
| 12 | Highly divisible triangular number | Avg | Fastest | 3.4e-3 |
| 13 | Large sum                  | Avg | Fastest | 1.5e-5 |
| 14 | Longest Collatz sequence   | Avg | Fastest | 2.7e-2 |
| 15 | Lattice paths              | Avg | Fastest | 2.3e-6 |
| 16 | Power digit sum            | Avg | Fastest | 1.7e-5 |
| 17 | Number letter counts       | Avg | Fastest | 6.9e-4 |
| 18 | Maximum path sum I         | Avg | Fastest | 6.3e-6 |
| 19 | Counting Sundays           | Avg | Fastest | 1.4e-5 |
| 20 | Factorial digit sum        | Avg | Fastest | 1.4e-5 |
| 21 | Amicable numbers           | Avg | Fastest | 4.5e-4 |
| 22 | Names scores               | Avg | Fastest | 1.5e-3 |
| 23 | Non-abundant sums          | Avg | Fastest | 6.9e-3 |
| 24 | Lexicographic permutations | Avg | Fastest | 4.3e-6 |
| 25 | 1000-digit Fibonacci number | Avg | Fastest | 8.6e-8 |
| 26 | Reciprocal cycles          | Avg | Fastest | 1.5e-5 |
| 27 | Quadratic primes           | Avg | Fastest | 1.2e-3 |
| 28 | Number spiral diagonals    | Avg | Fastest | 7.4e-7 |
| 29 | Distinct powers            | Avg | Fastest | 1.4e-3 |
| 30 | Digit fifth powers         | Avg | Fastest | 1.9e-4 |
| 31 | Coin sums                  | Avg | Fastest | 4.0e-4 |
| 32 | Pandigital products        | Avg | Fastest | 9.3e-4 |
| 33 | Digit cancelling fractions | Avg | Fastest | 3.9e-6 |
| 34 | Digit factorials           | Avg | Fastest | 4.0e-4 |
| 35 | Circular primes            | Avg | Fastest | 3.1e-3 |
| 36 | Double-base palindromes    | Avg | Fastest | 4.6e-5 |
| 37 | Truncatable primes         | Avg | Fastest | 2.8e-4 |
| 38 | Pandigital multiples       | Avg | Fastest | 1.3e-4 |
| 39 | Integer right triangles    | Avg | Fastest | 8.2e-6 |
| 40 | Champernowne's constant    | Avg | Fastest | 7.9e-7 |
| 41 | Pandigital prime           | Avg | Fastest | 7.4e-5 |
| 42 | Coded triangle numbers     | Avg | Fastest | 1.6e-4 |
| 43 | Sub-string divisibility    | Avg | Fastest | 2.4e-5 |
| 44 | Pentagon numbers           | Avg | Fastest | 1.2e-2 |
| 45 | Triangular, pentagonal, and hexagonal | Avg | Fastest | 4.5e-5 |
| 46 | Goldbach's other conjecture | Avg | Fastest | 6.5e-5 |
| 47 | Distinct primes factors    | Avg | Fastest | 7.4e-4 |
| 48 | Self powers                | Avg | Fastest | 3.4e-3 |
| 49 | Prime permutations         | Avg | Fastest | 1.3e-4 |
| 50 | Consecutive prime sum      | Avg | Fastest | 4.0e-3 |

`*` While my own times were easy enough to measure, the numbers for average and fastest times should be taken with a huge grain of salt.  Here are some issues:
1. It's difficult to get the numbers from the forum, so these are just the ones that I got as I scanned the first two and last pages of each thread manually.  So I didn't scan all 9 pages, and may have missed times within the pages I did scan.  
2.  Old numbers were obtained on old computers, which may be a bit slower.  On the other hand, some problems were made more difficult over time.  
3.  I didn't take time to check the validity of the solutions associated with the posted times.
4. While the first few pages of the forum are static, the last ones constantly change.  So these numbers could well be out of date by now.

## Priorities for this Repo
Everyone wants fast, elegant, correct code.  Things only get interesting when you have to choose between the different priorities.

In this repository, the priorities are
1. Correctness
2. Speed
3. Beauty

In other words, for these solutions we shouldn't give up correctness for speed or speed for beauty.  Here are brief definitions of the three terms.

### Correctness
Correctness for these solutions consists of two parts:  
1.  The solution must give the correct answer.  
2.  The solution must prove that the answer it gives is correct.

A lucky guess is better than no guess at all or a wrong one.  But it's not as correct as a solution that doesn't guess.  So a slower time that avoids guessing and instead provides a proof that the answer is indeed the correct one beats a solution that doesn't provide that proof.

### Speed
Speed is currently measured by the time it takes an algorithm to run and print the correct answer on my laptop.

### Beauty
Beauty isn't as well defined as speed and correctness and honestly it hasn't been a high priority yet.  But this code should be altered to be as beautiful as possible -- without sacrificing speed.

Here are some aspects of beauty (note that this is an unordered list):

- Readability
- Usability
- Idiomaticity

## Lessons Learned

## Contributors
Only me :-(.

Make a pull request and submit improvements so that we can change that!  There's a lot of low hanging fruit.
