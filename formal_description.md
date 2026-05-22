# Formal Description of a Screan Reader as a Distributed System

Applicatinos (server) and screen readers (clients) communicate over accessibility APIs in oredr to relay semantic information to blind and visually impaired users of a computer (generally via TTS/sound).
When considering this architecture, the client and server are sharing some central state (the UI state)—and the AT API is the **replication channel** which is meant to reproduce the UI state within the screen reader for further processing.
In general, current AT APIs do _not_ assume a consistent model, and instead just run all algorithms and processing on a _partial_ view of the tree—and anytime new state is requested, there is a new RPC call, and any failure results in overall failure for the action.

## Problems With Partial Trees

### Structural Navigation

- Structural navigation is the process of moving focus to the next/previous element in a document with a particular role (a role in this case is a property like `Link`, `Button`, `Heading` [+level], `Navigation`, etc.).
- Current screen reader implementations rely on one (1) IPC call _per node traversal_ in order to find the next in-order element with the given role.
- There are at least two possible issues here.
		1. During the navigation process, with starting node `S` andcurrent node being checked `M`: a new element (`N`) is added between `S` and `M` with the correct role. Assuming we had a consistent model, should the screen reader focus on `N`, or the _next_ element? What if there is a dangling reference (due to a stale view)?
		2. During the navigation process, with starting node `S` and current node being checked `M`: the element pointed to as "next" by `M` is deleted, causing a null read, with no progression possibilities. What should the screen reader do when its in-oredr node traversal is interrupted in this way? Keep track of the ancostors and move up until a valid node is once again found? Fail silently?

### Live/Atomic Regions

- `aria-live` regions indicate that text insertions should be spoken by the screen reader.
- `aria-atomic` regions indicate that text insertion/deletions shoulfd trigger a reading of the entire text area.
- These properties by default are not sent along with events about new nodes. Therefore a read IPC roundtrip is required to find this information.
- This leads to one major problem:
		1. The screen reader gets two events: one saying a node (`N`) has been created, followed by an event saying `N` has added the text "hello", followed by a final event deleting `N`. Here the screen reader, upon getting either event 1 or 2 will query whether it is inside a live/atomic region. By the time the application reponds, the element is already deleted.

### Atomicity and Ordering

- Some events which belong together, for example, adding a child to a node, are split into multiple events that have only a weak relationship. Example:
	- `AddNode: M, with attribuites X, Y, Z, etc`
	- `AddChild: parent N, child M`
- These two events are not guarenteed to come in a determined order, meaning a cache would either have to lazily load the fields on the node `M` (in case of a reversing of the events) or would need to have free floating nodes with connections drawn "at some time in the future".
- Both of these situations casue an inconsistent view of the UI state from the screen reader perspective.
- Similarly, when deleting, many APIs define separate events:
	- `RemoveChildAtIndex(parent, n)`
	- `RemoveItem(child)`
- These are also atomic events that cause an inconsistent view between the processing stages (especially in an undefined order).
- Events which "go together" (i.e., are one "unit of change") must come as a package deal: adding a node with a given parent/child relationship, attributes, etc. must come all together instead of disparate events with undefined ordering.
	- Basiccally, AT APIs should be atomic.

#### Test Cases & Consistency Model

This property can be tested with specific test cases—specifically, is the view the screen reader has a directed asyclic graph (connected tree) at all times (i.e., between each event).
Fuzz testing with the following tree/event generation could generate concrete failure modes:

```rust
// assume the `Tree::random_generate()` method creates a valid, random tree.
let mut tree = Tree::random_generate();
// This will randomly generate an event
let event = Event::random_generate();
assert!(tree.is_connected_dag());
tree.apply(event);
assart!(tree.is_connected_dag());
```

More formally, for any tree state `S` (where `S` is a connected DAG) and modification `M`, applying modification `M` to `S` will always result in the same typed output.

This can also be said to be the consistency model.

### Sparse Interfaces

A non-consistency issue with these APIs is the lack of support for sparse interfaces.
For spreadsheet applications, the entire spreadsheet (generally thousands of columns by millions of rows) should be exposed via the AT API, along with the content in the various cells.
Sparse interfaces require overlay representations similar in spirit to sparse matrices: define an N by M table with individual cells defined one-by-one.

## Solutions

For each problem faced in the previous section, we will describe what a consistent, atomic, isolated API would provide compared to the original AT-SPI model.

### `aria-live` and `aria-atomic` regions

Instead of the following information flow:

1. Application sends a `NewNode` event with partial information.
2. Application sends a `TextInserted` event with the next text.
3. The screen reader asks for the `aria-*` properties from the application.
4. ??? TODO

Instead, the following flow is suggested:

1. Application sends a `NewNode` event _which describes all `aria-*` attributes in one event_.
2. The application sends a text changed event.
3. The screen reader is able to respond by generating synthetic speech, without an IPC call back to the application.

This will be refered to in the future as the "push-based" protocol.
Push all known attributes to the screen reader in advance of it being asked for.
This allows the screen reader to have an _up to date_ copy of the UI state at any given point.

Formally, this is the description of a **replication channel**.
The screen reader can then act upon its known state at a point in time.

### Parent/Child Relationships & New Nodes

The separation of new nodes from the events which position them in the tree cause a temporary inconsistency in the tree structure (making it either not a tree, or an incomplete tree).
These events will be coalesced into single, atomic events preserving the structure of the original tree before the mutation.

So now, new node events _will also include its index as a child_ in order to place it within the tree structure.
Instead of:

1. `NewNode(parent, ...)`
2. `NewCHild(parent, child, index)`

Now the events are sent in a single event:

1. `NewNode(parent, index, ...)

No child mutation events need to be sent.
The reciprocal events for deletions would be implemented in the same way.
In the case of children removal, the additional fields are net needed, but inserting without all fields specified would be prohibited.

### Incramental Updates

Additionally specified in the API is an ordering for all updates.
For example, if an entire subtree needs to be added to the view, it must be done in order:

1. The root node (with no children specified). This is a complete tree.
2. The children of the root node, including their indicies. This creates a tree with exactly two levels, the root and its children.
3. Continue recursively down the tree.

If incremental updates become a performance bottleneck, consider tree diff events.
Each child can become its own thread of computation, since all appends will be applied further down the hirearchy.

Of note: the events do not need to be separated by new "frames" of data so to speak; they can be sent in one large change list.

### Screen Reader Usage

The screen reader can now use this data to respond to events in the order that are recieved with a _perfect view at that point in time_. 

- To use the `aria-live`/`aria-atomic` example, the node is created with its `aria-\*` attributes exposed.
- Then, the text changed evenvt comes in.
- The screen reader now has the `aria-\*` attribute list, and acts on the event with a perfect copy of the tree at the time of the text changed event. It can speak the inserted text, the entire text buffer, or nothing based on the attribute mix.
- When the next event is the deletion, the element is now removed without conflicting with the `aria-\*` attributes.


