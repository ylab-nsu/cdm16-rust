use crate::callconv::{ArgAbi, FnAbi};

fn classify_ret_ty<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() {
        ret.make_indirect();
    } else {
        ret.extend_integer_width_to(16);
    }
}

fn classify_arg_ty<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if arg.layout.is_aggregate() {
        arg.make_indirect();
    } else {
        arg.extend_integer_width_to(16);
    }
}

pub(crate) fn compute_abi_info<Ty>(fty: &mut FnAbi<'_, Ty>) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in fty.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }

        classify_arg_ty(arg);
    }
}
