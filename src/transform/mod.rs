// We need to transform the dog language structure from the analyzer into a structure that can be easily emitted to llvm
// You can think of this as the step that translates from dog concepts to llvm concepts.

// first pass -- add flags to existing structures to help transform correctly:
//  * figure out memory allocation: stack, heap, or static
//  * when we can identify variables accessed by multiple threads, maybe we make them atomic
//  *
