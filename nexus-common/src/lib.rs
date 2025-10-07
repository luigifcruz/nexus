//! Nexus Common Library

// Include the generated proto code
pub mod proto {
    pub mod generic {
        tonic::include_proto!("generic");
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("descriptors_generic");
    }

    pub mod enums {
        tonic::include_proto!("enums");
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("descriptors_enums");
    }

    pub mod meta {
        tonic::include_proto!("meta");
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("descriptors_meta");
    }

    pub mod replicant {
        tonic::include_proto!("replicant");
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("descriptors_replicant");
    }

    pub mod instance {
        tonic::include_proto!("instance");
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("descriptors_instance");
    }

    pub mod nexus {
        tonic::include_proto!("nexus");
        pub const FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("descriptors_nexus");
    }
}
