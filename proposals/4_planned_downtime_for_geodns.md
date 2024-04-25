# Planned downtime in the GeoDNS service

Currently the GeoDNS service is being run by 2 members that are expected to
remove manually the participants for the service and thats just hell of a 
burden for them to remove and add the participants back manually.

For example being removed from the GeoDNS the traffic from philippines
moves to use following endpoints when removed that seems suboptimal.
```bash
PING sys.dotters.network (103.240.197.6) 56(84) bytes of data.
64 bytes from 103.240.197.6: icmp_seq=1 ttl=50 time=387 ms

PING sys.ibp.network (104.247.178.10) 56(84) bytes of data.
64 bytes from 104.247.178.10: icmp_seq=1 ttl=236 time=311 ms
```

I propose to have a api/onchain-remarks for each participant to be able
to set a planned downtime for the services that GeoDNS code follows and
takes actions based on.
