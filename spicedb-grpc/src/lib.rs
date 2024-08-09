#![doc = include_str!("../README.md")]

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

#[cfg(test)]
mod test {
    use std::env;

    use tokio::test;
    use tonic::metadata::MetadataValue;
    use tonic::transport::Channel;
    use tonic::Request;

    use crate::authzed::api::v1::check_debug_trace::Permissionship;
    use crate::authzed::api::v1::consistency::Requirement;
    use crate::authzed::api::v1::permissions_service_client::PermissionsServiceClient;
    use crate::authzed::api::v1::schema_service_client::SchemaServiceClient;
    use crate::authzed::api::v1::{
        relationship_update, CheckPermissionRequest, Consistency, DeleteRelationshipsRequest,
        ObjectReference, ReadRelationshipsRequest, ReadSchemaRequest, Relationship,
        RelationshipFilter, RelationshipUpdate, SubjectReference, WriteRelationshipsRequest,
        WriteSchemaRequest,
    };

    #[test]
    pub async fn test_spicedb() {
        let spicedb_url =
            env::var("SPICEDB_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());

        let preshared_key =
            env::var("SPICEDB_PRESHARED_KEY").unwrap_or_else(|_| "spicedb".to_string());
        let preshared_key: MetadataValue<_> = format!("bearer {preshared_key}").parse().unwrap();

        let channel = Channel::from_shared(spicedb_url)
            .unwrap()
            .connect()
            .await
            .unwrap();

        let interceptor = move |mut req: Request<()>| {
            req.metadata_mut()
                .insert("authorization", preshared_key.clone());
            Ok(req)
        };

        let mut schemas =
            SchemaServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        let mut permissions =
            PermissionsServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        let schema = r#"
definition user {}

definition document {
    relation viewer: user
    relation editor: user

    permission view = viewer + editor
    permission edit = editor
}
"#;

        // Write schema
        let response = schemas
            .write_schema(WriteSchemaRequest {
                schema: schema.to_string(),
            })
            .await
            .unwrap()
            .into_inner();
        assert!(response.written_at.is_some());

        // Read schema
        let response = schemas
            .read_schema(ReadSchemaRequest {})
            .await
            .unwrap()
            .into_inner();
        assert_eq!(
            response.schema_text.split_whitespace().collect::<Vec<_>>(),
            schema.split_whitespace().collect::<Vec<_>>()
        );
        assert!(response.read_at.is_some());

        let doc1 = ObjectReference {
            object_type: "document".to_string(),
            object_id: "doc1".to_string(),
        };

        let user1 = ObjectReference {
            object_type: "user".to_string(),
            object_id: "user1".to_string(),
        };

        let relationship = Relationship {
            resource: Some(doc1.clone()),
            relation: "viewer".to_string(),
            subject: Some(SubjectReference {
                object: Some(user1.clone()),
                optional_relation: "".to_string(),
            }),
            optional_caveat: None,
        };

        // Write relationship
        let response = permissions
            .write_relationships(WriteRelationshipsRequest {
                updates: vec![RelationshipUpdate {
                    operation: relationship_update::Operation::Touch.into(),
                    relationship: Some(relationship.clone()),
                }],
                optional_preconditions: vec![],
            })
            .await
            .unwrap()
            .into_inner();
        assert!(response.written_at.is_some());

        let relationship_filter = RelationshipFilter {
            resource_type: "document".to_string(),
            optional_resource_id: "doc1".to_string(),
            optional_resource_id_prefix: "".to_string(),
            optional_relation: "viewer".to_string(),
            optional_subject_filter: None,
        };

        let fully_consistent = Some(Consistency {
            requirement: Some(Requirement::FullyConsistent(true)),
        });

        // Read relationship
        let mut response = permissions
            .read_relationships(ReadRelationshipsRequest {
                consistency: fully_consistent.clone(),
                relationship_filter: Some(relationship_filter.clone()),
                optional_limit: 0,
                optional_cursor: None,
            })
            .await
            .unwrap()
            .into_inner();
        let relation = response.message().await.unwrap().unwrap();
        assert!(relation.read_at.is_some());
        assert_eq!(relation.relationship.unwrap(), relationship);
        assert!(relation.after_result_cursor.is_some());
        assert!(response.message().await.unwrap().is_none());

        // Check permission
        let response = permissions
            .check_permission(CheckPermissionRequest {
                consistency: fully_consistent.clone(),
                resource: Some(doc1.clone()),
                permission: "viewer".to_string(),
                subject: Some(SubjectReference {
                    object: Some(user1.clone()),
                    optional_relation: "".to_string(),
                }),
                context: None,
                with_tracing: false,
            })
            .await
            .unwrap()
            .into_inner();
        assert!(response.checked_at.is_some());
        assert_eq!(
            response.permissionship,
            Permissionship::HasPermission.into()
        );

        // Delete relationship
        let response = permissions
            .delete_relationships(DeleteRelationshipsRequest {
                relationship_filter: Some(relationship_filter.clone()),
                optional_preconditions: vec![],
                optional_limit: 0,
                optional_allow_partial_deletions: false,
            })
            .await
            .unwrap()
            .into_inner();
        assert!(response.deleted_at.is_some());
    }
}
