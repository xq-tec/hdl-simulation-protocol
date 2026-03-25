# hdl-simulation-protocol

- [1. Overview](#1-overview)
- [2. Message Flow](#2-message-flow)
- [3. Protocol Modules](#3-protocol-modules)
  - [3.1. Commands (`to_simulator`)](#31-commands-to_simulator)
    - [3.1.1. `Command` Enum](#311-command-enum)
    - [3.1.2. `SignalTrackingRequest`](#312-signaltrackingrequest)
  - [3.2. Updates (`from_simulator`)](#32-updates-from_simulator)
    - [3.2.1. `SimulationUpdate` Enum](#321-simulationupdate-enum)
    - [3.2.2. `EventsUpdate`](#322-eventsupdate)
    - [3.2.3. `SignalEvents`](#323-signalevents)
    - [3.2.4. `Event`](#324-event)
    - [3.2.5. `RawValue`](#325-rawvalue)
  - [3.3. Design Hierarchy (`design_hierarchy`)](#33-design-hierarchy-design_hierarchy)
    - [3.3.1. `DesignHierarchy`](#331-designhierarchy)
    - [3.3.2. `Module`](#332-module)
    - [3.3.3. `ModuleKind`](#333-modulekind)
    - [3.3.4. `Signal`](#334-signal)
    - [3.3.5. `SignalInstanceId`](#335-signalinstanceid)
    - [3.3.6. `SignalType`](#336-signaltype)
    - [3.3.7. `Direction`](#337-direction)
  - [3.4. Time Representation (`time`)](#34-time-representation-time)
    - [3.4.1. `LogicalTime`](#341-logicaltime)
    - [3.4.2. `PhysicalTime`](#342-physicaltime)
    - [3.4.3. `Delta`](#343-delta)
  - [3.5. Logic Values (`Logic` enum)](#35-logic-values-logic-enum)
  - [3.6. Simulation Status](#36-simulation-status)
- [4. Serialization](#4-serialization)
  - [4.1. Human-Readable Formats (JSON)](#41-human-readable-formats-json)
  - [4.2. Binary Formats (Postcard)](#42-binary-formats-postcard)
  - [4.3. Custom Serialization (`serde_utils`)](#43-custom-serialization-serde_utils)
- [5. Usage Examples](#5-usage-examples)
  - [5.1. Sending Commands](#51-sending-commands)
  - [5.2. Receiving Updates](#52-receiving-updates)
  - [5.3. Working with Time](#53-working-with-time)
- [6. Dependencies](#6-dependencies)
- [7. Design Rationale](#7-design-rationale)
  - [7.1. Dense Signal IDs](#71-dense-signal-ids)
  - [7.2. LogicalTime Ordering](#72-logicaltime-ordering)
  - [7.3. RawValue Encoding](#73-rawvalue-encoding)
  - [7.4. Time in Femtoseconds](#74-time-in-femtoseconds)
- [8. Limitations and Future Work](#8-limitations-and-future-work)
- [9. License](#9-license)

## 1. Overview

Communication protocol for controlling HDL simulations and subscribing to simulation events.
`hdl-simulation-protocol` provides type-safe Rust definitions for bidirectional communication between:

- **Simulator**: GHDL adapter (producer of events, consumer of commands)
- **Controller**: typically a waveform viewer (consumer of events, producer of commands)

The protocol uses Serde for serialization, supporting both JSON (for debugging) and binary formats (for efficiency).

## 2. Message Flow

```plain
┌────────────┐                                    ┌────────────────────────────┐
│ Controller │◀────────────────────────────────▶│ Simulator (GHDL Adapter)   │
└────────────┘                                    └────────────────────────────┘
      │                                                         │
      │  Commands                                               │
      ├───────────────────────────────────────────────────────▶│
      │  • StartSimulation                                      │
      │  • PauseSimulation                                      │
      │  • TrackSignals                                         │
      │                                                         │
      │  Updates                                                │
      │◀───────────────────────────────────────────────────────┤
      │   • DesignHierarchy                                     │
      │   • Events (signal changes)                             │
      │   • Status changes                                      │
```

## 3. Protocol Modules

### 3.1. Commands (`to_simulator`)

Commands sent from controller to simulator to control execution and configure tracking.

#### 3.1.1. `Command` Enum

```rust
pub enum Command {
    StartSimulation,
    PauseSimulation,
    ResumeSimulation,
    RestartSimulation,
    StopSimulation,
    TrackSignals(SignalTrackingRequest),
}
```

#### 3.1.2. `SignalTrackingRequest`

Requests the simulator to track (or stop tracking) specific signals:

```rust
pub struct SignalTrackingRequest {
    pub signal_instance_ids: Vec<SignalInstanceId>,
    pub enabled: bool,    // true = track, false = untrack
    pub subscribe: bool,  // internal subscription flag
}
```

**Usage:**

- `enabled=true, subscribe=true`: Start tracking signals, receive events
- `enabled=false, subscribe=false`: Stop tracking signals (TODO: not fully implemented)

### 3.2. Updates (`from_simulator`)

Updates sent from simulator to controller to report events and status.

#### 3.2.1. `SimulationUpdate` Enum

```rust
pub enum SimulationUpdate {
    SimulationStarted,
    SimulationPaused,
    SimulationResumed,
    SimulationStopped,
    DesignHierarchy(DesignHierarchy),
    Events(EventsUpdate),
}
```

#### 3.2.2. `EventsUpdate`

Batch of signal events within a time range:

```rust
pub struct EventsUpdate {
    pub time_range: Range<LogicalTime>,
    pub signals: Vec<SignalEvents>,
}
```

**Semantics:**

- All events occurred within `time_range`
- Each `SignalEvents` contains events for one signal
- Events are ordered by time within each signal
- Multiple signals can have events at the same time

#### 3.2.3. `SignalEvents`

Events for a single signal:

```rust
pub struct SignalEvents {
    pub signal_instance_id: SignalInstanceId,
    pub events: Vec<Event>,
}
```

#### 3.2.4. `Event`

Individual signal value change:

```rust
pub struct Event {
    pub time: LogicalTime,
    pub value: RawValue,
}
```

#### 3.2.5. `RawValue`

Raw 64-bit representation of signal value:

```rust
pub struct RawValue(pub u64);
```

**Encoding:**

- **Logic/Bit**: Lower 4 bits encode logic state (0-8)
- **Integer (i32)**: Lower 32 bits as signed integer
- **Integer (i64)**: Full 64 bits as signed integer
- **Real (f64)**: IEEE 754 double-precision bits

### 3.3. Design Hierarchy (`design_hierarchy`)

Describes the structure of the elaborated design.

#### 3.3.1. `DesignHierarchy`

Root of the design tree:

```rust
pub struct DesignHierarchy {
    /// 53-bit instance id (matches marker filename id); set before sending to clients.
    pub simulation_id: u64,
    pub root_modules: Vec<Module>,
}
```

#### 3.3.2. `Module`

Entity instance or package:

```rust
pub struct Module {
    pub name: CompactString,      // Instance name
    pub kind: ModuleKind,          // Entity or Package
    pub submodules: Vec<Module>,   // Child instances
    pub signals: Vec<Signal>,      // Signals in this scope
}
```

#### 3.3.3. `ModuleKind`

```rust
pub enum ModuleKind {
    DesignEntity {
        entity: CompactString,
        architecture: CompactString,
    },
    Package,
}
```

#### 3.3.4. `Signal`

Signal or variable declaration:

```rust
pub struct Signal {
    pub name: CompactString,
    pub id: SignalInstanceId,
    pub typ: SignalType,
}
```

#### 3.3.5. `SignalInstanceId`

Unique identifier for an instantiated signal:

```rust
pub struct SignalInstanceId(pub NonZeroU32);
```

**Properties:**

- Assigned by simulator during elaboration
- Only valid for the duration of one simulation
- Dense allocation (1, 2, 3, ...)
- Cannot be zero (uses `NonZeroU32`)

#### 3.3.6. `SignalType`

Type information for signals:

```rust
pub enum SignalType {
    Bit,
    Logic,  // IEEE 1164: U, X, 0, 1, Z, W, L, H, -
    Integer {
        min: i64,
        max: i64,
        direction: Direction,
    },
    Real {
        min: f64,
        max: f64,
        direction: Direction,
    },
    Array {
        left: i32,
        right: i32,
        direction: Direction,
        element_type: Box<SignalType>,
    },
}
```

#### 3.3.7. `Direction`

Array/range direction:

```rust
pub enum Direction {
    To,      // left TO right (ascending)
    Downto,  // left DOWNTO right (descending)
}
```

### 3.4. Time Representation (`time`)

VHDL simulation time with delta cycle support.

#### 3.4.1. `LogicalTime`

Complete simulation time including delta cycles:

```rust
pub struct LogicalTime {
    pub physical: PhysicalTime,
    pub delta: Delta,
}
```

**Ordering:**

- First by physical time
- Then by delta cycle
- Example: (100fs, δ0) < (100fs, δ1) < (101fs, δ0)

#### 3.4.2. `PhysicalTime`

Physical simulation time in femtoseconds:

```rust
pub struct PhysicalTime(pub u64);
```

**Units:**

- Base unit: femtoseconds (1e-15 seconds)
- Range: 0 to ~584 years (u64::MAX femtoseconds)
- Typical VHDL time literals:
  - 1 ns = 1,000,000 fs
  - 1 us = 1,000,000,000 fs
  - 1 ms = 1,000,000,000,000 fs

#### 3.4.3. `Delta`

Delta cycle within a physical time:

```rust
pub struct Delta(pub u64);
```

**Semantics:**

- Represents zero-time simulation steps
- Used for signal resolution and event ordering
- Delta cycles occur when signals change without time advancing
- Important for VHDL semantics (concurrent signal assignments)

### 3.5. Logic Values (`Logic` enum)

IEEE 1164 standard 9-state logic:

```rust
#[repr(u8)]
pub enum Logic {
    U = 0,        // Uninitialized
    X = 1,        // Strong unknown
    Zero = 2,     // Strong 0
    One = 3,      // Strong 1
    Z = 4,        // High impedance
    W = 5,        // Weak unknown
    L = 6,        // Weak 0
    H = 7,        // Weak 1
    DontCare = 8, // Don't care (-)
}
```

**Display:**

- Formats as standard VHDL characters: 'U', 'X', '0', '1', 'Z', 'W', 'L', 'H', '-'

**Parsing:**

- Implements `FromStr` for string conversion
- Implements `TryFrom<u8>` for integer conversion

### 3.6. Simulation Status

Current state of the simulator:

```rust
#[repr(u8)]
pub enum SimulationStatus {
    Paused = 0,   // Waiting for resume command
    Running = 1,  // Actively simulating
    Stopped = 2,  // Simulation ended
}
```

## 4. Serialization

The protocol uses Serde with custom serialization:

### 4.1. Human-Readable Formats (JSON)

For debugging and logging:

- `PhysicalTime` and `Delta` serialize as string-encoded integers
- Example: `"12345"` instead of `12345`
- Prevents precision loss in JSON parsers

### 4.2. Binary Formats (Postcard)

For efficient transmission:

- All types serialize as native binary representation
- Compact encoding with no overhead
- Used in WebSocket communication

### 4.3. Custom Serialization (`serde_utils`)

Provides format-aware serialization:

```rust
fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer
{
    if serializer.is_human_readable() {
        // JSON: serialize as string
        value.to_string().serialize(serializer)
    } else {
        // Binary: serialize as u64
        value.serialize(serializer)
    }
}
```

## 5. Usage Examples

### 5.1. Sending Commands

```rust
use hdl_simulation_protocol::to_simulator::{Command, SignalTrackingRequest};
use hdl_simulation_protocol::design_hierarchy::SignalInstanceId;

// Start simulation
let cmd = Command::StartSimulation;
let json = serde_json::to_string(&cmd)?;

// Track signals
let cmd = Command::TrackSignals(SignalTrackingRequest {
    signal_instance_ids: vec![
        SignalInstanceId(NonZeroU32::new(1).unwrap()),
        SignalInstanceId(NonZeroU32::new(2).unwrap()),
    ],
    enabled: true,
    subscribe: true,
});
```

### 5.2. Receiving Updates

```rust
use hdl_simulation_protocol::from_simulator::SimulationUpdate;

// Parse update from simulator
let update: SimulationUpdate = serde_json::from_str(&json)?;

match update {
    SimulationUpdate::Events(events) => {
        println!("Events in time range: {:?}", events.time_range);
        for signal_events in events.signals {
            println!("  Signal {}: {} events",
                signal_events.signal_instance_id,
                signal_events.events.len()
            );
        }
    }
    SimulationUpdate::DesignHierarchy(hier) => {
        println!("Root modules: {}", hier.root_modules.len());
    }
    _ => {}
}
```

### 5.3. Working with Time

```rust
use hdl_simulation_protocol::time::{LogicalTime, PhysicalTime, Delta};

// Create logical time
let t1 = LogicalTime::new(PhysicalTime(1000), Delta(0));  // 1000fs, δ0
let t2 = LogicalTime::new(PhysicalTime(1000), Delta(1));  // 1000fs, δ1

assert!(t1 < t2);  // Same physical time, but t1 has earlier delta

// Add delta cycle
let t3 = t1 + Delta(5);  // (1000fs, δ5)

// Convert from physical time
let t4 = LogicalTime::from(PhysicalTime(2000));  // (2000fs, δ0)
```

## 6. Dependencies

Minimal dependencies for portability:

- `serde`: Serialization framework (with derive feature)
- `compact_str`: Memory-efficient strings for signal/module names

## 7. Design Rationale

### 7.1. Dense Signal IDs

Signal IDs use `NonZeroU32` starting from 1:

- Enables `Option<SignalInstanceId>` optimization (same size as `u32`)
- Dense allocation allows efficient Vec-based indexing
- Non-zero prevents accidental use of uninitialized IDs

### 7.2. LogicalTime Ordering

Logical time compares physical time first, then delta:

- Matches VHDL simulation semantics
- All events at (T, δN) occur before any event at (T+1, δ0)
- Enables efficient time-based queries

### 7.3. RawValue Encoding

64-bit raw values avoid type-specific variants:

- Reduces protocol message size
- Type information from design hierarchy
- Efficient for simulator (no encoding overhead)
- Flexible for future value types

### 7.4. Time in Femtoseconds

Using femtoseconds as base unit:

- Matches VHDL standard (time = fs precision)
- Covers entire practical simulation range
- Avoids floating-point precision issues
- Simple integer arithmetic

## 8. Limitations and Future Work

- **Array signals**: Full array support not yet implemented
- **Unsubscribe**: Signal untracking needs refinement
- **Value compression**: No compression for repeated values
- **Incremental hierarchy**: Design hierarchy sent in full (no diffs)
- **Time range**: Limited to u64 max (~584 years in fs)

## 9. License

MIT License - See LICENSE file for details.
