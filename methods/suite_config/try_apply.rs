use freenet_stdlib::prelude::*;

pub fn try_apply<C: ContractInterface>(
    params: Parameters<'static>,
    state: State<'static>,
    datas: Vec<UpdateData<'static>>,
) -> Option<State<'static>> {
    let Ok(m) = C::update_state(params, state, datas) else {
        return None;
    };
    m.new_state
}

// no test_usage necessary
