# 🚀 Ultra-Low Latency Limit Order Book (Rust)

**A high-frequency trading (HFT) matching engine written in Rust, capable of processing over 14 Million orders per second via TCP.**

This project demonstrates the transition from a standard application-level implementation to a systems-level optimized engine, achieving a **325x total performance improvement** (from 43k to 14M ops/sec) through memory arena allocation, zero-copy networking, and batch processing.

---

## 🏗 Architecture
* **Core Engine:** Single-threaded, pinned to a CPU core. Uses **Slab Allocation (Arena)** for O(1) order matching and zero runtime memory allocations.
* **Memory Layout:** Contiguous memory arrays (Cache Locality) replacing standard pointer-based collections (`BTreeMap`, `VecDeque`).
* **Gateway:** Async TCP (Tokio) with **Batch Processing** (4KB buffers) and Zero-Copy parsing.
* **Logging:** Decoupled critical path using a lock-free Ring Buffer for asynchronous logging.

---

## ⚡ The Performance Journey

We benchmarked the system at every stage of development. Here is the evolution of the engine's speed.

### Phase 1: Logic & Correctness
* **Implementation:** Standard Rust collections (`BTreeMap`, `VecDeque`) and standard I/O (`println!`).
* **Bottleneck:** Blocking I/O and Heap Allocation fragmentation.

| Metric | Result | Throughput |
| :--- | :--- | :--- |
| **Fastest Cycle** | 4.34 ms | **~43,000 req/sec** |
| **Average Cycle** | 4.63 ms | |
| **Slowest Cycle** | 4.93 ms | |

### Phase 2: System Optimization (The 1 Million Barrier)
* **Optimization 2.1 (Async Logging):** Removed blocking `println!` from the hot path. Spawned a background thread listening to a bounded `Crossbeam Channel`.
* **Optimization 2.2 (Slab Allocation):** Replaced `VecDeque` with a pre-allocated **Vector Arena**. Implemented a custom Linked List over Array indices to ensure **O(1)** insertions/deletions and perfect CPU Cache Locality.

| Optimization | Average Latency | Throughput | Improvement |
| :--- | :--- | :--- | :--- |
| **Async Logging** | 170.26 µs | **1.1 Million req/sec** | **9.5x** |
| **Arena (O(1))** | 150.00 µs | **1.2 Million req/sec** | Stable & Deterministic |

### Phase 3: Network Gateway Optimization (The 14 Million Barrier)
* **Goal:** Expose the engine via TCP without killing performance.
* **Challenge:** System Calls (syscalls) are expensive (~300ns). Reading 1 packet at a time chokes the CPU.

#### Phase 3.1: Naive TCP Implementation ("The Spoon")
* **Mechanism:** `read_exact(13 bytes)`.
* **Behavior:** The system sleeps and wakes up for *every single order*.
* **Result (1 Million Orders):** Took **~700 ms**.
* **Throughput:** ~1.4 Million req/sec.

#### Phase 3.2: Batch Processing & Nagle's Algorithm ("The Bucket")
* **Optimization 1:** Disabled Nagle's Algorithm (`TCP_NODELAY`) to reduce latency.
* **Optimization 2:** Implemented **Batch Reading** with a 4KB buffer.
    * Reads up to ~300 orders in a single System Call.
    * Parses orders in a tight loop in Userspace RAM (Zero System Cost).
    * **Zero-Allocation Loop:** Reuses the same buffer logic to prevent heap fragmentation during high load.
* **Result (1 Million Orders):** Took **~71 ms**.
* **Throughput:** **~14.1 Million req/sec**.

### 🏆 Final Result: 10x improvement over naive TCP.

---

## 🛠️ How to Run

### Prerequisites
* Rust (latest stable)
* Linux/MacOS (Preferred for accurate syscall benchmarking)

### 1. Run the Exchange Server
This starts the Matching Engine and the TCP Gateway.
```bash
# Run in Release mode for optimizations
cargo run --release