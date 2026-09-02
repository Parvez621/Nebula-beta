# Phase 1 --- Two-Node Pipeline Split

## Objective

Implement the first distributed inference version of Nebula:

-   **Node A (head):** runs Qwen2.5-7B-Instruct layers 0--13.
-   **Node B (tail):** runs layers 14--27 plus final normalization and
    LM head.
-   Nodes communicate over **raw TCP**.
-   Activations are serialized using **Protocol Buffers**.
-   The distributed result must match the Phase 0 single-node golden
    reference within a defined floating-point tolerance.

Phase 1 is intentionally limited to a **fixed two-node pipeline**.
Discovery, dynamic scheduling, fault tolerance, security, mobile
clients, and QUIC are deferred until Phase 1 correctness is established.

------------------------------------------------------------------------

## Phase 1 Architecture

``` text
                         Qwen2.5-7B-Instruct
                                  │
                    ┌─────────────┴─────────────┐
                    │                           │
                 Node A                      Node B
                  HEAD                         TAIL
              Layers 0–13                 Layers 14–27
                    │                           │
                    │      Activation          │
                    └────── TCP + Protobuf ────►
                                                │
                                         Final Norm
                                         + LM Head
                                                │
                                             Output
```

### Fixed split

  -----------------------------------------------------------------------
  Node                                Responsibility
  ----------------------------------- -----------------------------------
  Node A                              Load model, execute layers 0--13,
                                      serialize activation

  Node B                              Receive activation, execute layers
                                      14--27, final norm, LM head

  Transport                           TCP connection between A and B

  Serialization                       Protocol Buffers
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Milestones

## Milestone 1 --- Cargo Workspace and Repository Structure

### Goal

Create the Rust workspace and establish clear crate boundaries.

### Target structure

``` text
crates/
├── inference-core/
├── transport/
├── node-agent/
└── proto/
```

### Tasks

-   [ ] Create root `Cargo.toml` workspace.
-   [ ] Create `crates/inference-core`.
-   [ ] Create `crates/transport`.
-   [ ] Create `crates/node-agent`.
-   [ ] Create `crates/proto`.
-   [ ] Verify all crates compile.
-   [ ] Keep `vendor/llama.cpp` as the existing dependency/submodule.
-   [ ] Confirm the existing Phase 0 build still works.

### Definition of done

``` bash
cargo check --workspace
```

passes successfully.

Phase 0 inference remains functional.

------------------------------------------------------------------------

# Milestone 2 --- Protobuf Activation Protocol

### Goal

Define the wire format used to transfer activations between nodes.

### Initial message

The protocol should represent the activation tensor needed by Node B.

Conceptually:

``` text
Activation
├── sequence_length
├── hidden_size
└── data[]
```

For Qwen2.5-7B:

``` text
hidden_size = 3584
```

### Tasks

-   [ ] Create the `.proto` schema.
-   [ ] Define an activation message.
-   [ ] Represent tensor dimensions explicitly.
-   [ ] Represent activation data.
-   [ ] Add enough metadata to validate the message.
-   [ ] Generate Rust protobuf bindings.
-   [ ] Add serialization tests.
-   [ ] Add deserialization tests.
-   [ ] Test malformed/incomplete messages.

### Definition of done

A Rust activation can be:

``` text
Rust tensor
    ↓
protobuf serialization
    ↓
bytes
    ↓
protobuf deserialization
    ↓
identical tensor
```

with dimensions and values preserved.

------------------------------------------------------------------------

# Milestone 3 --- TCP Transport Layer

### Goal

Build a reliable TCP transport for sending protobuf messages.

### Architecture

``` text
Node A
  │
  │ TCP
  ▼
Node B
```

### Tasks

-   [ ] Implement TCP server/listener.
-   [ ] Implement TCP client/connector.
-   [ ] Define message framing.
-   [ ] Send protobuf messages over TCP.
-   [ ] Receive complete messages.
-   [ ] Handle partial TCP reads/writes correctly.
-   [ ] Detect connection closure.
-   [ ] Return transport errors cleanly.
-   [ ] Add unit tests for framing.
-   [ ] Add a local loopback integration test.

### Important

TCP is a byte stream. One `read()` does not necessarily correspond to
one protobuf message.

Therefore Phase 1 needs explicit framing, for example:

``` text
[length][protobuf payload]
```

### Definition of done

A test can send an activation from a TCP client and reconstruct the
exact activation on the server.

------------------------------------------------------------------------

# Milestone 4 --- Two-Node Agent

### Goal

Create the executable that can operate as either a head or tail node.

### Example

``` bash
node-agent --role head ...
```

and:

``` bash
node-agent --role tail ...
```

### Head responsibilities

``` text
Load model
   ↓
Run layers 0–13
   ↓
Create activation
   ↓
Send activation to Node B
```

### Tail responsibilities

``` text
Listen for Node A
   ↓
Receive activation
   ↓
Run layers 14–27
   ↓
Final norm
   ↓
LM head
   ↓
Produce output
```

### Tasks

-   [ ] Add CLI argument parsing.
-   [ ] Add `head` role.
-   [ ] Add `tail` role.
-   [ ] Configure model path.
-   [ ] Configure listen address/port.
-   [ ] Configure peer address/port.
-   [ ] Connect node-agent to transport crate.
-   [ ] Connect node-agent to inference-core.

### Definition of done

Two processes can be started manually and establish a connection:

``` text
Node A (head)  ───────── TCP ─────────►  Node B (tail)
```

------------------------------------------------------------------------

# Milestone 5 --- Split Qwen Inference

### Goal

Replace the transport test payload with a real Qwen activation.

The model is:

``` text
Qwen2.5-7B-Instruct
Q4_K_M
28 transformer layers
hidden size: 3584
```

The fixed split is:

``` text
Node A:
layers 0–13

Node B:
layers 14–27
+ final norm
+ LM head
```

### Tasks

-   [ ] Identify the existing llama.cpp model/context interfaces.
-   [ ] Determine how to execute a partial layer range.
-   [ ] Implement execution of layers 0--13.
-   [ ] Extract the intermediate activation.
-   [ ] Serialize the activation.
-   [ ] Send it to Node B.
-   [ ] Reconstruct the activation on Node B.
-   [ ] Execute layers 14--27.
-   [ ] Execute final normalization.
-   [ ] Execute LM head.
-   [ ] Produce logits/output.

### Definition of done

The complete inference path works across two machines/processes:

``` text
Prompt
  ↓
Node A
  ↓
Layers 0–13
  ↓
Activation
  ↓
TCP + Protobuf
  ↓
Node B
  ↓
Layers 14–27
  ↓
Final Norm + LM Head
  ↓
Output
```

------------------------------------------------------------------------

# Milestone 6 --- Distributed Correctness

### Goal

Prove that distributed inference produces the same result as the Phase 0
baseline.

The Phase 0 output is the golden reference for Phase 1 correctness.

### Comparison

``` text
                Same prompt
                     │
          ┌──────────┴──────────┐
          │                     │
     Phase 0                Phase 1
   Single node           Node A → Node B
          │                     │
          ▼                     ▼
      Output A              Output B
          │                     │
          └─────────┬───────────┘
                    ▼
              Compare outputs
```

### Tasks

-   [ ] Define the exact test prompt.
-   [ ] Capture/reference Phase 0 output.
-   [ ] Run the same prompt through distributed inference.
-   [ ] Compare logits or the appropriate output representation.
-   [ ] Define floating-point tolerance.
-   [ ] Report maximum/mean numerical difference where applicable.
-   [ ] Verify generated output matches the expected result.
-   [ ] Add an automated correctness test.

### Definition of done

Distributed inference matches the Phase 0 golden reference within the
agreed numerical tolerance.

**Correctness takes priority over performance at this milestone.**

------------------------------------------------------------------------

# Milestone 7 --- Prefill and Decode Validation

### Goal

Validate both major inference workloads.

Phase 1 should not only work for a tiny single-token case.

### Prefill

Test prompts containing multiple tokens.

The activation transferred during prefill can be large:

``` text
[sequence_length, 3584]
```

### Decode

Test token-by-token generation after the initial prefill.

### Tasks

-   [ ] Test short prompt.
-   [ ] Test 512-token prompt.
-   [ ] Test single-token decode.
-   [ ] Test multiple generated tokens.
-   [ ] Verify correctness for both paths.
-   [ ] Check activation sizes.
-   [ ] Check repeated TCP transfers.

### Definition of done

Both prefill and decode produce correct distributed results.

------------------------------------------------------------------------

# Milestone 8 --- Network Performance Measurement

### Goal

Measure the actual cost of distributing the model across two machines.

The current target is:

``` text
< 20 ms overhead per network hop on LAN
```

At the Phase 0 generation rate of approximately 6.3 tok/s, one token
takes roughly 159 ms, so the network hop should remain a relatively
small portion of token latency.

### Tasks

-   [ ] Measure serialization time.
-   [ ] Measure TCP send time.
-   [ ] Measure network round-trip/one-way transfer time where
    appropriate.
-   [ ] Measure deserialization time.
-   [ ] Measure total hop overhead.
-   [ ] Measure prefill activation transfer.
-   [ ] Measure decode activation transfer.
-   [ ] Record results in `docs/benchmarks/`.

### Definition of done

A reproducible benchmark reports:

``` text
serialization
+ network transfer
+ deserialization
= activation transfer overhead
```

and the result can be compared against the `<20 ms` target.

------------------------------------------------------------------------

# Milestone 9 --- Failure and Edge-Case Handling

### Goal

Make the fixed two-node prototype behave predictably when something goes
wrong.

This is **not** full fault tolerance. Fault-tolerant distributed
scheduling is explicitly outside Phase 1.

### Tasks

-   [ ] Handle Node B unavailable.
-   [ ] Handle connection refusal.
-   [ ] Handle connection closure.
-   [ ] Handle incomplete messages.
-   [ ] Handle invalid protobuf data.
-   [ ] Validate activation dimensions.
-   [ ] Validate expected hidden size.
-   [ ] Report useful errors.
-   [ ] Ensure processes terminate cleanly.

### Definition of done

Expected failures produce controlled errors rather than crashes, hangs,
or corrupted inference.

------------------------------------------------------------------------

# Milestone 10 --- Phase 1 Integration Test

### Goal

Run the entire system as one reproducible test.

### Test

Start:

``` text
Node B
  ↓
listen on TCP port
```

Then:

``` text
Node A
  ↓
load prompt
  ↓
layers 0–13
  ↓
TCP + protobuf
  ↓
Node B
  ↓
layers 14–27
  ↓
final norm + LM head
  ↓
output
```

Compare the result against Phase 0.

### Tasks

-   [ ] Create an end-to-end test procedure.
-   [ ] Run Node A and Node B on the same machine.
-   [ ] Run Node A and Node B on two LAN machines.
-   [ ] Verify distributed correctness.
-   [ ] Record latency.
-   [ ] Record throughput.
-   [ ] Record hardware/network configuration.
-   [ ] Document the exact commands needed to reproduce the test.

### Definition of done

A fresh setup can reproduce the two-node inference experiment using the
documented commands.

------------------------------------------------------------------------

# Phase 1 Final Acceptance Criteria

Phase 1 is complete only when all of the following are true:

-   [ ] Rust workspace builds successfully.
-   [ ] Protobuf activation messages serialize/deserialize correctly.
-   [ ] TCP transport reliably transfers framed messages.
-   [ ] Head and tail node-agent processes communicate.
-   [ ] Node A executes layers 0--13.
-   [ ] Node B executes layers 14--27.
-   [ ] Node B executes final norm + LM head.
-   [ ] Distributed inference produces correct output.
-   [ ] Distributed output matches the Phase 0 golden reference within
    tolerance.
-   [ ] Prefill is validated.
-   [ ] Decode is validated.
-   [ ] LAN network overhead is measured.
-   [ ] The `<20 ms` per-hop target is evaluated.
-   [ ] Basic transport/inference failures are handled cleanly.
-   [ ] Two-machine LAN execution is reproducible.
-   [ ] Documentation and benchmark results are committed.

------------------------------------------------------------------------

# Explicitly Out of Scope

Do **not** implement these during Phase 1:

-   mDNS/service discovery
-   Dynamic node discovery
-   Dynamic layer scheduling
-   Weighted scheduling
-   Load balancing
-   Raft
-   Fault-tolerant cluster management
-   Transport security
-   Authentication
-   Mobile clients
-   Cross-platform mobile runtime
-   QUIC migration

These should only be considered after the fixed two-node pipeline is
correct.

------------------------------------------------------------------------

# Recommended Implementation Order

``` text
M1  Workspace
 ↓
M2  Protobuf
 ↓
M3  TCP transport
 ↓
M4  Node agent
 ↓
M5  Split Qwen inference
 ↓
M6  Correctness
 ↓
M7  Prefill + decode
 ↓
M8  Performance
 ↓
M9  Error handling
 ↓
M10 End-to-end integration
 ↓
     PHASE 1 COMPLETE
```

## Golden Rule

**Do not optimize the distributed runtime before proving correctness.**

The first objective is:

``` text
Single-node output
        ==
Two-node output
```

within the defined numerical tolerance.

Once that is proven, optimize the network and pipeline.
