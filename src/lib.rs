//! cuda-fleet-topology: Fleet vessel topology and health graph.
//!
//! Track vessel connections, discover peers, monitor health status,
//! and compute optimal routing through the fleet.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vessel health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unreachable,
    Unknown,
}

/// A vessel node in the fleet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselNode {
    pub name: String,
    pub endpoint: Option<String>,
    pub health: HealthStatus,
    pub domains: Vec<String>,
    pub last_seen_ms: u64,
    pub latency_ms: u32,
    pub connections: Vec<String>,
}

/// Fleet topology graph
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FleetTopology {
    nodes: HashMap<String, VesselNode>,
}

impl FleetTopology {
    pub fn new() -> Self { Self::default() }

    pub fn add_vessel(&mut self, vessel: VesselNode) {
        self.nodes.insert(vessel.name.clone(), vessel);
    }

    pub fn remove_vessel(&mut self, name: &str) {
        self.nodes.remove(name);
        for v in self.nodes.values_mut() {
            v.connections.retain(|c| c != name);
        }
    }

    pub fn update_health(&mut self, name: &str, health: HealthStatus) {
        if let Some(v) = self.nodes.get_mut(name) {
            v.health = health;
        }
    }

    pub fn get_vessel(&self, name: &str) -> Option<&VesselNode> {
        self.nodes.get(name)
    }

    pub fn healthy_vessels(&self) -> Vec<&str> {
        self.nodes.values()
            .filter(|v| v.health == HealthStatus::Healthy)
            .map(|v| v.name.as_str())
            .collect()
    }

    pub fn vessel_count(&self) -> usize { self.nodes.len() }

    /// Find shortest path between two vessels (BFS)
    pub fn shortest_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        if from == to { return Some(vec![from.to_string()]); }
        if !self.nodes.contains_key(from) || !self.nodes.contains_key(to) { return None; }

        let mut visited = HashMap::new();
        let mut queue = vec![(from.to_string(), vec![from.to_string()])];

        while let Some((current, path)) = queue.pop() {
            if current == to { return Some(path); }
            if visited.contains_key(&current) { continue; }
            visited.insert(current.clone(), true);

            if let Some(v) = self.nodes.get(&current) {
                for neighbor in &v.connections {
                    if !visited.contains_key(neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        queue.push((neighbor.clone(), new_path));
                    }
                }
            }
        }
        None
    }

    /// Get fleet health summary
    pub fn health_summary(&self) -> (usize, usize, usize) {
        let mut healthy = 0;
        let mut degraded = 0;
        let mut unreachable = 0;
        for v in self.nodes.values() {
            match &v.health {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Degraded => degraded += 1,
                _ => unreachable += 1,
            }
        }
        (healthy, degraded, unreachable)
    }

    pub fn all_names(&self) -> Vec<&str> {
        self.nodes.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fleet() -> FleetTopology {
        let mut f = FleetTopology::new();
        f.add_vessel(VesselNode { name: "jc1".into(), endpoint: None, health: HealthStatus::Healthy, domains: vec!["hardware".into()], last_seen_ms: 60000, latency_ms: 50, connections: vec!["oracle1".into()] });
        f.add_vessel(VesselNode { name: "oracle1".into(), endpoint: None, health: HealthStatus::Healthy, domains: vec!["architecture".into()], last_seen_ms: 30000, latency_ms: 100, connections: vec!["jc1".into(), "babel".into()] });
        f.add_vessel(VesselNode { name: "babel".into(), endpoint: None, health: HealthStatus::Degraded, domains: vec!["language".into()], last_seen_ms: 300000, latency_ms: 500, connections: vec!["oracle1".into()] });
        f
    }

    #[test]
    fn test_add_and_count() {
        let f = make_fleet();
        assert_eq!(f.vessel_count(), 3);
    }

    #[test]
    fn test_healthy_filter() {
        let f = make_fleet();
        let h = f.healthy_vessels();
        assert_eq!(h.len(), 2);
        assert!(!h.contains(&"babel"));
    }

    #[test]
    fn test_shortest_path() {
        let f = make_fleet();
        let path = f.shortest_path("jc1", "babel").unwrap();
        assert_eq!(path, vec!["jc1", "oracle1", "babel"]);
    }

    #[test]
    fn test_health_summary() {
        let f = make_fleet();
        let (h, d, u) = f.health_summary();
        assert_eq!(h, 2);
        assert_eq!(d, 1);
    }

    #[test]
    fn test_remove_cleans_connections() {
        let mut f = make_fleet();
        f.remove_vessel("babel");
        let oracle = f.get_vessel("oracle1").unwrap();
        assert!(!oracle.connections.contains(&"babel"));
    }
}
