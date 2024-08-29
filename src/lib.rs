/*!

# libmadeline

Reimplements Celeste's open-source Player.cs as a DLL file you can put in anything.

*/

mod math;
pub use math::*;

mod input;
pub use input::*;

mod maddy;
pub use maddy::*;

mod sys;
pub use sys::*;