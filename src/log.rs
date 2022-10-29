use chrono::prelude::*;

use chrono::Utc;

use uuid::{uuid, Uuid};

#[derive(Debug)]
pub struct Log {
    pub user_id: Option<Uuid>,

    // fk to organization
    pub organization_id: Option<Uuid>,
    pub organization: Option<String>,

    // fk to project
    pub project_id: Option<Uuid>,
    pub project: Option<String>,

    // fk subproject
    pub subproject_id: Option<Uuid>,
    pub subproject: Option<String>,

    pub activity: Option<String>,
    pub detail1: Option<String>,
    pub detail2: Option<String>,
    pub spirits: Option<u8>,
    pub notes: Option<String>,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}

impl Log {
    pub fn new(
        user_id: Option<Uuid>,
        organization_id: Option<Uuid>,
        organization: Option<String>,
        project_id: Option<Uuid>,
        project: Option<String>,
        subproject_id: Option<Uuid>,
        subproject: Option<String>,
        activity: Option<String>,
        detail1: Option<String>,
        detail2: Option<String>,
        spirits: Option<u8>,
        notes: Option<String>,
    ) -> Log {
        Log {
            user_id,
            organization_id,
            organization,
            project_id,
            project,
            subproject_id,
            subproject,
            activity,
            detail1,
            detail2,
            spirits,
            notes,
            start: Utc::now(),
            end: None,
        }

    }
}