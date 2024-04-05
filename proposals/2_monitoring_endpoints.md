### Proposed Monitoring Enhancements

**1. Direct Endpoint Monitoring Enhancement**

A glaring omission in our current setup is the direct monitoring of rpc.ibp.network and
rpc.dotters.network endpoints. Our lack of direct data collection on these fronts means we're missing
out on valuable insights into our service's performance and availability.

- **Action:** Implement a dedicated monitoring task for these endpoints. This will not only provide
us with precise data regarding our service availability but also enable the collection of
location-based data from participants. Such data is invaluable for refining our routing strategies,
potentially leading to significant improvements in service performance and user experience.

**2. Database and Configuration Streamlining**

The existing approach incorporates an unnecessary layer of complexity by adding extra level(+1) in
the code for migrating the database, which complicates the retrieval process of configuration levels.
This structure not only deviates from our principle of simplicity but also introduces potential
inaccuracies in our monitoring data.

- **Action:** Eliminate the additional code layer and adopt a linear configuration retrieval model.
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
dotters.network, at the forefront. Relocate extensive P2P data to a secondary, discrete page for in-depth
troubleshooting. Main page centers on endpoint health; secondary page for member-specific, technical details.
Keep the primary interface clean and focused.

- [Gatus like status page](https://status.rotko.net/)

