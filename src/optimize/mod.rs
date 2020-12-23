// performs optimization for none, size, or speed (default to speed)
// llvm may do some optimization for us, need to verify, I never got around to writing the optimizer for smirk, so I'm not sure.

// none: do nothing
// size or speed: remove unreachable code (private methods that aren't called), code that has no impact (unused variable assignments or if statements against constants that can't ever be true), and reuse constants
// speed: inline when possible, loop optimizations (unwind small loops, loop inversion, invariant code movement, interchange, splitting, un-switching, etc)
// size: replace redundant code in functions

// functions that simply call another function or access a single variable like a proxy can sometimes be removed as a hop

// private functions that are never called can be removed entirely
// public functions that are not used and not part of a library can be removed entirely

// structures members that are not used in structures that aren't passed to any external library
// might be removable... this is tricky because it changes structure size and memory alignment...
// not sure on this one.

// NOTE
// I'm not sure the name of the performance optimization, but there's one like this with a complexity of O(N^2):
//
// for each member of X:
//   for each member of Y:
//     do computation with data from Y and use it with data from X
//
// This can become a complexity of O(N) with more memory used for lookup
//
// for each member of Y:
//   do partial computation with data and store it in one or more maps
// for each member of X:
//   do computation from lookup maps derived from Y and data from X
//
// This only works if the final computation is one-to-one with X and if a partial computation can be achieved using Y's members.
// Since map lookups are not free, this works best for large numbers of members in X and Y and if the computation is expensive or involves IO.


// another optimization: scope blocks that don't allocate locals don't need to exist, move code one scope level up