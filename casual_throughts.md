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


