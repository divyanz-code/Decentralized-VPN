#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{Address, Env, String, Symbol, contract, contractimpl, contracttype, log, symbol_short};

// Structure to track VPN node information
#[contracttype]
#[derive(Clone)]
pub struct VPNNode {
    pub node_id: u64,
    pub operator: Address,
    pub bandwidth_provided: u64, // in GB
    pub tokens_earned: u64,
    pub is_active: bool,
    pub registration_time: u64,
}

// Structure to track overall network statistics
#[contracttype]
#[derive(Clone)]
pub struct NetworkStats {
    pub total_nodes: u64,
    pub active_nodes: u64,
    pub total_bandwidth: u64,
    pub total_tokens_distributed: u64,
}

// Storage keys
const NETWORK_STATS: Symbol = symbol_short!("NET_STATS");
const NODE_COUNT: Symbol = symbol_short!("NODE_CNT");

// Mapping node_id to VPNNode
#[contracttype]
pub enum NodeBook {
    Node(u64)
}

#[contract]
pub struct DecentralizedVPNContract;

#[contractimpl]
impl DecentralizedVPNContract {
    
    // Function 1: Register a new VPN node operator
    pub fn register_node(env: Env, operator: Address) -> u64 {
        operator.require_auth();
        
        let mut node_count: u64 = env.storage().instance().get(&NODE_COUNT).unwrap_or(0);
        node_count += 1;
        
        let time = env.ledger().timestamp();
        
        // Create new VPN node
        let new_node = VPNNode {
            node_id: node_count,
            operator: operator.clone(),
            bandwidth_provided: 0,
            tokens_earned: 0,
            is_active: true,
            registration_time: time,
        };
        
        // Update network statistics
        let mut stats = Self::get_network_stats(env.clone());
        stats.total_nodes += 1;
        stats.active_nodes += 1;
        
        // Store the node and updated stats
        env.storage().instance().set(&NodeBook::Node(node_count), &new_node);
        env.storage().instance().set(&NETWORK_STATS, &stats);
        env.storage().instance().set(&NODE_COUNT, &node_count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "VPN Node registered with ID: {}", node_count);
        node_count
    }
    
    // Function 2: Report bandwidth and reward node operator
    pub fn report_bandwidth(env: Env, node_id: u64, bandwidth_gb: u64) {
        let mut node = Self::get_node_info(env.clone(), node_id);
        
        if !node.is_active || node.node_id == 0 {
            log!(&env, "Node not found or inactive");
            panic!("Node not found or inactive");
        }
        
        // Calculate tokens: 10 tokens per GB
        let tokens_to_reward = bandwidth_gb * 10;
        
        // Update node data
        node.bandwidth_provided += bandwidth_gb;
        node.tokens_earned += tokens_to_reward;
        
        // Update network statistics
        let mut stats = Self::get_network_stats(env.clone());
        stats.total_bandwidth += bandwidth_gb;
        stats.total_tokens_distributed += tokens_to_reward;
        
        // Store updated data
        env.storage().instance().set(&NodeBook::Node(node_id), &node);
        env.storage().instance().set(&NETWORK_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Node {} rewarded {} tokens for {} GB", node_id, tokens_to_reward, bandwidth_gb);
    }
    
    // Function 3: Deactivate a VPN node
    pub fn deactivate_node(env: Env, node_id: u64, operator: Address) {
        operator.require_auth();
        
        let mut node = Self::get_node_info(env.clone(), node_id);
        
        if node.node_id == 0 {
            log!(&env, "Node not found");
            panic!("Node not found");
        }
        
        if node.operator != operator {
            log!(&env, "Unauthorized: Not the node operator");
            panic!("Unauthorized");
        }
        
        if !node.is_active {
            log!(&env, "Node already inactive");
            panic!("Node already inactive");
        }
        
        // Deactivate node
        node.is_active = false;
        
        // Update network statistics
        let mut stats = Self::get_network_stats(env.clone());
        stats.active_nodes -= 1;
        
        // Store updated data
        env.storage().instance().set(&NodeBook::Node(node_id), &node);
        env.storage().instance().set(&NETWORK_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Node {} deactivated", node_id);
    }
    
    // Function 4: View node information
    pub fn get_node_info(env: Env, node_id: u64) -> VPNNode {
        let key = NodeBook::Node(node_id);
        
        env.storage().instance().get(&key).unwrap_or(VPNNode {
            node_id: 0,
            operator: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            bandwidth_provided: 0,
            tokens_earned: 0,
            is_active: false,
            registration_time: 0,
        })
    }
    
    // View network statistics
    pub fn get_network_stats(env: Env) -> NetworkStats {
        env.storage().instance().get(&NETWORK_STATS).unwrap_or(NetworkStats {
            total_nodes: 0,
            active_nodes: 0,
            total_bandwidth: 0,
            total_tokens_distributed: 0,
        })
    }
}

#[cfg(test)]
mod test;