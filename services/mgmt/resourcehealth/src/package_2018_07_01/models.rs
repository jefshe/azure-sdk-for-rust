#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Events {
    pub value: Vec<Event>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<event::Properties>,
}
pub mod event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
        pub event_type: Option<properties::EventType>,
        #[serde(rename = "eventSource", skip_serializing_if = "Option::is_none")]
        pub event_source: Option<properties::EventSource>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub header: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub level: Option<properties::Level>,
        #[serde(rename = "eventLevel", skip_serializing_if = "Option::is_none")]
        pub event_level: Option<properties::EventLevel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub article: Option<properties::Article>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub links: Vec<Link>,
        #[serde(rename = "impactStartTime", skip_serializing_if = "Option::is_none")]
        pub impact_start_time: Option<String>,
        #[serde(rename = "impactMitigationTime", skip_serializing_if = "Option::is_none")]
        pub impact_mitigation_time: Option<String>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub impact: Vec<Impact>,
        #[serde(rename = "recommendedActions", skip_serializing_if = "Option::is_none")]
        pub recommended_actions: Option<properties::RecommendedActions>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub faqs: Vec<Faq>,
        #[serde(rename = "isHIR", skip_serializing_if = "Option::is_none")]
        pub is_hir: Option<bool>,
        #[serde(rename = "enableMicrosoftSupport", skip_serializing_if = "Option::is_none")]
        pub enable_microsoft_support: Option<bool>,
        #[serde(rename = "enableChatWithUs", skip_serializing_if = "Option::is_none")]
        pub enable_chat_with_us: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<i64>,
        #[serde(rename = "lastUpdateTime", skip_serializing_if = "Option::is_none")]
        pub last_update_time: Option<String>,
        #[serde(rename = "hirStage", skip_serializing_if = "Option::is_none")]
        pub hir_stage: Option<String>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum EventType {
            ServiceIssue,
            PlannedMaintenance,
            HealthAdvisory,
            #[serde(rename = "RCA")]
            Rca,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum EventSource {
            ResourceHealth,
            ServiceHealth,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Active,
            Resolved,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Critical,
            Warning,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum EventLevel {
            Critical,
            Warning,
            Informational,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Article {
            #[serde(rename = "articleContent", skip_serializing_if = "Option::is_none")]
            pub article_content: Option<String>,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct RecommendedActions {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub message: Option<String>,
            #[serde(skip_serializing_if = "Vec::is_empty")]
            pub actions: Vec<serde_json::Value>,
            #[serde(rename = "localeCode", skip_serializing_if = "Option::is_none")]
            pub locale_code: Option<String>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<link::Type>,
    #[serde(rename = "displayText", skip_serializing_if = "Option::is_none")]
    pub display_text: Option<link::DisplayText>,
    #[serde(rename = "extensionName", skip_serializing_if = "Option::is_none")]
    pub extension_name: Option<String>,
    #[serde(rename = "bladeName", skip_serializing_if = "Option::is_none")]
    pub blade_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
pub mod link {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Button,
        Hyperlink,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DisplayText {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Faq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(rename = "localeCode", skip_serializing_if = "Option::is_none")]
    pub locale_code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Impact {
    #[serde(rename = "impactedService", skip_serializing_if = "Option::is_none")]
    pub impacted_service: Option<String>,
    #[serde(rename = "impactedRegions", skip_serializing_if = "Vec::is_empty")]
    pub impacted_regions: Vec<ImpactedServiceRegion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpactedServiceRegion {
    #[serde(rename = "impactedRegion", skip_serializing_if = "Option::is_none")]
    pub impacted_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<impacted_service_region::Status>,
    #[serde(rename = "impactedSubscriptions", skip_serializing_if = "Vec::is_empty")]
    pub impacted_subscriptions: Vec<String>,
    #[serde(rename = "lastUpdateTime", skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub updates: Vec<Update>,
}
pub mod impacted_service_region {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Resolved,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "updateDateTime", skip_serializing_if = "Option::is_none")]
    pub update_date_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityStatusListResult {
    pub value: Vec<AvailabilityStatus>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<availability_status::Properties>,
}
pub mod availability_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "availabilityState", skip_serializing_if = "Option::is_none")]
        pub availability_state: Option<properties::AvailabilityState>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub summary: Option<String>,
        #[serde(rename = "detailedStatus", skip_serializing_if = "Option::is_none")]
        pub detailed_status: Option<String>,
        #[serde(rename = "reasonType", skip_serializing_if = "Option::is_none")]
        pub reason_type: Option<String>,
        #[serde(rename = "rootCauseAttributionTime", skip_serializing_if = "Option::is_none")]
        pub root_cause_attribution_time: Option<String>,
        #[serde(rename = "healthEventType", skip_serializing_if = "Option::is_none")]
        pub health_event_type: Option<String>,
        #[serde(rename = "healthEventCause", skip_serializing_if = "Option::is_none")]
        pub health_event_cause: Option<String>,
        #[serde(rename = "healthEventCategory", skip_serializing_if = "Option::is_none")]
        pub health_event_category: Option<String>,
        #[serde(rename = "healthEventId", skip_serializing_if = "Option::is_none")]
        pub health_event_id: Option<String>,
        #[serde(rename = "resolutionETA", skip_serializing_if = "Option::is_none")]
        pub resolution_eta: Option<String>,
        #[serde(rename = "occurredTime", skip_serializing_if = "Option::is_none")]
        pub occurred_time: Option<String>,
        #[serde(rename = "reasonChronicity", skip_serializing_if = "Option::is_none")]
        pub reason_chronicity: Option<properties::ReasonChronicity>,
        #[serde(rename = "reportedTime", skip_serializing_if = "Option::is_none")]
        pub reported_time: Option<String>,
        #[serde(rename = "recentlyResolved", skip_serializing_if = "Option::is_none")]
        pub recently_resolved: Option<properties::RecentlyResolved>,
        #[serde(rename = "recommendedActions", skip_serializing_if = "Vec::is_empty")]
        pub recommended_actions: Vec<RecommendedAction>,
        #[serde(rename = "serviceImpactingEvents", skip_serializing_if = "Vec::is_empty")]
        pub service_impacting_events: Vec<ServiceImpactingEvent>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum AvailabilityState {
            Available,
            Unavailable,
            Degraded,
            Unknown,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ReasonChronicity {
            Transient,
            Persistent,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct RecentlyResolved {
            #[serde(rename = "unavailableOccurredTime", skip_serializing_if = "Option::is_none")]
            pub unavailable_occurred_time: Option<String>,
            #[serde(rename = "resolvedTime", skip_serializing_if = "Option::is_none")]
            pub resolved_time: Option<String>,
            #[serde(rename = "unavailabilitySummary", skip_serializing_if = "Option::is_none")]
            pub unavailability_summary: Option<String>,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecommendedAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "actionUrl", skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(rename = "actionUrlText", skip_serializing_if = "Option::is_none")]
    pub action_url_text: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceImpactingEvent {
    #[serde(rename = "eventStartTime", skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<String>,
    #[serde(rename = "eventStatusLastModifiedTime", skip_serializing_if = "Option::is_none")]
    pub event_status_last_modified_time: Option<String>,
    #[serde(rename = "correlationId", skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<service_impacting_event::Status>,
    #[serde(rename = "incidentProperties", skip_serializing_if = "Option::is_none")]
    pub incident_properties: Option<service_impacting_event::IncidentProperties>,
}
pub mod service_impacting_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Status {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IncidentProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(rename = "incidentType", skip_serializing_if = "Option::is_none")]
        pub incident_type: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusBanner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(rename = "lastModifiedTime", skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImpactedRegion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssueImpact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<ImpactedRegion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusActiveEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "trackingId", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<status_active_event::Severity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<status_active_event::Stage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(rename = "lastModifiedTime", skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub impacts: Vec<EmergingIssueImpact>,
}
pub mod status_active_event {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Information,
        Warning,
        Error,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Stage {
        Active,
        Resolve,
        Archived,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssuesGetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmergingIssue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssue {
    #[serde(rename = "refreshTimestamp", skip_serializing_if = "Option::is_none")]
    pub refresh_timestamp: Option<String>,
    #[serde(rename = "statusBanners", skip_serializing_if = "Vec::is_empty")]
    pub status_banners: Vec<StatusBanner>,
    #[serde(rename = "statusActiveEvents", skip_serializing_if = "Vec::is_empty")]
    pub status_active_events: Vec<StatusActiveEvent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmergingIssueListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EmergingIssuesGetResult>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntityListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetadataEntity>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntity {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataEntityProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataEntityProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "dependsOn", skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(rename = "applicableScenarios", skip_serializing_if = "Vec::is_empty")]
    pub applicable_scenarios: Vec<String>,
    #[serde(rename = "supportedValues", skip_serializing_if = "Vec::is_empty")]
    pub supported_values: Vec<MetadataSupportedValueDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSupportedValueDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
