### Proposed Monitoring Enhancements

## Overview
The integration of libp2p gossip connections within our monitoring has been fraught with challenges.
The primary codebase has exhibited longstanding issues, making effective network communication and
diagnostics problematic. In light of these difficulties, we feel that we should address these issues
first within our current version, rather than deferring solutions to a future major update (2.0).
Transition to version 2.0 lacks clarity, particularly in terms of specification and even implementation language,
underscores the need for immediate action. We propose adopting either Rust or Go for developing 2.0,
given their respective advantages in compliance, performance, and developer accessibility.
Furthermore, integrating GeoDNS with monitoring and leveraging blockchain technology for certain
aspects of the monitoring process are key considerations. Enhancing data collection through a
browser extension, possibly a fork of `substrate-connect/smoldot` or use `subxt_lightclient`,
with a reward system for data contributions, could significantly enrich data required for
routing optimization.

**1. Direct Endpoint Monitoring Enhancement**

A glaring omission in our current setup is the direct monitoring of `*.ibp.network` and
`*.dotters.network` endpoints. Our lack of direct data collection on actual service endpoints
means that we are not measuring keymetric like service's total performance and availability.
Like what is the actual average latency for our users? What is the actual uptime of our service?
How do we currently determine if the we are doing the best possible routing in GeoDNS for the users?
Misconfigurations happen so we should improve monitoring to know if some country is not using
the closest endpoint.

- **Action:** Implement a dedicated monitoring task for these endpoints. This will not only provide
us with precise data regarding our service availability but also enable the collection of
location-based data from participants. Such data is invaluable for refining our routing strategies,
potentially leading to significant improvements in service performance and user experience. 

**2. Database and Configuration Streamlining**

The existing approach incorporates an unnecessary layer of complexity by adding extra level(+1) in
the code for migrating the database, which complicates the retrieval process of configuration levels.
This structure not only deviates from our principle of simplicity but also introduces potential
inaccuracies in our monitoring data.

- **Action:** Eliminate the extra increment code layer and adopt a linear configuration retrieval model.
The database should still be utilized for storing time series data. However, a significant improvement
can be achieved by either reconstructing the data using the git history of the configuration file—given
the current discrepancies—or by resetting the configuration levels to a standard level (e.g., level 5)
under a new model. After resetting levels, we can proceed with the database migration.

**3. Libp2p Library Update**

Our monitoring efficiency is further hampered by the use of an outdated version of the libp2p library.
From personal experience and debugging efforts, it's clear that the current version's limitations,
including a known bug, prevent the establishment of gossip connections, which are crucial for effective
network communication and monitoring.

- **Action:** Upgrade to the latest version of the libp2p library. This step is not merely a routine
update but a necessary measure to fix gossip for participants running monitoring. Transitioning to
the most recent library version will not only resolve the current issue but also place us in a better
position to leverage ongoing improvements and bug fixes, ensuring that our monitoring infrastructure
remains robust and effective.

**4. Status Page Overhaul for Endpoint Visibility**

Restructure the main status page to showcase collective endpoint availability, like ibp.network and
dotters.network, at the forefront. Relocate extensive P2P data to a secondary page for in-depth
troubleshooting. Main page centers on endpoint health; secondary page for member-specific, technical details.
Keep the primary interface clean and focused showing general status.

- [Gatus like collective status page](https://status.rotko.net/)

5. Automate the monitoring of the bootnodes with worker script and crate monitoring page for bootnodes.
Would replace the need of manual time consuming labor and add confidence for sdk developers to merge bootnodes.
- Bonus: CI into polkadot-sdk repo for checking bootnodes on every PR.
```
```

**5. Automation of Bootnode Monitoring**

Current practices for bootnode monitoring involve manual checks, which are time-consuming and prone to errors.
Automating this process would greatly enhance efficiency and reliability.

- **Proposed Action:** Implement a worker script to automate bootnode monitoring, accompanied by a dedicated
monitoring page. This automation would reduce manual labor and provide SDK developers with greater confidence
in integrating bootnodes.

- **Additional Consideration:** Integrate Continuous Integration (CI) into the polkadot-sdk repository to
validate bootnodes with every pull request, further ensuring reliability and performance.

These proposed enhancements aim to fortify our monitoring framework, making it more robust, efficient,
and user-friendly. By addressing these areas, we can significantly improve our network's performance,
reliability, and the overall experience for our users and developers.

- [js worker for bootnodes](https://github.com/ibp-network/ibp-monitor/blob/develop/worker/lib/f-check-bootnode.js)
- `./polkadot-parachain --no-hardware-benchmarks --no-mdns --chain ./config/chain-spec/people-westend.json
  --relay-chain-rpc-urls wss://rpc.ibp.network/westend --bootnodes "/dns/wcore16.rotko.net/tcp/35736/wss/12D3KooWFmGg7EGzxGDawuJ9EfyEznCrZfMJgGa4eHpMWjcJmg85"`
