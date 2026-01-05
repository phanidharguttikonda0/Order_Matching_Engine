#### In First GO (first code we got this result)
Fastest Execution Time of each buy sell cycle : 4.34 ms

Average Execution Time of each buy sell cycle : 4.63 ms

Slowest Execution Time of each buy sell cycle : 4.93 ms

above for 100 buy and 100 sell orders it took around 4.63 ms

In a Second 200/0.00463 approx 43K requests per second.

#### After commenting unnecessary print statements

over 4 times i have ran the cargo bench, these are results

Average : 1.61 ms, 1.15 ms, 1.15 ms, 1.71 ms 

Fastest : 1.21 ms, 1.00 ms, 1.06 ms, 1.36 ms

Slowest : 2.19 ms, 1.33 ms, 1.24 ms, 2.13 ms

Totally 123K requests per second 2.8x faster than previous,

#### Our End Goal is to Achieve around 1.5 Million requests per second
