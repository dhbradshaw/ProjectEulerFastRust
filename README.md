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

Or just browse.  To look at the code for a given problem, go to `src/problems/rs`.

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

The times in the "This" column are based on running ```cargo run --release```

on my laptop.

Posts in the Project Euler forums can be divided into two sections, old and new.
The old posts seem to be fixed, and the new posts rotate through and disappear
after a time.

Looking for times for all 50 problems turned out to be a lot of work.  The times
that I found are documented in the spreadsheet titled "Euler 50 forum times.ods"
which is here in this repository.  That spreadsheet can be improved and added to.  
It's interesting to take a look at, partly because you can look at solutions and
posted times in many different languages from Assembly, Delphi, C/C++ and Rust on
the fast side to Powershell on the slow side.  

I ended up going through the first
couple of pages to get old times and the last three pages to get new times for each
problem.  

So a few points about the times that I'm comparing my solutions to:

1.  My search through the forums was systematic, but not exhaustive.  I sifted
through some of the first and last pages for each problem.
2.  The solutions posted here were not all optimized for speed.  Many are first
efforts, and many are made by people who are just learning the language that they
chose for their solution.
3.  Some judgment was required in converting solutions to actual numbers.  What
number do you enter if someone says that their solution is "faster than a second?"
(I typically put 0.9 seconds.)  How about a "blink of an eye?" (I usually skipped
these.)  How about "0.0 seconds?" (I wrote 0.04 for this, but I'm not sure that's
the right thing to do.  Should I just skip the number?)  
4.  Some problems were too easy to get good numbers for, meaning that they were
often solved by hand.  Or at least, meaning that no one felt like it was worth
posting times.  In these cases, I just listed the times as zero.

So with that as an introduction, here's a summary of the best of the Old times
and the best of the new times from the pages that I analyzed.  In the final column
are the times associated with this repository.

| p | Title            | Best Old`*`  | Best New`*` | This |
| ------- |:---------------------:| -----:| -----:| -----:|
|Sum| All | 3.9 | 1.6 | 9.3E-2 | 6.1E-01 |
|p1| Multiples of 3 and 5        |0.0E+00|0.0E+00|3.1E-07|
|p2| Even Fibonacci numbers      |2.9E-05|0.0E+00|1.9E-07|
|p3| Largest prime factor        |1.6E-02|1.8E-06|1.8E-05|
|p4| Largest palindrome product  |1.3E-03|4.0E-03|1.8E-05|
|p5| Smallest multiple           |0.0E+00|0.0E+00|1.3E-07|
|p6| Sum square difference       |0.0E+00|0.0E+00|1.3E-07|
|p7| 10001st prime               |9.5E-04|1.4E-02|5.6E-04|
|p8| Largest product in a series |0.0E+00|9.0E-04|4.3E-05|
|p9| Special Pythagorean triplet |9.0E-01|3.6E-05|1.8E-06|
|p10| Summation of primes        |2.7E-02|9.0E-01|1.9E-02|
|p11| Largest product in a grid  |9.0E-04|1.0E-03|1.3E-04|
|p12| Highly divisible triangular number |9.4E-02|9.8E-02|4.2E-03|
|p13| Large sum                  |1.2E-02|1.7E-04|1.5E-05|
|p14| Longest Collatz sequence   |8.1E-01|3.3E-01|3.1E-02|
|p15| Lattice paths              |2.9E-03|4.0E-04|2.3E-06|
|p16| Power digit sum            |9.0E-06|1.5E-03|2.5E-05|
|p17| Number letter counts       |0.0E+00|0.0E+00|6.8E-04|
|p18| Maximum path sum I         |4.0E-07|1.0E-04|5.0E-06|
|p19| Counting Sundays           |7.3E-03|1.6E-04|1.3E-05|
|p20| Factorial digit sum        |1.3E-03|1.0E-02|1.4E-05|
|p21| Amicable numbers           |5.0E-04|6.0E-03|4.6E-04|
|p22| Names scores               |3.6E-02|3.0E-03|1.6E-03|
|p23| Non-abundant sums          |1.2E-01|1.4E-01|8.0E-03|
|p24| Lexicographic permutations |2.6E-05|2.4E-04|3.6E-06|
|p25| 1000-digit Fibonacci number |0.0E+00|0.0E+00|1.7E-07|
|p26| Reciprocal cycles          |1.1E-02|9.0E-04|1.5E-05|
|p27| Quadratic primes           |1.0E+00|0.0E+00|2.0E-03|
|p28| Number spiral diagonals    |1.0E-02|4.0E-05|7.8E-07|
|p29| Distinct powers            |7.5E-03|1.0E-02|1.5E-03|
|p30| Digit fifth powers         |2.3E-01|5.0E-03|1.9E-04|
|p31| Coin sums                  |4.0E-03|4.0E-04|4.0E-04|
|p32| Pandigital products        |9.0E-04|4.0E-04|1.0E-03|
|p33| Digit cancelling fractions |2.0E-03|8.0E-05|4.2E-06|
|p34| Digit factorials           |9.5E-02|1.9E-02|4.0E-04|
|p35| Circular primes            |1.0E-02|8.0E-03|2.0E-03|
|p36| Double-base palindromes    |2.0E-03|3.2E-03|4.6E-05|
|p37| Truncatable primes         |2.0E-03|4.0E-04|2.9E-04|
|p38| Pandigital multiples       |0.0E+00|0.0E+00|1.4E-04|
|p39| Integer right triangles    |9.0E-04|1.0E-04|7.9E-06|
|p40| Champernowne's constant    |4.0E-04|2.0E-05|1.1E-06|
|p41| Pandigital prime           |3.0E-05|1.0E-04|7.2E-05|
|p42| Coded triangle numbers     |2.6E-03|4.9E-04|1.6E-04|
|p43| Sub-string divisibility    |4.0E-05|3.0E-03|2.1E-05|
|p44| Pentagon numbers           |1.5E-01|3.6E-02|1.3E-02|
|p45| Triangular, pentagonal, and hexagonal |9.0E-04|3.0E-04|4.9E-05|2.5E-04|
|p46| Goldbach's other conjecture |8.0E-03|2.0E-03|7.3E-05|1.9E-03|
|p47| Distinct primes factors    |6.0E-02|3.5E-02|7.8E-04|3.4E-02|
|p48| Self powers                |7.0E-03|2.0E-03|3.4E-03|-1.4E-03|
|p49| Prime permutations         |6.0E-03|2.0E-04|6.5E-05|1.3E-04|
|p50| Consecutive prime sum      |3.2E-03|5.0E-04|1.7E-03|-1.2E-03|


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
Several people, upon learning what I was working on, have asked me what I've learned.  So I thought I would write a bit on that here.

Here are some of the main things.

### 1. If you want fast code, you need to measure its speed.

My first attempt at a fast solution to problem 14 on Collatz chains used a Hashmap for memoization.  But when I actually measured its performance compared to a naive approach, it was significantly slower.  That completely surprised me coming from Python where using dicts and sets for memoization was one of my most useful tricks.

### 2.  Hunt where there's game.

If you're searching for a rare condition, the slowest thing you can do is check everything.  For example, if you're looking for numbers that are both hexagonal and pentagonal, don't check every number to see if it's hexagonal.  Instead, generate hexagonal numbers.  Maybe even co-generate pentagonal numbers.

If you're looking for palindromes that meet a particular condition, don't check every number to see if it's a palindrome.  Generate the palindromes.

Don't check large even numbers to see if they're prime.

This principle sped several solutions up by multiple orders of magnitude.  Applying it is also one of the most fun parts of figuring out the right solution to these problems.

### 3.  Memoize if you can.  But also avoid hashing if you can.

If you're going for speed, memoizing can be a huge help.  However, hashing has real overhead.  Even with FNV or another fast hasher, it's going to cost you to use a Hashmap or a Hashset.

Project Euler problems tend to be special in that often the parameter space that you care about are dense sets of integers.  So for many of the problems you can avoid hashing by using an array of booleans in place of a Hash Set or an array of integers in place of a Hashmap.  If you can get away with that, it will speed up your code considerably.

I used this trick with problem 14.  Where the Hashmap solution was slower than the non-memoizing solution, the Array solution was a fair amount faster.  Hashing is part of the issue here.  Another part is the speed difference between the stack and the heap.  So,

### 4.  If you can, use the stack not the heap.

This is obvious, I think, to programmers with some systems work under their belt.  My experience before now has been primarily Mathematica, Matlab, Python, and Javascript.  So it's been nice to have tools that let me distinguish between the stack and the heap and use the stack when it's profitable.  It was surprising how much the stack can actually hold.  However,

### 5.  While the stack is fast, it has limits.

1.  Important note learned from experience: if your rust code crashes with no error message and for no apparent reason, you may have been putting too much onto the stack.  It's impressive how much room there is there, but there are limits.
2.  I achieved measurable speed ups by decreasing the size of arrays that I had on the stack.  For example, a u32 array with a length of 100_000 will be measurably slower than a u16 array with the same length.  
3.  I haven't quantified this, but I also get the sense that large arrays are associated with more variance in the time it takes a bit of code to run.

### 6.  If you need bulk information, process in bulk.

A prime sieve is a perfect example of bulk processing.  If you need all the primes under 2 million, you can find them much more quickly by finding all those primes at once using a sieve than by checking integers one at a time to see if they are prime.

But the same sieve concept can be and is used in other ways for other problems here and leads to significant speed improvements for those other solutions as well.

### 7.  A small subset of Rust is already enough to do interesting things.

The code here is simple.  It only uses a small subset of what Rust has to offer.  But that small subset was enough to let me solve interesting problems in interesting ways and achieve speeds that I'm proud of.  

The point I'm trying to make is that a beginner doesn't need to get discouraged.  While there is a ton to learn, you don't need to learn everything to do something interesting.  It's worthwhile finding a useful subset of a new tool and then expanding your knowledge as the need arises (or as you find excuses).

### 8.  Math is faster than string formatting.

I gradually moved over to handling as much number manipulation as possible using math instead of using string formatting.  Use modulus rather than getting the last character of a stringified number, for example.

### 9.  Math on built-in types is faster than using BigInt or similar libraries.

Rust has a BigInt/BigUint library called num.  It's handy.  But if you want fast Euler Rust solutions and if it's feasible, then it's probably faster to use math to avoid working with it.

## Contributors
Only me :-(.

Make a pull request and submit improvements so that we can change that!  There's a lot of low hanging fruit.
