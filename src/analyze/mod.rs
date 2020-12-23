//semantic analyzer

// takes output from parser and translates it to an internal, execution representation
// this representation will be sent to the optimizer

// in first pass:
// literal mapping to canonical form
//      * identify variable and return types
//      * move lambdas to anonymous functions
//      * establish casting of types
// access and scope checks

