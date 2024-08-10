use prost_types::Value;
use spicedb_grpc::authzed::api::v1::{subject_filter::RelationFilter, *};

use crate::types::{
    ConsistencyRequirement, ContextValue, PreconditionOperation, RelationshipUpdateOperation,
};

pub trait ReadRelationshipsRequestBuilder {
    fn new() -> Self;

    fn relationship_filter(&mut self) -> &mut RelationshipFilter;

    fn clear_relationship_filter(&mut self) -> &mut Self;

    fn consistency(&mut self, requirement: ConsistencyRequirement) -> &mut Self;

    fn clear_consistency(&mut self) -> &mut Self;

    fn limit(&mut self, limit: u32) -> &mut Self;

    fn clear_limit(&mut self) -> &mut Self;

    fn cursor(&mut self, token: impl ToString) -> &mut Self;

    fn clear_cursor(&mut self) -> &mut Self;
}

impl ReadRelationshipsRequestBuilder for ReadRelationshipsRequest {
    fn new() -> Self {
        Default::default()
    }

    fn relationship_filter(&mut self) -> &mut RelationshipFilter {
        self.relationship_filter
            .get_or_insert_with(Default::default)
    }

    fn clear_relationship_filter(&mut self) -> &mut Self {
        self.relationship_filter = None;
        self
    }

    fn consistency(&mut self, requirement: ConsistencyRequirement) -> &mut Self {
        self.consistency = Some(Consistency {
            requirement: Some(requirement),
        });
        self
    }

    fn clear_consistency(&mut self) -> &mut Self {
        self.consistency = None;
        self
    }

    fn limit(&mut self, limit: u32) -> &mut Self {
        self.optional_limit = limit;
        self
    }

    fn clear_limit(&mut self) -> &mut Self {
        self.optional_limit = 0;
        self
    }

    fn cursor(&mut self, token: impl ToString) -> &mut Self {
        self.optional_cursor = Some(Cursor {
            token: token.to_string(),
        });
        self
    }

    fn clear_cursor(&mut self) -> &mut Self {
        self.optional_cursor = None;
        self
    }
}

impl RelationshipFilterBuilder for ReadRelationshipsRequest {
    fn new() -> Self {
        ReadRelationshipsRequestBuilder::new()
    }

    fn resource_type(&mut self, resource_type: impl ToString) -> &mut Self {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .resource_type = resource_type.to_string();
        self
    }

    fn clear_resource_type(&mut self) -> &mut Self {
        if let Some(filter) = self.relationship_filter.as_mut() {
            filter.resource_type.clear();
        }
        self
    }

    fn resource_id(&mut self, resource_id: impl ToString) -> &mut Self {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .optional_resource_id = resource_id.to_string();
        self
    }

    fn clear_resource_id(&mut self) -> &mut Self {
        if let Some(filter) = self.relationship_filter.as_mut() {
            filter.optional_resource_id.clear();
        }
        self
    }

    fn resource_id_prefix(&mut self, resource_id_prefix: impl ToString) -> &mut Self {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .optional_resource_id_prefix = resource_id_prefix.to_string();
        self
    }

    fn clear_resource_id_prefix(&mut self) -> &mut Self {
        if let Some(filter) = self.relationship_filter.as_mut() {
            filter.optional_resource_id_prefix.clear();
        }
        self
    }

    fn relation(&mut self, relation: impl ToString) -> &mut Self {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .optional_relation = relation.to_string();
        self
    }

    fn clear_relation(&mut self) -> &mut Self {
        if let Some(filter) = self.relationship_filter.as_mut() {
            filter.optional_relation.clear();
        }
        self
    }

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut SubjectFilter {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .subject_type(subject_type)
    }

    fn subject_filter(&mut self) -> Option<&mut SubjectFilter> {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .subject_filter()
    }

    fn clear_subject_filter(&mut self) -> &mut Self {
        self.relationship_filter
            .get_or_insert_with(Default::default)
            .clear_subject_filter();
        self
    }
}

pub trait RelationshipFilterBuilder {
    fn new() -> Self;

    fn resource_type(&mut self, resource_type: impl ToString) -> &mut Self;

    fn clear_resource_type(&mut self) -> &mut Self;

    /// Set the resource ID.
    ///
    /// This method clears `resource_id_prefix`.
    fn resource_id(&mut self, resource_id: impl ToString) -> &mut Self;

    fn clear_resource_id(&mut self) -> &mut Self;

    /// Set the resource ID prefix.
    ///
    /// This method clears `resource_id`.
    fn resource_id_prefix(&mut self, resource_id_prefix: impl ToString) -> &mut Self;

    fn clear_resource_id_prefix(&mut self) -> &mut Self;

    fn relation(&mut self, relation: impl ToString) -> &mut Self;

    fn clear_relation(&mut self) -> &mut Self;

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut SubjectFilter;

    fn subject_filter(&mut self) -> Option<&mut SubjectFilter>;

    fn clear_subject_filter(&mut self) -> &mut Self;
}

impl RelationshipFilterBuilder for RelationshipFilter {
    fn new() -> Self {
        Default::default()
    }

    fn resource_type(&mut self, resource_type: impl ToString) -> &mut Self {
        self.resource_type = resource_type.to_string();
        self
    }

    fn clear_resource_type(&mut self) -> &mut Self {
        self.resource_type.clear();
        self
    }

    fn resource_id(&mut self, resource_id: impl ToString) -> &mut Self {
        self.optional_resource_id = resource_id.to_string();
        self.clear_resource_id_prefix();
        self
    }

    fn clear_resource_id(&mut self) -> &mut Self {
        self.optional_resource_id.clear();
        self
    }

    fn resource_id_prefix(&mut self, resource_id_prefix: impl ToString) -> &mut Self {
        self.clear_resource_id();
        self.optional_resource_id_prefix = resource_id_prefix.to_string();
        self
    }

    fn clear_resource_id_prefix(&mut self) -> &mut Self {
        self.optional_resource_id_prefix.clear();
        self
    }

    fn relation(&mut self, relation: impl ToString) -> &mut Self {
        self.optional_relation = relation.to_string();
        self
    }

    fn clear_relation(&mut self) -> &mut Self {
        self.optional_relation.clear();
        self
    }

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut SubjectFilter {
        let filter = self
            .optional_subject_filter
            .get_or_insert_with(Default::default);
        filter.subject_type = subject_type.to_string();
        filter
    }

    fn subject_filter(&mut self) -> Option<&mut SubjectFilter> {
        self.optional_subject_filter.as_mut()
    }

    fn clear_subject_filter(&mut self) -> &mut Self {
        self.optional_subject_filter = None;
        self
    }
}

/// Specify a filter on the subject of a relationship.
///
/// The subject tipe is required and all other fields are optional.
pub trait SubjectFilterBuilder {
    fn new(subject_type: impl ToString) -> Self;

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut Self;

    fn subject_id(&mut self, subject_id: impl ToString) -> &mut Self;

    fn clear_subject_id(&mut self) -> &mut Self;

    fn relation(&mut self, relation: impl ToString) -> &mut Self;

    fn clear_relation(&mut self) -> &mut Self;
}

impl SubjectFilterBuilder for SubjectFilter {
    fn new(subject_type: impl ToString) -> Self {
        SubjectFilter {
            subject_type: subject_type.to_string(),
            ..Default::default()
        }
    }

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut Self {
        self.subject_type = subject_type.to_string();
        self
    }

    fn subject_id(&mut self, subject_id: impl ToString) -> &mut Self {
        self.optional_subject_id = subject_id.to_string();
        self
    }

    fn clear_subject_id(&mut self) -> &mut Self {
        self.optional_subject_id.clear();
        self
    }

    fn relation(&mut self, relation: impl ToString) -> &mut Self {
        self.optional_relation = Some(RelationFilter {
            relation: relation.to_string(),
        });
        self
    }

    fn clear_relation(&mut self) -> &mut Self {
        self.optional_relation = None;
        self
    }
}

pub trait WriteRelationshipsRequestBuilder {
    fn new(updates: impl IntoIterator<Item = RelationshipUpdate>) -> Self;

    fn updates(&mut self) -> &mut Vec<RelationshipUpdate>;

    fn add_relationship_update(
        &mut self,
        operation: RelationshipUpdateOperation,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate;

    fn create_relationship(
        &mut self,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate;

    fn update_relationship(
        &mut self,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate;

    fn delete_relationship(
        &mut self,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate;

    fn preconditions(&mut self) -> &mut Vec<Precondition>;

    fn add_precondition(&mut self, operation: PreconditionOperation) -> &mut Precondition;

    fn match_precondition(&mut self) -> &mut Precondition;

    fn not_match_precondition(&mut self) -> &mut Precondition;

    fn clear_preconditions(&mut self) -> &mut Self;
}

impl WriteRelationshipsRequestBuilder for WriteRelationshipsRequest {
    fn new(updates: impl IntoIterator<Item = RelationshipUpdate>) -> Self {
        Self {
            updates: updates.into_iter().collect(),
            ..Default::default()
        }
    }

    fn updates(&mut self) -> &mut Vec<RelationshipUpdate> {
        &mut self.updates
    }

    fn add_relationship_update(
        &mut self,
        operation: RelationshipUpdateOperation,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate {
        let i = self.updates.len();
        self.updates.push(RelationshipUpdate {
            operation: operation.into(),
            relationship: Some(Relationship::new(
                object_type,
                object_id,
                relation,
                subject_type,
                subject_id,
            )),
        });
        &mut self.updates[i]
    }

    fn create_relationship(
        &mut self,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate {
        self.add_relationship_update(
            RelationshipUpdateOperation::Create,
            object_type,
            object_id,
            relation,
            subject_type,
            subject_id,
        )
    }

    fn update_relationship(
        &mut self,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate {
        self.add_relationship_update(
            RelationshipUpdateOperation::Touch,
            object_type,
            object_id,
            relation,
            subject_type,
            subject_id,
        )
    }

    fn delete_relationship(
        &mut self,
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> &mut RelationshipUpdate {
        self.add_relationship_update(
            RelationshipUpdateOperation::Delete,
            object_type,
            object_id,
            relation,
            subject_type,
            subject_id,
        )
    }

    fn preconditions(&mut self) -> &mut Vec<Precondition> {
        &mut self.optional_preconditions
    }

    fn add_precondition(&mut self, operation: PreconditionOperation) -> &mut Precondition {
        let i: usize = self.optional_preconditions.len();
        self.optional_preconditions.push(Default::default());
        let precondition = &mut self.optional_preconditions[i];
        precondition.operation = operation.into();
        precondition
    }

    fn match_precondition(&mut self) -> &mut Precondition {
        self.add_precondition(PreconditionOperation::MustMatch)
    }

    fn not_match_precondition(&mut self) -> &mut Precondition {
        self.add_precondition(PreconditionOperation::MustNotMatch)
    }

    fn clear_preconditions(&mut self) -> &mut Self {
        self.optional_preconditions.clear();
        self
    }
}

pub trait RelationshipBuilder {
    fn new(
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> Self;

    fn object_type(&mut self, object_type: impl ToString) -> &mut Self;

    fn object_id(&mut self, object_id: impl ToString) -> &mut Self;

    fn relation(&mut self, relation: impl ToString) -> &mut Self;

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut Self;

    fn subject_id(&mut self, subject_id: impl ToString) -> &mut Self;

    fn subject_relation(&mut self, subject_relation: impl ToString) -> &mut Self;

    fn clear_subject_relation(&mut self) -> &mut Self;

    fn caveat(
        &mut self,
        name: impl ToString,
        context: Option<impl IntoIterator<Item = (String, ContextValue)>>,
    ) -> &mut Self;

    fn clear_caveat(&mut self) -> &mut Self;
}

impl RelationshipBuilder for Relationship {
    fn new(
        object_type: impl ToString,
        object_id: impl ToString,
        relation: impl ToString,
        subject_type: impl ToString,
        subject_id: impl ToString,
    ) -> Self {
        Self {
            resource: Some(ObjectReference {
                object_type: object_type.to_string(),
                object_id: object_id.to_string(),
            }),
            relation: relation.to_string(),
            subject: Some(SubjectReference {
                object: Some(ObjectReference {
                    object_type: subject_type.to_string(),
                    object_id: subject_id.to_string(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn object_type(&mut self, object_type: impl ToString) -> &mut Self {
        self.resource
            .get_or_insert_with(Default::default)
            .object_type = object_type.to_string();
        self
    }

    fn object_id(&mut self, object_id: impl ToString) -> &mut Self {
        self.resource.get_or_insert_with(Default::default).object_id = object_id.to_string();
        self
    }

    fn relation(&mut self, relation: impl ToString) -> &mut Self {
        self.relation = relation.to_string();
        self
    }

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut Self {
        self.subject
            .get_or_insert_with(Default::default)
            .object
            .get_or_insert_with(Default::default)
            .object_type = subject_type.to_string();
        self
    }

    fn subject_id(&mut self, subject_id: impl ToString) -> &mut Self {
        self.subject
            .get_or_insert_with(Default::default)
            .object
            .get_or_insert_with(Default::default)
            .object_id = subject_id.to_string();
        self
    }

    fn subject_relation(&mut self, subject_relation: impl ToString) -> &mut Self {
        self.subject
            .get_or_insert_with(Default::default)
            .optional_relation = subject_relation.to_string();
        self
    }

    fn clear_subject_relation(&mut self) -> &mut Self {
        if let Some(subject) = self.subject.as_mut() {
            subject.optional_relation.clear();
        }
        self
    }

    fn caveat(
        &mut self,
        name: impl ToString,
        context: Option<impl IntoIterator<Item = (String, ContextValue)>>,
    ) -> &mut Self {
        let caveat = self.optional_caveat.get_or_insert_with(Default::default);
        caveat.caveat_name = name.to_string();
        if let Some(context) = context {
            let fields = &mut caveat.context.get_or_insert_with(Default::default).fields;
            for (key, value) in context.into_iter() {
                fields.insert(key, Value { kind: Some(value) });
            }
        }
        self
    }

    fn clear_caveat(&mut self) -> &mut Self {
        self.optional_caveat = None;
        self
    }
}

impl RelationshipFilterBuilder for Precondition {
    fn new() -> Self {
        Default::default()
    }

    fn resource_type(&mut self, resource_type: impl ToString) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .resource_type(resource_type);
        self
    }

    fn clear_resource_type(&mut self) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .clear_resource_type();
        self
    }

    fn resource_id(&mut self, resource_id: impl ToString) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .resource_id(resource_id);
        self
    }

    fn clear_resource_id(&mut self) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .clear_resource_id();
        self
    }

    fn resource_id_prefix(&mut self, resource_id_prefix: impl ToString) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .resource_id_prefix(resource_id_prefix);
        self
    }

    fn clear_resource_id_prefix(&mut self) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .clear_resource_id_prefix();
        self
    }

    fn relation(&mut self, relation: impl ToString) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .relation(relation);
        self
    }

    fn clear_relation(&mut self) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .clear_relation();
        self
    }

    fn subject_type(&mut self, subject_type: impl ToString) -> &mut SubjectFilter {
        self.filter
            .get_or_insert_with(Default::default)
            .subject_type(subject_type)
    }

    fn subject_filter(&mut self) -> Option<&mut SubjectFilter> {
        self.filter
            .get_or_insert_with(Default::default)
            .subject_filter()
    }

    fn clear_subject_filter(&mut self) -> &mut Self {
        self.filter
            .get_or_insert_with(Default::default)
            .clear_subject_filter();
        self
    }
}
