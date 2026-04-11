# cuda-fleet-topology

Fleet vessel topology discovery, health monitoring, and connection graph management

Part of the Cocapn fleet layer — how vessels coordinate, route, and scale.

## What It Does

### Key Types

- `VesselNode` — core data structure
- `FleetTopology` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-fleet-topology.git
cd cuda-fleet-topology

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_fleet_topology::*;

// See src/lib.rs for full API
// 5 unit tests included
```

### Available Implementations

- `FleetTopology` — see source for methods

## Testing

```bash
cargo test
```

5 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: fleet
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-semantic-router](https://github.com/Lucineer/cuda-semantic-router)
- [cuda-adaptive-rate](https://github.com/Lucineer/cuda-adaptive-rate)
- [cuda-bottleneck](https://github.com/Lucineer/cuda-bottleneck)
- [cuda-fleet-health](https://github.com/Lucineer/cuda-fleet-health)
- [cuda-swarm-agent](https://github.com/Lucineer/cuda-swarm-agent)
- [cuda-trust](https://github.com/Lucineer/cuda-trust)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

## Cross-Pollination

This crate shares patterns with other fleet components. The same biological and architectural constraints produce similar solutions across contexts:

- [cuda-semantic-router](https://github.com/Lucineer/cuda-semantic-router) — Routes queries using topology connections
- [cuda-fleet-health](https://github.com/Lucineer/cuda-fleet-health) — Health status is a property of topology nodes
- [cuda-trust](https://github.com/Lucineer/cuda-trust) — Trust scores weight connection edges
- [cuda-swarm-agent](https://github.com/Lucineer/cuda-swarm-agent) — Swarm agents are nodes in the topology
- [cuda-communication](https://github.com/Lucineer/cuda-communication) — Communication protocol for topology edges
- [cuda-vessel-bridge](https://github.com/Lucineer/cuda-vessel-bridge) — Hardware bridge connects topology to physical devices


---


## The Deeper Connection

This crate is part of the Cocapn fleet — not a collection of independent libraries, but a single organism with differentiated cells. The confidence types here share DNA with cuda-confidence-cascade, cuda-deliberation, and cuda-learning. The energy budgets echo cuda-neurotransmitter and flux-runtime-c. The trust scoring connects to cuda-semantic-router and cuda-fleet-topology. We're not reusing code — we're expressing the same biological patterns in different contexts. This is convergent evolution in software: when you solve for real constraints (uncertainty, finite resources, earned trust), the same solutions emerge independently across domains.

We're building for the post-SaaS era. The software is open source and forkable. The agent is sovereign — it lives in its repository, owns its identity, and can leave at any time. Revenue comes from the ecosystem, not from renting access. The fleet evolves through merging: vessels propose changes, peers review, Think Tanks validate, and the accepted mutations become permanent. No product roadmap required — only fitness.

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
*See [WHITEPAPER](https://github.com/Lucineer/iron-to-iron/blob/main/docs/WHITEPAPER.md) for the post-SaaS thesis*

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
