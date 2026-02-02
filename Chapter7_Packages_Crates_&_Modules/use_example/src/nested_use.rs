// --snip--
use std::{cmp::Ordering, io};
// --snip--
/*works like:
use std::cmp::Ordering;
use std::io;
*/

use std::io{self, Write}

/*works like:
use std::io
use std::io::Write;
*/

//can also do
use std::collections::*;
//wildcard "Glob" operator