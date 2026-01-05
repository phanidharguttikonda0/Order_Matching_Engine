
## Building Order Matching Engine

#### Our End Goal is to Achieve around 1.5 Million requests per second as High Frequency Trading Systems Achieves.


### Phase - 1
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


### Phase - 2 (Doing Enhancements to current scenario)

#### Phase - 2.1 Implementing Logger Event

###### where the println are i/o bound task, which block the main thread execution. so spawning a system level thread and listening to a cross beam channel, every log will be added to the cross beam channel(which is a bounded channel) and write logs over there.

##### Results: Phase - 2.1

Fastest : 165.70 micro seconds

Average : 170.26 micro seconds

Slowest : 176.05 micro seconds

Totally 1.1M requests per second , 9.5x faster than previous.

Running I/O tasks in the background made the total Execution too Fast.


##### Phase - 2.2

###### Design

###### Making all Operations in O(1), Not Only that making sure that CPU travelling time is also reduced, basically when we store a struct value in RAM it will be stored here and there for each and every value the CPU needs to travel long distance, so packing every Order Linerly, such that CPU can directly jump to that address, where CPU travelling Time will be reduced. Making it more efficient and Faster.

#### Not a Big Change in Results it became standard and executing 1.2 M Requests per second

