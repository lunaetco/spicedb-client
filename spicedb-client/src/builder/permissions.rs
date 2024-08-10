use spicedb_grpc::authzed::api::v1::{
    subject_filter::RelationFilter, Consistency, Cursor, ReadRelationshipsRequest,
    RelationshipFilter, SubjectFilter,
};

use crate::types::ConsistencyRequirement;

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
