# From Path-Based Routing to Subdomain-Based Routing

## 1. Summary

This RFC details the limitations and challenges of using the current path-based routing
model (`rpc.ibp.network/$network`) in achieving effective redundancy and high availability
with HAProxy for blockchain RPC services. It outlines why a transition to a subdomain-based
routing model would overcome these limitations.

## 2. Motivation

The goal is to enhance our infrastructure's resilience and fault tolerance. The current
path-based routing model complicates the implementation of redundancy strategies within
HAProxy configurations, impacting service availability and scalability.

## 3. Background

Our blockchain RPC services are crucial for network operations and require high availability.
Currently, services are differentiated by URL paths. This model presents significant challenges
in configuring HAProxy for redundancy and backup services, a key strategy in maintaining uninterrupted service.

## 4. Limitations of Path-Based Routing

### 4.1 Complexity in Backup Configuration

- **HAProxy's Configuration Limitations:** HAProxy configures backups primarily at the backend level,
  without inherent support for path-based differentiation within the same backend. This makes configuring
  a path-based service as a backup for another cumbersome and less intuitive.

### 4.2 Inefficient Health Checks

- **Granular Health Checks:** Performing health checks on services differentiated by paths rather than
  domains or subdomains complicates monitoring. It requires custom configurations for each path, increasing
  complexity and potential for misconfiguration.

### 4.3 SSL/TLS Management Challenges

- **Certificate Management:** Using paths for service differentiation does not align well with SSL/TLS practices,
  where certificates are issued based on domain names. This misalignment complicates secure communication setup and maintenance.

### 4.4 Scalability and Flexibility Concerns

- **Scaling Services:** Path-based routing limits the flexibility in scaling services independently.
  It binds the scalability of services to the domain's scalability, potentially leading to bottlenecks.
- **Operational Rigidity:** Adjusting backup configurations or scaling services requires significant
  changes to HAProxy's setup, reducing operational agility.
- **Challenge to launch new networks:** Adding new network to GeoDNS is currently challenging with single subdomain
  for relay and system parachain services. All providers are required to have new deploys ready on the same day
  to avoid any downtime around the world. Having subdomains instead of paths for each network would allow to have
  full control for each network.

## 5. Proposal for Subdomain-Based Routing

Transitioning to a subdomain-based model (`$network.rpc.ibp.network`) addresses these limitations:

- **Simplified Configuration:** Each service on its subdomain allows for straightforward HAProxy backend
  configurations, making backup setups intuitive.
- **Effective Health Checks:** Subdomains enable more granular and manageable health checks at the
  DNS level, improving service monitoring and reliability.
- **Streamlined SSL/TLS Management:** Aligns with standard SSL/TLS practices, simplifying certificate
  management and enhancing security.
- **Enhanced Scalability and Flexibility:** Facilitates independent scaling of services and easy adjustment
  of backup configurations, improving infrastructure resilience.

## 6. Conclusion

The current path-based routing model introduces significant challenges in configuring HAProxy for effective
redundancy and high availability. This RFC advocates for a shift to a subdomain-based routing model, which
promises to streamline backup configurations, enhance security, and improve overall system scalability and
flexibility. Transition to subdomain-based routing would require improvements in automation of GeoDNS to avoid
causing too much burden for Gatotech and Stake.plus teams.

