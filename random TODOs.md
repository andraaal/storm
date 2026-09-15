# Random TODOs
1. Create prelude and fix imports
3. Check for triggered abilities AFTER executing game actions; just keep the action objects around
7. Create gameactions for DrawCard and DrawCardSingular
8. fix folder structure: cards, context, controller, rules
9. Don't copy targets for stack objects. Implement a IdRef / TargetRef for holding arbitrary mutable refs (what about players?)
10. Support combining multiple StackDefinitions
11. User proc macro for nicer StackDefiniton enum generation
12. Add special casing to avoid single item enums (T,) in Selector + its Output
13. Create specialized Damage Target
