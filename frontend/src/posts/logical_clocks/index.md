---
title: "Logical Clocks"
date: 2025-06-03
---
# Logical Clocks

Unlike the monotomic clock and the wall clock, logical clocks are typically fully
independent of the device clock. They are defined exclusvely in software, and
two independent logical clocks cannot be compared in any meaningful way. Logical
clocks are widely used in distributed systems to determine the relative ordering
of events, and are often used as Version Vectors.

## Happens Before Relationship

Before we take a look at logical clocks, we should take understand what it means
if an event happens before another. If you were to ask a random person on the
street what this means, you will likely get something like "The time that event
A happened at is earlier than the time that event B happened". If you read my
post on clock synchronization, you'd be familiar with the idea that the concept
of time is not uniform across machines. Therefore, when discussing distributed
systems, the happens before relationship works a little bit different.  

When event A happens before event B (i.e $A -> B$), one of the following is
true:

1. Events A and B both happen on node X, and A happens before B
2. Event A causes event B to happen
3. If event A happens before event B, event B before C, then $A -> C$


## Lamport Clock

The Lamport clock is a logical clock that allows you to get a *partial ordering*
of events in your system. This partial ordering tells us that particular events
happened before others. A partial ordering does not guarantee that any two elements
are comparible, only that if events are comparible, that there is an ordering
between the two events.

The algorithm is roughly as follows:

1. Any time an event is discovered (message received, state changed, etc), you
   increment your own logical clock.
2. If you are sending knowledge of this event to another node, include the value
   of your own logical clock. 
3. Any time a message is received, you set your clock to the maximum of
   (your_time, message_time)

In rust, the algorithm looks something like this:

```rust
use std::cmp::max;

#[derive(Clone, Debug)]
struct LamportClock {
    pub time: i64,
}

impl Clock for LamportClock {
    /// Whenever an event occurs on this node (message received, state changed, etc), we
    /// immediately increment our clock to show the passing of time
    fn advance_clock(&mut self) -> i64 {
        self.time += 1;
        self.time
    }

    /// Whenever we receive a message from another node, we set our clock to the maximum of our own
    /// clock and the message timestamp. This ensures time always moves forward, and is what allows
    /// us to define partial ordering for particular events.
    fn update_clock(&mut self, message_timestamp: i64) -> i64 {
        self.time = max(self.time, message_timestamp);
        self.advance_clock()
    }
}

impl LamportClock {
    pub fn new() -> Self {
        Self { time: 0 }
    }
}
```

<figure class="mermaid-diagram">
  <pre class="mermaid">
sequenceDiagram
    participant A
    participant B
    participant C
    autonumber
    A->>A: t = 0: Garbage collection
    C->>C: t = 1: Garbage collection
    A->>B: t = 2: M1
    B->>C: t = 4: M2
  </pre>
  <figcaption>Figure 1: Event Sequence Diagram</figcaption>
</figure>


Lamport clock have several interesting properties. First, they provide 

events *A* and *B*, with lamport timestamps $L_A$ and $L_B$ respecfully, we have
the following:

1. If $A -> B$ , then $L_A < L_B$
2. If $L_A < L_B$ then either $A -> B$ or $A || B$
3. Lamport clocks can provide a total ordering if there is a way to break ties

Let's explore these properties and try to understand their consequences. First,
(1) states that if event $E_1$ happened before event $E_2$, then the lamport timestamp
for event $E_1$ with be less than that of $E_2$. In figure 1, the sending of
message $M_1$ on node A must have a lower timestamp than that of the receival of
$M_1$ on node B because you cannot receive a message that was not sent.  


(2) states that given two lamport timestamps $L_A$ and $L_B$, if $L_A < L_B$,
then either A happened before B, or A and B are *concurrent*. Concurrent in this
case means that we don't actually know which of the two events happened first,
and also that the events are independent. Events $A_1$ and $B_1$ in figure 2 are
concurrent since there is no happens before relationship between them.
Concurrent events are the reason why we have a partial ordering rather than a
total ordering. Some events such as those mentioned above do not have an
ordering between them according to lamport timetsamps, which brings us to (3).  

(3) states that assuming we have a way to break ties between events with the
same timestamp, we get a *total ordering* of the events in our system. A total
ordering is an order of events in the system that all nodes agree on. A total
ordering you are likely familiar with is the lexographical ordering of words
composed of alphabetical letters commonly found in dictionary references. We
know that the word bat should show up before cat, and cat before zebra. Indeed,
with any two words in the English language, you can tell me which one shows up
first in a dictionary. Lamport timestamps can provide us a total ordering
assuming we have a way to break ties of events with the same timestamp. One
common way to do this is to include a node identifier in the event, which would
be compared lexographically if there was a tie. In figure 1, events $A_1$ and
$C_1$ can be ordered if the node ID is attached to the event, in which case
$A_1 < C_1$ because A comes before C in the alphabet. If all of the nodes were
threads on the same system, you could instead use the process ID (PID). If all
events are generated in a single thread context on the same node, then no extra
information is needed for a total ordering because you can never generate two
events with the same timestamp, and all timestamps can be compared by default.



## Hybrid Clocks
## Vector Clocks
## Chain clocks
## Bloom Clocks






