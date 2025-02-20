use sov_modules_api::Context;
use sov_modules_macros::ModuleInfo;
use sov_state::StateMap;

#[derive(ModuleInfo)]
struct TestStruct<C: Context> {
    #[state]
    test_state1: [usize; 22],

    #[state]
    test_state2: StateMap<u32, u32, C::Storage>,
}

fn main() {}
