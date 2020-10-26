#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod account {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get_firewall_rule(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        firewall_rule_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<FirewallRule, get_firewall_rule::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules/{}",
            &operation_config.base_path, subscription_id, resource_group_name, account_name, firewall_rule_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get_firewall_rule::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get_firewall_rule::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_firewall_rule::ResponseBytesError)?;
                let rsp_value: FirewallRule = serde_json::from_slice(&body).context(get_firewall_rule::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get_firewall_rule::ResponseBytesError)?;
                get_firewall_rule::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get_firewall_rule {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete_firewall_rule(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        firewall_rule_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete_firewall_rule::Response, delete_firewall_rule::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules/{}",
            &operation_config.base_path, subscription_id, resource_group_name, account_name, firewall_rule_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete_firewall_rule::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete_firewall_rule::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete_firewall_rule::Response::Ok200),
            StatusCode::NO_CONTENT => Ok(delete_firewall_rule::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete_firewall_rule::ResponseBytesError)?;
                delete_firewall_rule::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete_firewall_rule {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_firewall_rules(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreFirewallRuleListResult, list_firewall_rules::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules",
            &operation_config.base_path, subscription_id, resource_group_name, account_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_firewall_rules::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_firewall_rules::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_firewall_rules::ResponseBytesError)?;
                let rsp_value: DataLakeStoreFirewallRuleListResult =
                    serde_json::from_slice(&body).context(list_firewall_rules::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_firewall_rules::ResponseBytesError)?;
                list_firewall_rules::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_firewall_rules {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create_or_update_firewall_rule(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        name: &str,
        parameters: &FirewallRule,
        subscription_id: &str,
    ) -> std::result::Result<FirewallRule, create_or_update_firewall_rule::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/firewallRules/{}",
            &operation_config.base_path, subscription_id, resource_group_name, account_name, name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update_firewall_rule::BuildRequestError)?;
        let rsp = client
            .execute(req)
            .await
            .context(create_or_update_firewall_rule::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update_firewall_rule::ResponseBytesError)?;
                let rsp_value: FirewallRule =
                    serde_json::from_slice(&body).context(create_or_update_firewall_rule::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update_firewall_rule::ResponseBytesError)?;
                create_or_update_firewall_rule::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update_firewall_rule {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &DataLakeStoreAccount,
        subscription_id: &str,
    ) -> std::result::Result<create::Response, create::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccount = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Created201(rsp_value))
            }
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccount = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                create::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(DataLakeStoreAccount),
            Ok200(DataLakeStoreAccount),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &DataLakeStoreAccount,
        subscription_id: &str,
    ) -> std::result::Result<update::Response, update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccount = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Ok200(rsp_value))
            }
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccount = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(update::Response::Created201(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(DataLakeStoreAccount),
            Created201(DataLakeStoreAccount),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreAccount, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            &operation_config.base_path, subscription_id, resource_group_name, account_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccount = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}",
            &operation_config.base_path, subscription_id, resource_group_name, account_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NOT_FOUND => delete::NotFound404 {}.fail(),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                delete::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn enable_key_vault(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        account_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<(), enable_key_vault::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts/{}/enableKeyVault",
            &operation_config.base_path, subscription_id, resource_group_name, account_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(enable_key_vault::BuildRequestError)?;
        let rsp = client.execute(req).await.context(enable_key_vault::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(()),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(enable_key_vault::ResponseBytesError)?;
                enable_key_vault::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod enable_key_vault {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        filter: Option<&str>,
        top: Option<i32>,
        skip: Option<i32>,
        expand: Option<&str>,
        select: Option<&str>,
        orderby: Option<&str>,
        count: Option<bool>,
        search: Option<&str>,
        format: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreAccountListResult, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataLakeStore/accounts",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        if let Some(top) = top {
            req_builder = req_builder.query(&[("$top", top)]);
        }
        if let Some(skip) = skip {
            req_builder = req_builder.query(&[("$skip", skip)]);
        }
        if let Some(expand) = expand {
            req_builder = req_builder.query(&[("$expand", expand)]);
        }
        if let Some(select) = select {
            req_builder = req_builder.query(&[("$select", select)]);
        }
        if let Some(orderby) = orderby {
            req_builder = req_builder.query(&[("$orderby", orderby)]);
        }
        if let Some(count) = count {
            req_builder = req_builder.query(&[("$count", count)]);
        }
        if let Some(search) = search {
            req_builder = req_builder.query(&[("$search", search)]);
        }
        if let Some(format) = format {
            req_builder = req_builder.query(&[("$format", format)]);
        }
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccountListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        filter: Option<&str>,
        top: Option<i32>,
        skip: Option<i32>,
        expand: Option<&str>,
        select: Option<&str>,
        orderby: Option<&str>,
        count: Option<bool>,
        search: Option<&str>,
        format: Option<&str>,
        subscription_id: &str,
    ) -> std::result::Result<DataLakeStoreAccountListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.DataLakeStore/accounts",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token) = &operation_config.bearer_access_token {
            req_builder = req_builder.bearer_auth(token);
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        if let Some(filter) = filter {
            req_builder = req_builder.query(&[("$filter", filter)]);
        }
        if let Some(top) = top {
            req_builder = req_builder.query(&[("$top", top)]);
        }
        if let Some(skip) = skip {
            req_builder = req_builder.query(&[("$skip", skip)]);
        }
        if let Some(expand) = expand {
            req_builder = req_builder.query(&[("$expand", expand)]);
        }
        if let Some(select) = select {
            req_builder = req_builder.query(&[("$select", select)]);
        }
        if let Some(orderby) = orderby {
            req_builder = req_builder.query(&[("$orderby", orderby)]);
        }
        if let Some(count) = count {
            req_builder = req_builder.query(&[("$count", count)]);
        }
        if let Some(search) = search {
            req_builder = req_builder.query(&[("$search", search)]);
        }
        if let Some(format) = format {
            req_builder = req_builder.query(&[("$format", format)]);
        }
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: DataLakeStoreAccountListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
        }
    }
}