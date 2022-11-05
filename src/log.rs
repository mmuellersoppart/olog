use std::fmt::{Display, Formatter};

use chrono::prelude::*;
use chrono::Utc;

use csv::Writer;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub user_id: Option<u64>,

    // fk to organization
    pub organization_id: Option<u64>,
    pub organization: Option<String>,

    // fk to project
    pub project_id: Option<u64>,
    pub project: Option<String>,

    // fk subproject
    pub subproject_id: Option<u64>,
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
        user_id: Option<u64>,
        organization_id: Option<u64>,
        organization: Option<String>,
        project_id: Option<u64>,
        project: Option<String>,
        subproject_id: Option<u64>,
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



impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let user_id = match self.user_id {
            Some(ref id) => id.to_string(),
            None => "".to_string()
        };

        let organization_id = match self.organization_id {
            Some(ref id) => id.to_string(),
            None => "".to_string()
        };

        let organization = match self.organization {
            Some(ref id) => id,
            None => ""
        };

        let project_id = match self.project_id {
            Some(ref id) => id.to_string(),
            None => "".to_string()
        };

        let project = match self.project {
            Some(ref name) => name,
            None => ""
        } ;

        write!(f, "{},{},{},{},{}",
              user_id,
              organization_id,
              organization,
              project_id,
              project,
              // // fk subproject
              // self.subproject_id,
              // self.subproject,
              // self.activity,
              // self.detail1,
              // self.detail2,
              // self.spirits,
              // self.notes,
              // self.start,
              // self.end,
        )
    }
}