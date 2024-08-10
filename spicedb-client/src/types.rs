use prost_types::value::Kind;
use spicedb_grpc::authzed::api::v1::consistency::Requirement;
pub use spicedb_grpc::authzed::api::v1::consistency::Requirement::*;
pub use spicedb_grpc::authzed::api::v1::relationship_update::Operation::*;
use spicedb_grpc::authzed::api::v1::*;

pub type ContextValue = Kind;
pub type ConsistencyRequirement = Requirement;
pub type RelationshipUpdateOperation = relationship_update::Operation;
pub type PreconditionOperation = precondition::Operation;
