# Random TODOs
1. Create prelude and fix imports
3. Check for triggered abilities AFTER executing game actions; just keep the action objects around
7. Create gameactions for DrawCard and DrawCardSingular - and every similar action
8. fix folder structure: cards, context, controller, rules
9. Don't copy targets for stack objects. Implement a IdRef / TargetRef for holding arbitrary mutable refs (what about players?)
10. Support combining multiple StackDefinitions + Behaviours
11. User proc macro for nicer StackDefiniton enum generation
12. Add special casing to avoid single item enums (T,) in various List traits
14. Allow casting from non-hand zone
15. Handle casting of multi-type spells (the same as 10.)
16. Remove the general dumping ground that effect.rs is
17. Make type names more consistent
18. Remove useless Choice type; Use lifetimes to prevent reuse of Choices instead
19. Verifier that tracks valid/maybe-invalid ids at compile time
20. Make lands not use stack objects internatlly
21. Refact