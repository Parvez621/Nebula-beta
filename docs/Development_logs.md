# Nebula Development Log

> Development record for the Nebula distributed AI compute runtime.

---

# 1. Project Vision

Nebula is an experimental distributed AI compute runtime designed to allow multiple devices owned by a user to pool their available computing resources and behave as a unified compute environment.

The initial target workload is:

- Distributed LLM inference

The long-term goal is to support additional compute-intensive workloads such as:

- Image generation
- Speech processing
- Video processing
- Distributed compilation
- Scientific workloads
- Other AI and compute workloads

The user should not need to manually manage individual devices, model shards, or workload placement.

Nebula should eventually handle:

- Device discovery
- Resource monitoring
- Workload scheduling
- Model placement
- Distributed execution
- Node failure
- Device joining/leaving
- Heterogeneous hardware
- Network conditions
- Battery and thermal constraints

---

# 2. Development Philosophy

Nebula is being developed incrementally.

Each milestone should:

1. Implement one meaningful capability.
2. Test the implementation.
3. Benchmark it where appropriate.
4. Commit the changes to Git.
5. Record the milestone in this document.
6. Only then move to the next milestone.

Complex infrastructure should not be introduced before it is actually required.

Examples of technologies that should not be introduced prematurely:

- Kubernetes
- Raft
- Distributed databases
- Service meshes
- Complex orchestration systems
- Async runtimes when synchronous networking is sufficient

The initial goal is to understand and implement the fundamental distributed-system primitives ourselves.

---

# 3. Development Environment

Initial development environment:

```text
OS          : Fedora Linux
Architecture: x86_64
Kernel      : 7.1.8-200.fc44.x86_64
```

Development hardware:

```text
CPU         : Intel Core i5-9400H
Cores       : 4
Threads     : 8
RAM         : 16 GB
GPU         : Intel UHD Graphics 630
```

Compiler/toolchain:

```text
Rust
rustc 1.97.1
cargo 1.97.1

GCC
16.1.1

Git
2.55

Python
3.14.7
```

Build tools used for llama.cpp:

```text
CMake
Ninja
```

---

# 4. Initial Repository

## Milestone 0.1 — Initial Project

**Status:** Complete

The initial Nebula repository was created.

Initial repository concept:

```text
Nebula/
├── crates/
├── vendor/
├── docs/
├── Cargo.toml
└── README.md
```

The project was initialized under Git version control.

### Commit

```text
initial project commit
```

Commit hash:

```text
TBD
```

---

# 5. llama.cpp Integration

## Milestone 0.2 — llama.cpp

**Status:** Complete

`llama.cpp` was added to the repository under:

```text
vendor/llama.cpp/
```

The project was built locally using CMake and Ninja.

Available binaries include:

```text
llama
llama-cli
llama-bench
llama-server
```

OpenSSL development support was installed and enabled so that llama.cpp could download models over HTTPS.

The build completed successfully.

### Purpose

llama.cpp is currently used as the local LLM inference backend.

Nebula itself is being written in Rust, while llama.cpp provides the low-level inference implementation that Nebula will eventually orchestrate.

---

# 6. Qwen2.5-7B Baseline

## Milestone 0.3 — Local LLM Inference

**Status:** Complete

The first model used for Nebula testing:

```text
Qwen2.5-7B-Instruct
Q4_K_M
GGUF
```

Approximate model size:

```text
4.36 GiB
```

Parameter count:

```text
7.62B
```

The model was successfully downloaded and executed locally through llama.cpp.

The model cache was verified using:

```bash
llama-cli --cache-list
```

The model appeared as:

```text
Qwen/Qwen2.5-7B-Instruct-GGUF:Q4_K_M
```

---

# 7. Single-Node Inference Baseline

## Milestone 0.4 — Benchmark

**Status:** Complete

Before attempting distributed inference, a controlled single-node baseline was established.

Hardware:

```text
CPU         : Intel Core i5-9400H
Cores       : 4
Threads     : 8
RAM         : 16 GB
GPU         : Intel UHD Graphics 630
Backend     : CPU
```

Benchmark command:

```bash
./vendor/llama.cpp/build/bin/llama-bench \
    -hf Qwen/Qwen2.5-7B-Instruct-GGUF:Q4_K_M \
    -t 8 \
    -p 512 \
    -n 128 \
    -r 3
```

Results:

| Model | Size | Params | Backend | Threads | Test | Performance |
|---|---:|---:|---|---:|---|---:|
| Qwen2 7B Q4_K Medium | 4.36 GiB | 7.62B | CPU | 8 | pp512 | 19.94 ± 0.08 t/s |
| Qwen2 7B Q4_K Medium | 4.36 GiB | 7.62B | CPU | 8 | tg128 | 6.30 ± 0.55 t/s |

Where:

```text
pp512 = prompt processing using 512 tokens

tg128 = generation of 128 tokens
```

This benchmark establishes the first performance baseline against which future Nebula configurations can be compared.

Benchmark documentation:

```text
docs/benchmarks/baseline-001.md
```

### Commit

```text
bench: establish single-node inference baseline
```

Commit hash:

```text
TBD
```

---

# 8. Rust Workspace

## Milestone 0.5 — Rust Workspace

**Status:** Complete

Nebula's Rust workspace was established.

Workspace configuration:

```toml
[workspace]
members = [
    "crates/node-agent",
]
resolver = "3"
```

The initial Rust project structure:

```text
Nebula/
├── crates/
│   └── node-agent/
│       └── src/
│           └── main.rs
├── vendor/
│   └── llama.cpp/
├── docs/
├── Cargo.toml
└── Cargo.lock
```

### Design Decision

Nebula system components will initially be implemented in Rust.

Rust was selected because the project requires:

- Systems programming
- Concurrency
- Networking
- Memory safety
- Low-level operating-system interaction
- Distributed runtime development

### Commit

```text
feat: establish phase 1 rust workplace
```

Commit hash:

```text
TBD
```

---

# 9. Initial Node Agent

## Milestone 0.6 — Node Agent

**Status:** Complete

The first Nebula Node Agent was implemented.

The Node Agent is intended to run on every device participating in a Nebula cluster.

Its long-term responsibilities include:

- Identifying the device
- Detecting available resources
- Communicating with other nodes
- Receiving workloads
- Executing workloads
- Reporting status
- Eventually loading model shards

Initial milestone:

```text
nebula-agent
    ↓
identify itself
    ↓
detect CPU
    ↓
detect RAM
    ↓
report READY
```

---

# 10. Node Resource Detection

## Milestone 0.7 — CPU and Memory Detection

**Status:** Complete

The Node Agent was extended to detect basic system resources.

Current resource model:

```text
NodeResources
├── CPU cores
├── CPU utilization
├── Total RAM
└── Available RAM
```

## CPU Cores

CPU parallelism is detected using:

```rust
std::thread::available_parallelism()
```

This allows the agent to determine the amount of CPU parallelism exposed by the operating system.

---

## Memory

Memory information is obtained from:

```text
/proc/meminfo
```

The agent currently reads:

```text
MemTotal
MemAvailable
```

The distinction between total and available memory is important for scheduling.

For example:

```text
Node
├── Total RAM: 16 GB
└── Available RAM: 10 GB
```

The scheduler should consider the available memory rather than assuming the complete physical memory capacity is available.

---

## CPU Utilization

CPU utilization is obtained from:

```text
/proc/stat
```

The agent samples CPU statistics over a short interval and calculates CPU utilization from the difference between the two samples.

Conceptually:

```text
CPU usage =
    1 - (idle time delta / total CPU time delta)
```

The resulting value is reported as a percentage.

---

## Current Node Agent Output

The Node Agent reports information similar to:

```text
Nebula Node Agent
=================

CPU cores       : 8
CPU usage       : XX.XX%
Total RAM       : XXXXX MB
Available RAM   : XXXXX MB

READY
```

Exact values vary depending on current system state.

### Commit

```text
feat: add initial nebula node agent
```

Commit hash:

```text
TBD
```

---

# 11. Initial Networking Layer

## Milestone 0.8 — TCP Networking

**Status:** Complete

The first networking layer was implemented using Rust's standard library.

Current networking APIs:

```rust
std::net::TcpListener
std::net::TcpStream
```

No asynchronous runtime is currently required.

This is intentional.

The first networking milestones are intended to establish an understanding of:

- TCP connections
- Socket lifecycle
- Blocking I/O
- Client/server communication
- Message boundaries
- Request/response communication

---

# 12. Networking Structure

Current Node Agent structure:

```text
crates/
└── node-agent/
    └── src/
        ├── main.rs
        └── network/
            ├── mod.rs
            ├── server.rs
            └── client.rs
```

### `network/mod.rs`

Exports:

```rust
pub mod client;
pub mod server;
```

---

# 13. TCP Server

## `server.rs`

The TCP server uses:

```rust
TcpListener
```

Its current responsibilities are:

1. Bind to a TCP address.
2. Listen for incoming connections.
3. Accept a connection.
4. Read incoming data.
5. Print the received message.
6. Send a response.

Conceptually:

```text
TCP Listener
     │
     ▼
Incoming connection
     │
     ▼
Read message
     │
     ▼
Process message
     │
     ▼
Send response
```

---

# 14. TCP Client

## `client.rs`

The TCP client uses:

```rust
TcpStream
```

Its current responsibilities are:

1. Connect to a TCP address.
2. Send a message.
3. Read the response.
4. Print the response.

Conceptually:

```text
TcpStream
    │
    ▼
Connect
    │
    ▼
Send message
    │
    ▼
Read response
```

---

# 15. Initial TCP Message Test

The initial networking implementation uses a simple message exchange to prove that TCP communication works.

Initial message:

```text
HELLO_FROM_NEBULA
```

The server responds with:

```text
HELLO_FROM_NEBULA
```

The purpose of this test is not to establish a production protocol.

It only verifies the underlying communication path.

---

# 16. Networking Design Decision

At this stage, Nebula is intentionally using:

```text
Rust standard library
        +
TCP
```

rather than immediately introducing:

```text
Tokio
async/await
complex networking frameworks
```

The initial goal is to understand the networking primitives first.

Async networking can be introduced later when the system has a concrete requirement for high connection concurrency or asynchronous workload management.

---

# 17. Current Architecture

The current conceptual architecture is:

```text
Nebula
│
├── node-agent
│   │
│   ├── resource detection
│   │   ├── CPU
│   │   └── memory
│   │
│   └── network
│       ├── TCP server
│       └── TCP client
│
├── coordinator       [future]
├── scheduler         [future]
├── transport         [future]
└── inference         [future]
```

---

# 18. Current Node Agent Architecture

Current:

```text
                 Node Agent
                     │
          ┌──────────┴──────────┐
          │                     │
      Resources              Network
          │                     │
    ┌─────┴─────┐         ┌─────┴─────┐
    │           │         │           │
   CPU          RAM      Server      Client
```

Future:

```text
                       Node Agent
                           │
       ┌───────────┬───────┼────────┬───────────┐
       │           │       │        │           │
   Identity    Resources Network  Workload   Runtime
                   │
          ┌────────┼─────────┐
          │        │         │
         CPU      RAM     Accelerators
```

---

# 19. Git History

Current major commits:

| Order | Commit | Milestone |
|---:|---|---|
| 1 | `bench: establish single-node inference baseline` | Single-node LLM benchmark |
| 2 | `initial project commit` | Initial Nebula repository |
| 3 | `feat: establish phase 1 rust workplace` | Rust workspace |
| 4 | `feat: add initial nebula node agent` | Node Agent |
| 5 | `feat: add initial tcp networking` | Initial TCP networking |

Commit hashes:

```text
bench: establish single-node inference baseline
Hash: TBD

initial project commit
Hash: TBD

feat: establish phase 1 rust workplace
Hash: TBD

feat: add initial nebula node agent
Hash: TBD

feat: add initial tcp networking
Hash: TBD
```

Hashes can be retrieved with:

```bash
git log --oneline --decorate --graph
```

---

# 20. Current Project State

At the current checkpoint, Nebula can:

```text
✓ Build a Rust workspace
✓ Run the Node Agent
✓ Detect CPU parallelism
✓ Detect total RAM
✓ Detect available RAM
✓ Detect CPU utilization
✓ Build llama.cpp
✓ Run a local Qwen2.5-7B model
✓ Benchmark local inference
✓ Provide an initial TCP server
✓ Provide an initial TCP client
```

Nebula cannot yet:

```text
✗ Discover other Nebula nodes
✗ Identify remote nodes
✗ Exchange structured node information
✗ Advertise resources
✗ Maintain heartbeats
✗ Detect failed nodes
✗ Schedule workloads
✗ Execute workloads remotely
✗ Split models across nodes
✗ Perform distributed inference
```

---

# 21. Next Development Phase — Node-to-Node Protocol

## Milestone 1.1 — HELLO / HELLO_ACK

**Status:** Next

The next objective is to make two actual Nebula Node Agents communicate.

Target:

```text
Node A                              Node B
  │                                   │
  │────── TCP connection ────────────>│
  │                                   │
  │────────── HELLO ─────────────────>│
  │                                   │
  │<───────── HELLO_ACK ──────────────│
  │                                   │
```

The first structured protocol messages will be:

```text
HELLO
HELLO_ACK
```

The purpose is to establish that:

- Node A can connect to Node B.
- Node B can recognize a Nebula connection.
- Node B can respond.
- The protocol is independent of the underlying test tools.

---

# 22. Planned Networking Milestones

## Milestone 1.1 — HELLO / HELLO_ACK

```text
Node A
  │
  │ HELLO
  ▼
Node B
  │
  │ HELLO_ACK
  ▼
Node A
```

---

## Milestone 1.2 — Node Identity

Every Nebula node will require a persistent identity.

Conceptual structure:

```text
NodeIdentity
├── node_id
├── hostname
├── platform
└── capabilities
```

---

## Milestone 1.3 — Node Information Exchange

Nodes should be able to exchange basic information:

```text
NODE_INFO
```

Example conceptual information:

```text
Node
├── node_id
├── hostname
├── operating system
├── architecture
└── capabilities
```

---

## Milestone 1.4 — Resource Advertisement

Nodes should be able to communicate their current resources.

Example:

```text
Node A
├── CPU cores
├── CPU utilization
├── Total RAM
├── Available RAM
└── Accelerator information
```

This will eventually provide the scheduler with the information required to make placement decisions.

---

## Milestone 1.5 — Node Discovery

Nebula nodes should be able to discover other Nebula nodes on the same network.

Target:

```text
Node A
 │
 ├── discovers Node B
 ├── discovers Node C
 └── discovers Node D
```

The initial implementation will focus on local-network discovery.

---

## Milestone 1.6 — Heartbeats

Nodes will periodically exchange health information.

Conceptually:

```text
Node A ───── HEARTBEAT ─────> Node B
Node A <──── HEARTBEAT ────── Node B
```

Heartbeats will eventually allow Nebula to determine whether a node is:

```text
ONLINE
DEGRADED
UNRESPONSIVE
OFFLINE
```

---

## Milestone 1.7 — Failure Detection

The system must detect when a device disappears.

Example:

```text
Node A
  │
  │ heartbeat
  ▼
Node B

Node B suddenly disappears

  ↓

Node A detects timeout

  ↓

Node B marked unavailable
```

This becomes important once workloads are distributed across multiple devices.

---

# 23. Planned Distributed Runtime

After the networking foundation is complete:

```text
Networking
    ↓
Node Identity
    ↓
Discovery
    ↓
Resource Advertisement
    ↓
Heartbeats
    ↓
Failure Detection
    ↓
Coordinator
    ↓
Scheduler
    ↓
Remote Workload Execution
```

---

# 24. Future Scheduler

The scheduler will eventually need to consider heterogeneous resources.

Potential scheduling inputs:

```text
CPU availability
RAM availability
GPU availability
NPU availability
Network latency
Network bandwidth
Model availability
Model cache
Device reliability
Battery state
Thermal state
Power consumption
Workload requirements
```

The objective is not simply:

```text
"Which node has the most CPU?"
```

Instead, the scheduler should eventually answer:

```text
"Which combination of available devices provides
the best execution strategy for this workload?"
```

---

# 25. Distributed Inference

The first major distributed workload will be LLM inference.

Conceptual target:

```text
                    Nebula
                      │
                 Coordinator
                      │
                  Scheduler
                      │
          ┌───────────┼───────────┐
          │           │           │
          ▼           ▼           ▼
       Node A       Node B      Node C
          │           │           │
       Shard A      Shard B     Shard C
          │           │           │
          └───────────┼───────────┘
                      │
                      ▼
                   Output
```

The system should eventually determine how model computation can be distributed across heterogeneous devices.

---

# 26. Long-Term Runtime Direction

Nebula should eventually evolve beyond distributed LLM inference.

Potential workloads include:

```text
LLM inference
Image generation
Speech processing
Video processing
Distributed compilation
Scientific workloads
Data processing
Other AI workloads
General compute workloads
```

LLM inference is the first workload because it exercises several difficult distributed-system problems simultaneously:

- Memory requirements
- Compute requirements
- Heterogeneous hardware
- Inter-device communication
- Scheduling
- Model placement
- Fault tolerance

---

# 27. Important Architectural Principles

## Principle 1 — Device abstraction

Users should not need to manually manage:

```text
model shards
node assignments
workload placement
device capabilities
```

Nebula should handle these internally.

---

## Principle 2 — Heterogeneous devices

Nebula should not assume that all nodes have identical hardware.

A cluster may contain:

```text
Laptop
Phone
Desktop
Raspberry Pi
NAS
Server
```

with different:

```text
CPU
GPU
NPU
RAM
Network
Power
Thermal characteristics
```

---

## Principle 3 — Failure is normal

Devices can:

- disconnect
- sleep
- lose network connectivity
- become thermally constrained
- run out of memory
- become overloaded
- run out of battery

The runtime must eventually treat node failure as an expected operating condition rather than an exceptional event.

---

## Principle 4 — Measure before optimizing

Every major distributed configuration should be benchmarked.

The original single-node benchmark provides the reference point.

Future benchmarks should compare:

```text
Single Node
     vs
2 Nodes
     vs
3+ Nodes
```

and measure:

- Throughput
- Latency
- Network overhead
- CPU utilization
- Memory utilization
- Scaling efficiency
- Failure recovery time

---

# 28. Current Checkpoint

```text
Phase 0 — Foundation
        │
        ├── Project setup              ✓
        ├── Rust workspace             ✓
        ├── llama.cpp                  ✓
        ├── Local LLM                  ✓
        ├── Baseline benchmark         ✓
        ├── Node Agent                 ✓
        ├── Resource detection         ✓
        └── Initial TCP layer          ✓

Phase 1 — Networking
        │
        ├── TCP communication          NEXT
        ├── HELLO / HELLO_ACK          NEXT
        ├── Node identity              TODO
        ├── Resource advertisement     TODO
        ├── Discovery                  TODO
        ├── Heartbeats                 TODO
        └── Failure detection          TODO
```

---

# 29. Immediate Next Step

The immediate development target is:

```text
Two Nebula Node Agents
        │
        │ TCP
        ▼
      HELLO
        │
        ▼
    HELLO_ACK
```

Once this works, the next step will be exchanging actual Node Agent identity and resource information.

---

# 30. Change Log

## Current Session

### Completed

- Created initial Rust Node Agent.
- Added CPU detection.
- Added total RAM detection.
- Added available RAM detection.
- Added CPU utilization detection.
- Added networking module.
- Added TCP server.
- Added TCP client.
- Established a Git checkpoint for the TCP networking layer.

### Current Commit

```text
feat: add initial tcp networking
```

Hash:

```text
TBD
```

### Next

Implement the first actual Nebula node-to-node protocol:

```text
HELLO
HELLO_ACK
```
