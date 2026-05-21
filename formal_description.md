# Formal Description of a Screan Reader as a Distributed System

Applicatinos (server) and screen readers (clients) communicate over accessibility APIs in oredr to relay semantic information to blind and visually impaired users of a computer (generally via TTS/sound).
When considering this architecture, the client and server are sharing some central state (the UI state)—and the AT API is the **replication channel** which is meant to reproduce the UI state within the screen reader for further processing.
In general, current AT APIs do _not_ assume a consistent model, and instead just run all algorithms and processing on a _partial_ view of the tree—and anytime new state is requested, there is a new RPC call, and any failure results in overall failure for the action.

## Problems With Partial Trees

### Structural Navigation

- Structural navigation is the process of moving focus to the next/previous element in a document with a particular role (a role in this case is a property like `Link`, `Button`, `Heading` [+level], `Navigation`, etc.).
- Current screen reader implementations rely on one (1) IPC call _per node traversal_ in order to find the next in-order element with the given role.
- There are at least two possible issues here.
		1. During the navigation process, with starting node `S` andcurrent node being checked `M`: a new element (`N`) is added between `S` and `M` with the correct role. Assuming we had a consistent model, should the screen reader focus on `N`, or the _next_ element? What if there is none?
		2. During the navigation process, with starting node `S` and current node being checked `M`: the element pointed to as "next" by `M` is deleted, causing a null read, with no progression possibilities. What should the screen reader do when its in-oredr node traversal is interrupted in this way? Keep track of the ancostors and move up until a valid node is once again found? Fail silently?

### Live/Atomic Regions

- `aria-live` regions indicate that text insertions should be spoken by the screen reader.
- `aria-atomic` regions indicate that text insertion/deletions shoulfd trigger a reading of the entire text area.
- These properties by default are not sent along with events about new nodes. Therefore a read IPC roundtrip is required to find this information.
- This leads to one major problem:
		1. The screen reader gets two events: one saying a node (`N`) has been created, followed by an event saying `N` has added the text "hello", followed by a final event deleting `N`. Here the screen reader, upon getting either event 1 or 2 will query whether it is inside a live/atomic region. By the time the application reponds, the element is already deleted.

