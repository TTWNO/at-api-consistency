# Casual Issues Brought Up Day One

## Consistency Issues

- What happens if the application re-computes the entire semantic state? Think like a React App that re-draws from scratch upon a chnage of viewport orientation.
	- How does focus move?
	- Can you manitain focus?
	- What is the "state machine mechanics" to get back to the same state?
	- Or would this be "prohibited" from the protocol level? If React wants to do this, then a DOM diff would need to be calculated upon re-draw to push only the changes between the two views?

## Motheds of Communication

- What about shored memory?
	- Shored memory necessitates that the application keeps the entire accessibility tree in memory—outside of issues with memory safety across application boundaries, most applications have an internal represetation of their GUI and is not the same as what the API, and is essentially a projection from the application to the screen reader. This is much harder than having an entire tree ready for sharing.
- Synchronization
	- You can not haphazardly interleave IPC and update events.
	- If there are requests that fail for some reason, the reason must be stated. For example, if an element is now gone, the application must respond with an `OutOfDate(N)` where `N` is the number of "frames" the screen reader is behind.
		- The best way to do this would be if there were epoch counters on the vents, then requests can be tagged with the counter at the point of event; the response can be "the interface is now in front of the screen reader view)
			- Sub.Q: how you know which state effects casue errors in the response? Epoch counters are fine in theory, but we need a way to do counters that only go up during changes that effect those actions.

## Amazon + Android Issues

- Similar issue to the overal goal: IPC was taking most of the time when doing certain screen reader actions.
- TODO

## AT APIs as replication channels

Why are screen readers slow? They are slow because of IPC overhead and a lack of censistency model to allow caching.
Due to this, there is many seconds of latency in the P99 tail end.
This area is treated much more like an application rather than a system.
When treating the UI state as a distributed system, we can leverage database and CS research to reliably re-construct UI state on the screen reader side.

The goal of this project is to create a consistency model for AT APIs that allow these implovements into screen readers in the future.
