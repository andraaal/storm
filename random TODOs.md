# Random TODOs
1. Create prelude and fix imports
3. Check for triggered abilities AFTER executing game actions; just keep the action objects around
7. Create gameactions for DrawCard and DrawCardSingular - and every similar action
8. fix folder structure: cards, context, controller, rules
9. Don't copy targets for stack objects. Implement a IdRef / TargetRef for holding arbitrary mutable refs (what about players?)
10. Support combining multiple StackDefinitions + Behaviours
11. User proc macro for nicer StackDefiniton enum generation (Or #25?)
12. Add special casing to avoid single item enums (T,) in various List traits
14. Allow casting from non-hand zone
15. Handle casting of multi-type spells (the same as 10.)
16. Remove the general dumping ground that effect.rs is
17. Make type names more consistent
18. Remove useless Choice type; Use lifetimes to prevent reuse of Choices instead
19. Verifier that tracks valid/maybe-invalid ids at compile time
20. Make lands not use stack objects internatlly
21. Milling out should be a state-based action
22. Make losing/winning game actions
23. Make all state-based game-actions simultaneous
24. Implement cancellable for X and Modes
25. Turn the macros in Cards into Blanket implementations
26. Collect activated abilities in the game to allow for more than one activated ability
27. Use GameActions for casting
28. Redo the Cost implementation
29. Move end condition upwards in Continuous ability
