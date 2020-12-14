pub mod spu;
pub mod topic;
pub mod partition;
pub mod spg;
pub mod message;

pub use fluvio_stream_model::core;

pub mod store {
    pub use fluvio_stream_model::store::*;
}

#[cfg(feature = "k8")]
pub use fluvio_stream_model::k8;

pub mod extended {

    use super::core::Spec;

    #[derive(Debug, Clone, PartialEq, Hash, Eq)]
    #[cfg_attr(feature = "use_serde", derive(serde::Serialize, serde::Deserialize))]
    pub enum ObjectType {
        Spu,
        CustomSpu,
        SpuGroup,
        Topic,
        Partition,
    }

    pub trait SpecExt: Spec {
        const OBJECT_TYPE: ObjectType;
    }
}
