pub mod authzed {
    pub mod api {
        pub mod v1 {
            include!("gen/authzed.api.v1.rs");
        }
    }
}
pub mod google {
    pub mod rpc {
        include!("gen/google.api.rs");
        include!("gen/google.rpc.rs");
    }
}
pub mod validate {
    include!("gen/validate.rs");
}
