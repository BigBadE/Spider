# Design

Spider is designed to simulate every unique thread order of a program. Unique is defined as an order which leads 
to a different outcome, since task ordering is O(n!), Spider is designed to detect which orders have an impact on how the program runs.

This is done by replacing all async Tokio types with a wrapper through Spider. Without the Spider feature, these are just 
aliases to the Tokio type. In Spider mode, it instead starts running the test over and over in dependency detection mode.

## Dependency detection mode

A program's tasks can be represented by a graph of sets. Each task is broken up by its Spider awaits 
(since those are the only ones with inter-task dependencies), where each set is made up of each task which doesn't depend on each other
to run (spawning a task is also considered a Spider await, and ends the task).