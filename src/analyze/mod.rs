//semantic analyzer

// fleshes out details from parser/transformer so that it can be sent to optimizer

// in first pass:
// literal mapping to canonical form
//      * identify variable and return types
//      * move lambdas to anonymous functions
//      * establish casting of types
//
// access and scope checks
//
// add flags to existing structures to help transform correctly:
//  * figure out memory allocation: stack, heap, or static
//  * when we can identify variables accessed by multiple threads, maybe we make them atomic
//  *
