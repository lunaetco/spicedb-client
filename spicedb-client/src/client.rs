use bytes::Bytes;
use spicedb_grpc::authzed::api::v1::{
    permissions_service_client::PermissionsServiceClient,
    schema_service_client::SchemaServiceClient, watch_service_client::WatchServiceClient, *,
};
use tonic::{
    metadata::{Ascii, MetadataValue},
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status, Streaming,
};

use crate::result::Result;

/// SpiceDB client
#[derive(Clone, Debug)]
pub struct SpicedbClient {
    pub channel: Channel,
    schemas: SchemaServiceClient<InterceptedService<Channel, SpicedbMiddleware>>,
    permissions: PermissionsServiceClient<InterceptedService<Channel, SpicedbMiddleware>>,
    watch: WatchServiceClient<InterceptedService<Channel, SpicedbMiddleware>>,
}

impl SpicedbClient {
    /// Create a new [`SpicedbClient`] from the server URL and a preshared key.
    ///
    /// ```rust
    /// # use spicedb_client::SpicedbClient;
    /// #
    /// # async fn create_client() {
    /// let mut client =
    ///     SpicedbClient::from_url_and_preshared_key("http://localhost:50051", "spicedb")
    ///         .await
    ///         .unwrap();
    /// # }
    /// ```
    pub async fn from_url_and_preshared_key(
        url: impl Into<Bytes>,
        preshared_key: impl ToString,
    ) -> Result<Self> {
        let interceptor = SpicedbMiddleware {
            preshared_key: Box::new(format!("bearer {}", preshared_key.to_string()).parse()?),
        };

        let channel = Channel::from_shared(url)?.connect().await?;

        let schemas = SchemaServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        let permissions =
            PermissionsServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        let watch = WatchServiceClient::with_interceptor(channel.clone(), interceptor.clone());

        Ok(SpicedbClient {
            channel,
            schemas,
            permissions,
            watch,
        })
    }

    /// Read the current Object Definitions for a Permissions System.
    ///
    /// Errors include:
    /// - INVALID_ARGUMENT: a provided value has failed to semantically validate
    /// - NOT_FOUND: no schema has been defined
    pub async fn read_schema(&mut self) -> Result<ReadSchemaResponse> {
        let response = self
            .schemas
            .read_schema(ReadSchemaRequest {})
            .await?
            .into_inner();

        Ok(response)
    }

    /// Overwrite the current Object Definitions for a Permissions System.
    pub async fn write_schema(&mut self, schema: impl ToString) -> Result<WriteSchemaResponse> {
        let response = self
            .schemas
            .write_schema(WriteSchemaRequest {
                schema: schema.to_string(),
            })
            .await?
            .into_inner();

        Ok(response)
    }

    /// Read a set of the relationships matching one or more filters.
    pub async fn read_relationships(
        &mut self,
        request: ReadRelationshipsRequest,
    ) -> Result<Streaming<ReadRelationshipsResponse>> {
        let stream = self
            .permissions
            .read_relationships(request)
            .await?
            .into_inner();

        Ok(stream)
    }

    /// Atomically write and/or delete a set of specified relationships. An
    /// optional set of preconditions can be provided that must be satisfied for
    /// the operation to commit.
    pub async fn write_relationships(
        &mut self,
        request: WriteRelationshipsRequest,
    ) -> Result<WriteRelationshipsResponse> {
        let response = self
            .permissions
            .write_relationships(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Atomically bulk delete all relationships matching the provided filter.
    /// If no relationships match, none will be deleted and the operation will
    /// succeed. An optional set of preconditions can be provided that must be
    /// satisfied for the operation to commit.
    pub async fn delete_relationships(
        &mut self,
        request: DeleteRelationshipsRequest,
    ) -> Result<DeleteRelationshipsResponse> {
        let response = self
            .permissions
            .delete_relationships(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Determine, for a given resource, whether a subject computes to having a
    /// permission or is a direct member of a particular relation.
    pub async fn check_permission(
        &mut self,
        request: CheckPermissionRequest,
    ) -> Result<CheckPermissionResponse> {
        let response = self
            .permissions
            .check_permission(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Evaluate the given list of permission checks.
    pub async fn check_bulk_permissions(
        &mut self,
        request: CheckBulkPermissionsRequest,
    ) -> Result<CheckBulkPermissionsResponse> {
        let response = self
            .permissions
            .check_bulk_permissions(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Reveal the graph structure for a resource's permission or relation. This
    /// RPC does not recurse infinitely deep and may require multiple calls to
    /// fully unnest a deeply nested graph.
    pub async fn expand_permission_tree(
        &mut self,
        request: ExpandPermissionTreeRequest,
    ) -> Result<ExpandPermissionTreeResponse> {
        let response = self
            .permissions
            .expand_permission_tree(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Return all the resources of a given type that a subject can access
    /// whether via a computed permission or relation membership.
    pub async fn lookup_resources(
        &mut self,
        request: LookupResourcesRequest,
    ) -> Result<Streaming<LookupResourcesResponse>> {
        let response = self
            .permissions
            .lookup_resources(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Return all the subjects of a given type that have access whether via a
    /// computed permission or relation membership.
    pub async fn lookup_subjects(
        &mut self,
        request: LookupSubjectsRequest,
    ) -> Result<Streaming<LookupSubjectsResponse>> {
        let response = self
            .permissions
            .lookup_subjects(request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Watch the database for mutations.
    ///
    /// SpiceDB's Watch API requires [commit timestamp tracking][postgres] be
    /// enabled for PostgreSQL and the [experimental changefeed][cockroachdb]
    /// for CockroachDB.
    ///
    /// [cockroachdb]:
    ///     https://authzed.com/docs/spicedb/concepts/datastores#memdb
    /// [postgres]:
    ///     https://authzed.com/docs/spicedb/concepts/datastores#postgresql
    pub async fn watch(&mut self, request: WatchRequest) -> Result<Streaming<WatchResponse>> {
        let response = self.watch.watch(request).await?.into_inner();

        Ok(response)
    }
}

#[derive(Clone)]
struct SpicedbMiddleware {
    preshared_key: Box<MetadataValue<Ascii>>,
}

impl Interceptor for SpicedbMiddleware {
    fn call(&mut self, mut request: Request<()>) -> Result<tonic::Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", (*self.preshared_key).clone());
        Ok(request)
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use tokio::test;

    use crate::reader::*;

    use super::*;

    #[test]
    pub async fn test_spicedb() {
        let spicedb_url =
            env::var("SPICEDB_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());

        let preshared_key =
            env::var("SPICEDB_PRESHARED_KEY").unwrap_or_else(|_| "spicedb".to_string());

        let mut client = SpicedbClient::from_url_and_preshared_key(spicedb_url, preshared_key)
            .await
            .unwrap();

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
        let response = client.write_schema(schema).await.unwrap();
        assert!(response.written_at().is_some());

        // Read schema
        let response = client.read_schema().await.unwrap();
        assert_eq!(
            response
                .schema_text()
                .split_whitespace()
                .collect::<Vec<_>>(),
            schema.split_whitespace().collect::<Vec<_>>()
        );
        assert!(response.read_at().is_some());
    }
}
