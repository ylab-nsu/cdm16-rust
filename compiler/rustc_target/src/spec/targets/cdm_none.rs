use crate::spec::{
    PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions, LinkerFlavor
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "e-S16-p:16:16-i8:8-i16:16-i32:16-i64:16-f16:16-f32:16-f64:16-f128:16-m:C-n16".into(),
        llvm_target: "cdm-unknown-unknown".into(),
        metadata: TargetMetadata {
            description: Some("CdM-16".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 16,
        arch: "cdm".into(),

        options: TargetOptions {
            cpu: "cdm".into(),
            c_int_width: 16,
            max_atomic_width: Some(0),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            eh_frame_header: false,
            allow_asm: false,
            obj_is_bitcode: true,
            dynamic_linking: true,
            only_cdylib: true,
            linker_flavor: LinkerFlavor::Llbc,
            linker: Some("cdm-linker".into()),
            dll_suffix: ".o".into(),
            ..Default::default()
        },
    }
}
