use crate::constpool::{
    ConstClassInfo, ConstModuleInfo, ConstPackageInfo, ConstPoolIndex, ConstUtf8Info,
};
use crate::vec::JvmVecU2;
use crate::{classfile_writable, classfile_writable_mask_flags};

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub MaskFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    ModuleFlags => {
        Open = 0x20,
        Synthetic = 0x1000,
        Mandated = 0x8000,
    }
}

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModuleRequires {
        requires: ConstPoolIndex<ConstModuleInfo>,
        flags: RequiresFlags,
        version_index: ConstPoolIndex<ConstUtf8Info>,
    }
);

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub RequiresFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    RequiresFlags => {
        Transitive = 0x20,
        StaticPhase = 0x40,
        Synthetic = 0x1000,
        Mandated = 0x8000,
    }
}

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModuleExports {
        exports: ConstPoolIndex<ConstPackageInfo>,
        flags: ExportsFlags,
        tos: JvmVecU2<ConstPoolIndex<ConstModuleInfo>>,
    }
);

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub ExportsFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    ExportsFlags => {
        Synthetic = 0x1000,
        Mandated = 0x8000,
    }
}

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModuleOpens {
        opens: ConstPoolIndex<ConstPackageInfo>,
        flags: OpensFlags,
        tos: JvmVecU2<ConstPoolIndex<ConstModuleInfo>>,
    }
);

classfile_writable_mask_flags! {
    #[derive(Eq, PartialEq, Debug)]
    pub OpensFlag as u16 = 0;
    #[derive(Eq, PartialEq, Debug)]
    OpensFlags => {
        Synthetic = 0x1000,
        Mandated = 0x8000,
    }
}

pub type ModuleUses = ConstPoolIndex<ConstClassInfo>;

classfile_writable!(
    #[derive(Eq, PartialEq, Debug)]
    pub struct ModuleProvides {
        provides: ConstPoolIndex<ConstClassInfo>,
        withs: JvmVecU2<ConstPoolIndex<ConstClassInfo>>,
    }
);
