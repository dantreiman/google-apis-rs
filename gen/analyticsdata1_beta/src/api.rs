use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your Google Analytics data
    Analytic,

    /// See and download your Google Analytics data
    AnalyticReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Analytic => "https://www.googleapis.com/auth/analytics",
            Scope::AnalyticReadonly => "https://www.googleapis.com/auth/analytics.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::AnalyticReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all AnalyticsData related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// use analyticsdata1_beta::api::RunReportRequest;
/// use analyticsdata1_beta::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use analyticsdata1_beta::AnalyticsData;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RunReportRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().run_report(req, "property")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct AnalyticsData<> {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for AnalyticsData<> {}

impl<'a, > AnalyticsData<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> AnalyticsData<> {
        AnalyticsData {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/2.0.5".to_string(),
            _base_url: "https://analyticsdata.googleapis.com/".to_string(),
            _root_url: "https://analyticsdata.googleapis.com/".to_string(),
        }
    }

    pub fn properties(&'a self) -> PropertyMethods<'a> {
        PropertyMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://analyticsdata.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://analyticsdata.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The batch request containing multiple pivot report requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run pivot reports properties](PropertyBatchRunPivotReportCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunPivotReportsRequest {
    /// Individual requests. Each request has a separate pivot report response. Each batch request is allowed up to 5 requests.
    pub requests: Option<Vec<RunPivotReportRequest>>,
}

impl client::RequestValue for BatchRunPivotReportsRequest {}


/// The batch response containing multiple pivot reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run pivot reports properties](PropertyBatchRunPivotReportCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunPivotReportsResponse {
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#batchRunPivotReports". Useful to distinguish between response types in JSON.
    pub kind: Option<String>,
    /// Individual responses. Each response has a separate pivot report request.
    #[serde(rename="pivotReports")]
    pub pivot_reports: Option<Vec<RunPivotReportResponse>>,
}

impl client::ResponseResult for BatchRunPivotReportsResponse {}


/// The batch request containing multiple report requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run reports properties](PropertyBatchRunReportCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunReportsRequest {
    /// Individual requests. Each request has a separate report response. Each batch request is allowed up to 5 requests.
    pub requests: Option<Vec<RunReportRequest>>,
}

impl client::RequestValue for BatchRunReportsRequest {}


/// The batch response containing multiple reports.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch run reports properties](PropertyBatchRunReportCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchRunReportsResponse {
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#batchRunReports". Useful to distinguish between response types in JSON.
    pub kind: Option<String>,
    /// Individual responses. Each response has a separate report request.
    pub reports: Option<Vec<RunReportResponse>>,
}

impl client::ResponseResult for BatchRunReportsResponse {}


/// To express that the result needs to be between two numbers (inclusive).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BetweenFilter {
    /// Begins with this number.
    #[serde(rename="fromValue")]
    pub from_value: Option<NumericValue>,
    /// Ends with this number.
    #[serde(rename="toValue")]
    pub to_value: Option<NumericValue>,
}

impl client::Part for BetweenFilter {}


/// Used to convert a dimension value to a single case.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CaseExpression {
    /// Name of a dimension. The name must refer back to a name in dimensions field of the request.
    #[serde(rename="dimensionName")]
    pub dimension_name: Option<String>,
}

impl client::Part for CaseExpression {}


/// Defines a cohort selection criteria. A cohort is a group of users who share a common characteristic. For example, users with the same `firstSessionDate` belong to the same cohort.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cohort {
    /// The cohort selects users whose first touch date is between start date and end date defined in the `dateRange`. This `dateRange` does not specify the full date range of event data that is present in a cohort report. In a cohort report, this `dateRange` is extended by the granularity and offset present in the `cohortsRange`; event data for the extended reporting date range is present in a cohort report. In a cohort request, this `dateRange` is required and the `dateRanges` in the `RunReportRequest` or `RunPivotReportRequest` must be unspecified. This `dateRange` should generally be aligned with the cohort's granularity. If `CohortsRange` uses daily granularity, this `dateRange` can be a single day. If `CohortsRange` uses weekly granularity, this `dateRange` can be aligned to a week boundary, starting at Sunday and ending Saturday. If `CohortsRange` uses monthly granularity, this `dateRange` can be aligned to a month, starting at the first and ending on the last day of the month.
    #[serde(rename="dateRange")]
    pub date_range: Option<DateRange>,
    /// Dimension used by the cohort. Required and only supports `firstSessionDate`.
    pub dimension: Option<String>,
    /// Assigns a name to this cohort. The dimension `cohort` is valued to this name in a report response. If set, cannot begin with `cohort_` or `RESERVED_`. If not set, cohorts are named by their zero based index `cohort_0`, `cohort_1`, etc.
    pub name: Option<String>,
}

impl client::Part for Cohort {}


/// Optional settings of a cohort report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortReportSettings {
    /// If true, accumulates the result from first touch day to the end day. Not supported in `RunReportRequest`.
    pub accumulate: Option<bool>,
}

impl client::Part for CohortReportSettings {}


/// The specification of cohorts for a cohort report. Cohort reports create a time series of user retention for the cohort. For example, you could select the cohort of users that were acquired in the first week of September and follow that cohort for the next six weeks. Selecting the users acquired in the first week of September cohort is specified in the `cohort` object. Following that cohort for the next six weeks is specified in the `cohortsRange` object. For examples, see [Cohort Report Examples](https://developers.google.com/analytics/devguides/reporting/data/v1/advanced#cohort_report_examples). The report response could show a weekly time series where say your app has retained 60% of this cohort after three weeks and 25% of this cohort after six weeks. These two percentages can be calculated by the metric `cohortActiveUsers/cohortTotalUsers` and will be separate rows in the report.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortSpec {
    /// Optional settings for a cohort report.
    #[serde(rename="cohortReportSettings")]
    pub cohort_report_settings: Option<CohortReportSettings>,
    /// Defines the selection criteria to group users into cohorts. Most cohort reports define only a single cohort. If multiple cohorts are specified, each cohort can be recognized in the report by their name.
    pub cohorts: Option<Vec<Cohort>>,
    /// Cohort reports follow cohorts over an extended reporting date range. This range specifies an offset duration to follow the cohorts over.
    #[serde(rename="cohortsRange")]
    pub cohorts_range: Option<CohortsRange>,
}

impl client::Part for CohortSpec {}


/// Configures the extended reporting date range for a cohort report. Specifies an offset duration to follow the cohorts over.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CohortsRange {
    /// Required. `endOffset` specifies the end date of the extended reporting date range for a cohort report. `endOffset` can be any positive integer but is commonly set to 5 to 10 so that reports contain data on the cohort for the next several granularity time periods. If `granularity` is `DAILY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset` days. If `granularity` is `WEEKLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 7` days. If `granularity` is `MONTHLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 30` days.
    #[serde(rename="endOffset")]
    pub end_offset: Option<i32>,
    /// Required. The granularity used to interpret the `startOffset` and `endOffset` for the extended reporting date range for a cohort report.
    pub granularity: Option<String>,
    /// `startOffset` specifies the start date of the extended reporting date range for a cohort report. `startOffset` is commonly set to 0 so that reports contain data from the acquisition of the cohort forward. If `granularity` is `DAILY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset` days. If `granularity` is `WEEKLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 7` days. If `granularity` is `MONTHLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 30` days.
    #[serde(rename="startOffset")]
    pub start_offset: Option<i32>,
}

impl client::Part for CohortsRange {}


/// Used to combine dimension values to a single dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConcatenateExpression {
    /// The delimiter placed between dimension names. Delimiters are often single characters such as "|" or "," but can be longer strings. If a dimension value contains the delimiter, both will be present in response with no distinction. For example if dimension 1 value = "US,FR", dimension 2 value = "JP", and delimiter = ",", then the response will contain "US,FR,JP".
    pub delimiter: Option<String>,
    /// Names of dimensions. The names must refer back to names in the dimensions field of the request.
    #[serde(rename="dimensionNames")]
    pub dimension_names: Option<Vec<String>>,
}

impl client::Part for ConcatenateExpression {}


/// A contiguous set of days: startDate, startDate + 1, ..., endDate. Requests are allowed up to 4 date ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone.
    #[serde(rename="endDate")]
    pub end_date: Option<String>,
    /// Assigns a name to this date range. The dimension `dateRange` is valued to this name in a report response. If set, cannot begin with `date_range_` or `RESERVED_`. If not set, date ranges are named by their zero based index in the request: `date_range_0`, `date_range_1`, etc.
    pub name: Option<String>,
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone.
    #[serde(rename="startDate")]
    pub start_date: Option<String>,
}

impl client::Part for DateRange {}


/// Dimensions are attributes of your data. For example, the dimension city indicates the city from which an event originates. Dimension values in report responses are strings; for example, city could be "Paris" or "New York". Requests are allowed up to 8 dimensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    /// One dimension can be the result of an expression of multiple dimensions. For example, dimension "country, city": concatenate(country, ", ", city).
    #[serde(rename="dimensionExpression")]
    pub dimension_expression: Option<DimensionExpression>,
    /// The name of the dimension. See the [API Dimensions](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions) for the list of dimension names. If `dimensionExpression` is specified, `name` can be any string that you would like within the allowed character set. For example if a `dimensionExpression` concatenates `country` and `city`, you could call that dimension `countryAndCity`. Dimension names that you choose must match the regular expression "^[a-zA-Z0-9_]$". Dimensions are referenced by `name` in `dimensionFilter`, `orderBys`, `dimensionExpression`, and `pivots`.
    pub name: Option<String>,
}

impl client::Part for Dimension {}


/// Used to express a dimension which is the result of a formula of multiple dimensions. Example usages: 1) lower_case(dimension) 2) concatenate(dimension1, symbol, dimension2).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionExpression {
    /// Used to combine dimension values to a single dimension. For example, dimension "country, city": concatenate(country, ", ", city).
    pub concatenate: Option<ConcatenateExpression>,
    /// Used to convert a dimension value to lower case.
    #[serde(rename="lowerCase")]
    pub lower_case: Option<CaseExpression>,
    /// Used to convert a dimension value to upper case.
    #[serde(rename="upperCase")]
    pub upper_case: Option<CaseExpression>,
}

impl client::Part for DimensionExpression {}


/// Describes a dimension column in the report. Dimensions requested in a report produce column entries within rows and DimensionHeaders. However, dimensions used exclusively within filters or expressions do not produce columns in a report; correspondingly, those dimensions do not produce headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionHeader {
    /// The dimension's name.
    pub name: Option<String>,
}

impl client::Part for DimensionHeader {}


/// Explains a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionMetadata {
    /// This dimension's name. Useable in [Dimension](#Dimension)'s `name`. For example, `eventName`.
    #[serde(rename="apiName")]
    pub api_name: Option<String>,
    /// True if the dimension is a custom dimension for this property.
    #[serde(rename="customDefinition")]
    pub custom_definition: Option<bool>,
    /// Still usable but deprecated names for this dimension. If populated, this dimension is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the dimension will be available only by `apiName`.
    #[serde(rename="deprecatedApiNames")]
    pub deprecated_api_names: Option<Vec<String>>,
    /// Description of how this dimension is used and calculated.
    pub description: Option<String>,
    /// This dimension's name within the Google Analytics user interface. For example, `Event name`.
    #[serde(rename="uiName")]
    pub ui_name: Option<String>,
}

impl client::Part for DimensionMetadata {}


/// Sorts by dimension values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionOrderBy {
    /// A dimension name in the request to order by.
    #[serde(rename="dimensionName")]
    pub dimension_name: Option<String>,
    /// Controls the rule for dimension value ordering.
    #[serde(rename="orderType")]
    pub order_type: Option<String>,
}

impl client::Part for DimensionOrderBy {}


/// The value of a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionValue {
    /// Value as a string if the dimension type is a string.
    pub value: Option<String>,
}

impl client::Part for DimensionValue {}


/// An expression to filter dimension or metric values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// A filter for two values.
    #[serde(rename="betweenFilter")]
    pub between_filter: Option<BetweenFilter>,
    /// The dimension name or metric name. Must be a name defined in dimensions or metrics.
    #[serde(rename="fieldName")]
    pub field_name: Option<String>,
    /// A filter for in list values.
    #[serde(rename="inListFilter")]
    pub in_list_filter: Option<InListFilter>,
    /// A filter for numeric or date values.
    #[serde(rename="numericFilter")]
    pub numeric_filter: Option<NumericFilter>,
    /// Strings related filter.
    #[serde(rename="stringFilter")]
    pub string_filter: Option<StringFilter>,
}

impl client::Part for Filter {}


/// To express dimension or metric filters. The fields in the same FilterExpression need to be either all dimensions or all metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterExpression {
    /// The FilterExpressions in and_group have an AND relationship.
    #[serde(rename="andGroup")]
    pub and_group: Option<FilterExpressionList>,
    /// A primitive filter. All fields in filter in same FilterExpression needs to be either all dimensions or metrics.
    pub filter: Option<Filter>,
    /// The FilterExpression is NOT of not_expression.
    #[serde(rename="notExpression")]
    pub not_expression: Option<Option<Box<FilterExpression>>>,
    /// The FilterExpressions in or_group have an OR relationship.
    #[serde(rename="orGroup")]
    pub or_group: Option<FilterExpressionList>,
}

impl client::Part for FilterExpression {}


/// A list of filter expressions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterExpressionList {
    /// A list of filter expressions.
    pub expressions: Option<Vec<FilterExpression>>,
}

impl client::Part for FilterExpressionList {}


/// The result needs to be in a list of string values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InListFilter {
    /// If true, the string value is case sensitive.
    #[serde(rename="caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// The list of string values. Must be non-empty.
    pub values: Option<Vec<String>>,
}

impl client::Part for InListFilter {}


/// The dimensions and metrics currently accepted in reporting methods.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get metadata properties](PropertyGetMetadataCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The dimension descriptions.
    pub dimensions: Option<Vec<DimensionMetadata>>,
    /// The metric descriptions.
    pub metrics: Option<Vec<MetricMetadata>>,
    /// Resource name of this metadata.
    pub name: Option<String>,
}

impl client::ResponseResult for Metadata {}


/// The quantitative measurements of a report. For example, the metric `eventCount` is the total number of events. Requests are allowed up to 10 metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    /// A mathematical expression for derived metrics. For example, the metric Event count per user is `eventCount/totalUsers`.
    pub expression: Option<String>,
    /// Indicates if a metric is invisible in the report response. If a metric is invisible, the metric will not produce a column in the response, but can be used in `metricFilter`, `orderBys`, or a metric `expression`.
    pub invisible: Option<bool>,
    /// The name of the metric. See the [API Metrics](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics) for the list of metric names. If `expression` is specified, `name` can be any string that you would like within the allowed character set. For example if `expression` is `screenPageViews/sessions`, you could call that metric's name = `viewsPerSession`. Metric names that you choose must match the regular expression "^[a-zA-Z0-9_]$". Metrics are referenced by `name` in `metricFilter`, `orderBys`, and metric `expression`.
    pub name: Option<String>,
}

impl client::Part for Metric {}


/// Describes a metric column in the report. Visible metrics requested in a report produce column entries within rows and MetricHeaders. However, metrics used exclusively within filters or expressions do not produce columns in a report; correspondingly, those metrics do not produce headers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricHeader {
    /// The metric's name.
    pub name: Option<String>,
    /// The metric's data type.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::Part for MetricHeader {}


/// Explains a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// A metric name. Useable in [Metric](#Metric)'s `name`. For example, `eventCount`.
    #[serde(rename="apiName")]
    pub api_name: Option<String>,
    /// True if the metric is a custom metric for this property.
    #[serde(rename="customDefinition")]
    pub custom_definition: Option<bool>,
    /// Still usable but deprecated names for this metric. If populated, this metric is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the metric will be available only by `apiName`.
    #[serde(rename="deprecatedApiNames")]
    pub deprecated_api_names: Option<Vec<String>>,
    /// Description of how this metric is used and calculated.
    pub description: Option<String>,
    /// The mathematical expression for this derived metric. Can be used in [Metric](#Metric)'s `expression` field for equivalent reports. Most metrics are not expressions, and for non-expressions, this field is empty.
    pub expression: Option<String>,
    /// The type of this metric.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// This metric's name within the Google Analytics user interface. For example, `Event count`.
    #[serde(rename="uiName")]
    pub ui_name: Option<String>,
}

impl client::Part for MetricMetadata {}


/// Sorts by metric values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricOrderBy {
    /// A metric name in the request to order by.
    #[serde(rename="metricName")]
    pub metric_name: Option<String>,
}

impl client::Part for MetricOrderBy {}


/// The value of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    /// Measurement value. See MetricHeader for type.
    pub value: Option<String>,
}

impl client::Part for MetricValue {}


/// Filters for numeric or date values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericFilter {
    /// The operation type for this filter.
    pub operation: Option<String>,
    /// A numeric value or a date value.
    pub value: Option<NumericValue>,
}

impl client::Part for NumericFilter {}


/// To represent a number.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumericValue {
    /// Double value
    #[serde(rename="doubleValue")]
    pub double_value: Option<f64>,
    /// Integer value
    #[serde(rename="int64Value")]
    pub int64_value: Option<String>,
}

impl client::Part for NumericValue {}


/// The sort options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrderBy {
    /// If true, sorts by descending order.
    pub desc: Option<bool>,
    /// Sorts results by a dimension's values.
    pub dimension: Option<DimensionOrderBy>,
    /// Sorts results by a metric's values.
    pub metric: Option<MetricOrderBy>,
    /// Sorts results by a metric's values within a pivot column group.
    pub pivot: Option<PivotOrderBy>,
}

impl client::Part for OrderBy {}


/// Describes the visible dimension columns and rows in the report response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pivot {
    /// Dimension names for visible columns in the report response. Including "dateRange" produces a date range column; for each row in the response, dimension values in the date range column will indicate the corresponding date range from the request.
    #[serde(rename="fieldNames")]
    pub field_names: Option<Vec<String>>,
    /// The number of unique combinations of dimension values to return in this pivot. The `limit` parameter is required. A `limit` of 10,000 is common for single pivot requests. The product of the `limit` for each `pivot` in a `RunPivotReportRequest` must not exceed 100,000. For example, a two pivot request with `limit: 1000` in each pivot will fail because the product is `1,000,000`.
    pub limit: Option<String>,
    /// Aggregate the metrics by dimensions in this pivot using the specified metric_aggregations.
    #[serde(rename="metricAggregations")]
    pub metric_aggregations: Option<Vec<String>>,
    /// The row count of the start row. The first row is counted as row 0.
    pub offset: Option<String>,
    /// Specifies how dimensions are ordered in the pivot. In the first Pivot, the OrderBys determine Row and PivotDimensionHeader ordering; in subsequent Pivots, the OrderBys determine only PivotDimensionHeader ordering. Dimensions specified in these OrderBys must be a subset of Pivot.field_names.
    #[serde(rename="orderBys")]
    pub order_bys: Option<Vec<OrderBy>>,
}

impl client::Part for Pivot {}


/// Summarizes dimension values from a row for this pivot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotDimensionHeader {
    /// Values of multiple dimensions in a pivot.
    #[serde(rename="dimensionValues")]
    pub dimension_values: Option<Vec<DimensionValue>>,
}

impl client::Part for PivotDimensionHeader {}


/// Dimensions' values in a single pivot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotHeader {
    /// The size is the same as the cardinality of the corresponding dimension combinations.
    #[serde(rename="pivotDimensionHeaders")]
    pub pivot_dimension_headers: Option<Vec<PivotDimensionHeader>>,
    /// The cardinality of the pivot. The total number of rows for this pivot's fields regardless of how the parameters `offset` and `limit` are specified in the request.
    #[serde(rename="rowCount")]
    pub row_count: Option<i32>,
}

impl client::Part for PivotHeader {}


/// Sorts by a pivot column group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotOrderBy {
    /// In the response to order by, order rows by this column. Must be a metric name from the request.
    #[serde(rename="metricName")]
    pub metric_name: Option<String>,
    /// Used to select a dimension name and value pivot. If multiple pivot selections are given, the sort occurs on rows where all pivot selection dimension name and value pairs match the row's dimension name and value pair.
    #[serde(rename="pivotSelections")]
    pub pivot_selections: Option<Vec<PivotSelection>>,
}

impl client::Part for PivotOrderBy {}


/// A pair of dimension names and values. Rows with this dimension pivot pair are ordered by the metric's value. For example if pivots = {{"browser", "Chrome"}} and metric_name = "Sessions", then the rows will be sorted based on Sessions in Chrome. ---------|----------|----------------|----------|---------------- | Chrome | Chrome | Safari | Safari ---------|----------|----------------|----------|---------------- Country | Sessions | Pages/Sessions | Sessions | Pages/Sessions ---------|----------|----------------|----------|---------------- US | 2 | 2 | 3 | 1 ---------|----------|----------------|----------|---------------- Canada | 3 | 1 | 4 | 1 ---------|----------|----------------|----------|----------------
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotSelection {
    /// Must be a dimension name from the request.
    #[serde(rename="dimensionName")]
    pub dimension_name: Option<String>,
    /// Order by only when the named dimension is this value.
    #[serde(rename="dimensionValue")]
    pub dimension_value: Option<String>,
}

impl client::Part for PivotSelection {}


/// Current state of all quotas for this Analytics Property. If any quota for a property is exhausted, all requests to that property will return Resource Exhausted errors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PropertyQuota {
    /// Standard Analytics Properties can send up to 10 concurrent requests; Analytics 360 Properties can use up to 50 concurrent requests.
    #[serde(rename="concurrentRequests")]
    pub concurrent_requests: Option<QuotaStatus>,
    /// Analytics Properties can send up to 120 requests with potentially thresholded dimensions per hour. In a batch request, each report request is individually counted for this quota if the request contains potentially thresholded dimensions.
    #[serde(rename="potentiallyThresholdedRequestsPerHour")]
    pub potentially_thresholded_requests_per_hour: Option<QuotaStatus>,
    /// Standard Analytics Properties and cloud project pairs can have up to 10 server errors per hour; Analytics 360 Properties and cloud project pairs can have up to 50 server errors per hour.
    #[serde(rename="serverErrorsPerProjectPerHour")]
    pub server_errors_per_project_per_hour: Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 25,000 tokens per day; Analytics 360 Properties can use 250,000 tokens per day. Most requests consume fewer than 10 tokens.
    #[serde(rename="tokensPerDay")]
    pub tokens_per_day: Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 5,000 tokens per hour; Analytics 360 Properties can use 50,000 tokens per hour. An API request consumes a single number of tokens, and that number is deducted from both the hourly and daily quotas.
    #[serde(rename="tokensPerHour")]
    pub tokens_per_hour: Option<QuotaStatus>,
}

impl client::Part for PropertyQuota {}


/// Current state for a particular quota group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QuotaStatus {
    /// Quota consumed by this request.
    pub consumed: Option<i32>,
    /// Quota remaining after this request.
    pub remaining: Option<i32>,
}

impl client::Part for QuotaStatus {}


/// Response's metadata carrying additional information about the report content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetaData {
    /// If true, indicates some buckets of dimension combinations are rolled into "(other)" row. This can happen for high cardinality reports.
    #[serde(rename="dataLossFromOtherRow")]
    pub data_loss_from_other_row: Option<bool>,
}

impl client::Part for ResponseMetaData {}


/// Report data for each row. For example if RunReportRequest contains: ```none "dimensions": [ { "name": "eventName" }, { "name": "countryId" } ], "metrics": [ { "name": "eventCount" } ] ``` One row with 'in_app_purchase' as the eventName, 'JP' as the countryId, and 15 as the eventCount, would be: ```none "dimensionValues": [ { "value": "in_app_purchase" }, { "value": "JP" } ], "metricValues": [ { "value": "15" } ] ```
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// List of requested dimension values. In a PivotReport, dimension_values are only listed for dimensions included in a pivot.
    #[serde(rename="dimensionValues")]
    pub dimension_values: Option<Vec<DimensionValue>>,
    /// List of requested visible metric values.
    #[serde(rename="metricValues")]
    pub metric_values: Option<Vec<MetricValue>>,
}

impl client::Part for Row {}


/// The request to generate a pivot report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run pivot report properties](PropertyRunPivotReportCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunPivotReportRequest {
    /// Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present.
    #[serde(rename="cohortSpec")]
    pub cohort_spec: Option<CohortSpec>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY". If the field is empty, the report uses the property's default currency.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
    /// The date range to retrieve event data for the report. If multiple date ranges are specified, event data from each date range is used in the report. A special dimension with field name "dateRange" can be included in a Pivot's field names; if included, the report compares between date ranges. In a cohort request, this `dateRanges` must be unspecified.
    #[serde(rename="dateRanges")]
    pub date_ranges: Option<Vec<DateRange>>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter.
    #[serde(rename="dimensionFilter")]
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions requested. All defined dimensions must be used by one of the following: dimension_expression, dimension_filter, pivots, order_bys.
    pub dimensions: Option<Vec<Dimension>>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter.
    #[serde(rename="keepEmptyRows")]
    pub keep_empty_rows: Option<bool>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter.
    #[serde(rename="metricFilter")]
    pub metric_filter: Option<FilterExpression>,
    /// The metrics requested, at least one metric needs to be specified. All defined metrics must be used by one of the following: metric_expression, metric_filter, order_bys.
    pub metrics: Option<Vec<Metric>>,
    /// Describes the visual format of the report's dimensions in columns or rows. The union of the fieldNames (dimension names) in all pivots must be a subset of dimension names defined in Dimensions. No two pivots can share a dimension. A dimension is only visible if it appears in a pivot.
    pub pivots: Option<Vec<Pivot>>,
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    pub property: Option<String>,
    /// Toggles whether to return the current state of this Analytics Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[serde(rename="returnPropertyQuota")]
    pub return_property_quota: Option<bool>,
}

impl client::RequestValue for RunPivotReportRequest {}


/// The response pivot report table corresponding to a pivot request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run pivot report properties](PropertyRunPivotReportCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunPivotReportResponse {
    /// Aggregation of metric values. Can be totals, minimums, or maximums. The returned aggregations are controlled by the metric_aggregations in the pivot. The type of aggregation returned in each row is shown by the dimension_values which are set to "RESERVED_".
    pub aggregates: Option<Vec<Row>>,
    /// Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows.
    #[serde(rename="dimensionHeaders")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#runPivotReport". Useful to distinguish between response types in JSON.
    pub kind: Option<String>,
    /// Metadata for the report.
    pub metadata: Option<ResponseMetaData>,
    /// Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows.
    #[serde(rename="metricHeaders")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    /// Summarizes the columns and rows created by a pivot. Each pivot in the request produces one header in the response. If we have a request like this: "pivots": [{ "fieldNames": ["country", "city"] }, { "fieldNames": "eventName" }] We will have the following `pivotHeaders` in the response: "pivotHeaders" : [{ "dimensionHeaders": [{ "dimensionValues": [ { "value": "United Kingdom" }, { "value": "London" } ] }, { "dimensionValues": [ { "value": "Japan" }, { "value": "Osaka" } ] }] }, { "dimensionHeaders": [{ "dimensionValues": [{ "value": "session_start" }] }, { "dimensionValues": [{ "value": "scroll" }] }] }]
    #[serde(rename="pivotHeaders")]
    pub pivot_headers: Option<Vec<PivotHeader>>,
    /// This Analytics Property's quota state including this request.
    #[serde(rename="propertyQuota")]
    pub property_quota: Option<PropertyQuota>,
    /// Rows of dimension value combinations and metric values in the report.
    pub rows: Option<Vec<Row>>,
}

impl client::ResponseResult for RunPivotReportResponse {}


/// The request to generate a realtime report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run realtime report properties](PropertyRunRealtimeReportCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunRealtimeReportRequest {
    /// The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter.
    #[serde(rename="dimensionFilter")]
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions requested and displayed.
    pub dimensions: Option<Vec<Dimension>>,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. `limit` must be positive. The API can also return fewer rows than the requested `limit`, if there aren't as many dimension values as the `limit`. For instance, there are fewer than 300 possible values for the dimension `country`, so when reporting on only `country`, you can't get more than 300 rows, even if you set `limit` to a higher value.
    pub limit: Option<String>,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[serde(rename="metricAggregations")]
    pub metric_aggregations: Option<Vec<String>>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter.
    #[serde(rename="metricFilter")]
    pub metric_filter: Option<FilterExpression>,
    /// The metrics requested and displayed.
    pub metrics: Option<Vec<Metric>>,
    /// Specifies how rows are ordered in the response.
    #[serde(rename="orderBys")]
    pub order_bys: Option<Vec<OrderBy>>,
    /// Toggles whether to return the current state of this Analytics Property's Realtime quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[serde(rename="returnPropertyQuota")]
    pub return_property_quota: Option<bool>,
}

impl client::RequestValue for RunRealtimeReportRequest {}


/// The response realtime report table corresponding to a request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run realtime report properties](PropertyRunRealtimeReportCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunRealtimeReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows.
    #[serde(rename="dimensionHeaders")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#runRealtimeReport". Useful to distinguish between response types in JSON.
    pub kind: Option<String>,
    /// If requested, the maximum values of metrics.
    pub maximums: Option<Vec<Row>>,
    /// Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows.
    #[serde(rename="metricHeaders")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    /// If requested, the minimum values of metrics.
    pub minimums: Option<Vec<Row>>,
    /// This Analytics Property's Realtime quota state including this request.
    #[serde(rename="propertyQuota")]
    pub property_quota: Option<PropertyQuota>,
    /// The total number of rows in the query result. `rowCount` is independent of the number of rows returned in the response and the `limit` request parameter. For example if a query returns 175 rows and includes `limit` of 50 in the API request, the response will contain `rowCount` of 175 but only 50 rows.
    #[serde(rename="rowCount")]
    pub row_count: Option<i32>,
    /// Rows of dimension value combinations and metric values in the report.
    pub rows: Option<Vec<Row>>,
    /// If requested, the totaled values of metrics.
    pub totals: Option<Vec<Row>>,
}

impl client::ResponseResult for RunRealtimeReportResponse {}


/// The request to generate a report.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run report properties](PropertyRunReportCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunReportRequest {
    /// Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present.
    #[serde(rename="cohortSpec")]
    pub cohort_spec: Option<CohortSpec>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY". If the field is empty, the report uses the property's default currency.
    #[serde(rename="currencyCode")]
    pub currency_code: Option<String>,
    /// Date ranges of data to read. If multiple date ranges are requested, each response row will contain a zero based date range index. If two date ranges overlap, the event data for the overlapping days is included in the response rows for both date ranges. In a cohort request, this `dateRanges` must be unspecified.
    #[serde(rename="dateRanges")]
    pub date_ranges: Option<Vec<DateRange>>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter.
    #[serde(rename="dimensionFilter")]
    pub dimension_filter: Option<FilterExpression>,
    /// The dimensions requested and displayed.
    pub dimensions: Option<Vec<Dimension>>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter.
    #[serde(rename="keepEmptyRows")]
    pub keep_empty_rows: Option<bool>,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. `limit` must be positive. The API can also return fewer rows than the requested `limit`, if there aren't as many dimension values as the `limit`. For instance, there are fewer than 300 possible values for the dimension `country`, so when reporting on only `country`, you can't get more than 300 rows, even if you set `limit` to a higher value. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination).
    pub limit: Option<String>,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[serde(rename="metricAggregations")]
    pub metric_aggregations: Option<Vec<String>>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter.
    #[serde(rename="metricFilter")]
    pub metric_filter: Option<FilterExpression>,
    /// The metrics requested and displayed.
    pub metrics: Option<Vec<Metric>>,
    /// The row count of the start row. The first row is counted as row 0. When paging, the first request does not specify offset; or equivalently, sets offset to 0; the first request returns the first `limit` of rows. The second request sets offset to the `limit` of the first request; the second request returns the second `limit` of rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination).
    pub offset: Option<String>,
    /// Specifies how rows are ordered in the response.
    #[serde(rename="orderBys")]
    pub order_bys: Option<Vec<OrderBy>>,
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    pub property: Option<String>,
    /// Toggles whether to return the current state of this Analytics Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[serde(rename="returnPropertyQuota")]
    pub return_property_quota: Option<bool>,
}

impl client::RequestValue for RunReportRequest {}


/// The response report table corresponding to a request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [run report properties](PropertyRunReportCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows.
    #[serde(rename="dimensionHeaders")]
    pub dimension_headers: Option<Vec<DimensionHeader>>,
    /// Identifies what kind of resource this message is. This `kind` is always the fixed string "analyticsData#runReport". Useful to distinguish between response types in JSON.
    pub kind: Option<String>,
    /// If requested, the maximum values of metrics.
    pub maximums: Option<Vec<Row>>,
    /// Metadata for the report.
    pub metadata: Option<ResponseMetaData>,
    /// Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows.
    #[serde(rename="metricHeaders")]
    pub metric_headers: Option<Vec<MetricHeader>>,
    /// If requested, the minimum values of metrics.
    pub minimums: Option<Vec<Row>>,
    /// This Analytics Property's quota state including this request.
    #[serde(rename="propertyQuota")]
    pub property_quota: Option<PropertyQuota>,
    /// The total number of rows in the query result. `rowCount` is independent of the number of rows returned in the response, the `limit` request parameter, and the `offset` request parameter. For example if a query returns 175 rows and includes `limit` of 50 in the API request, the response will contain `rowCount` of 175 but only 50 rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination).
    #[serde(rename="rowCount")]
    pub row_count: Option<i32>,
    /// Rows of dimension value combinations and metric values in the report.
    pub rows: Option<Vec<Row>>,
    /// If requested, the totaled values of metrics.
    pub totals: Option<Vec<Row>>,
}

impl client::ResponseResult for RunReportResponse {}


/// The filter for string
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringFilter {
    /// If true, the string value is case sensitive.
    #[serde(rename="caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// The match type for this filter.
    #[serde(rename="matchType")]
    pub match_type: Option<String>,
    /// The string value used for the matching.
    pub value: Option<String>,
}

impl client::Part for StringFilter {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *property* resources.
/// It is not used directly, but through the `AnalyticsData` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use analyticsdata1_beta::AnalyticsData;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_run_pivot_reports(...)`, `batch_run_reports(...)`, `get_metadata(...)`, `run_pivot_report(...)`, `run_realtime_report(...)` and `run_report(...)`
/// // to build up your call.
/// let rb = hub.properties();
/// # }
/// ```
pub struct PropertyMethods<'a>
    where  {

    hub: &'a AnalyticsData<>,
}

impl<'a> client::MethodsBuilder for PropertyMethods<'a> {}

impl<'a> PropertyMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns multiple pivot reports in a batch. All reports must be for the same GA4 Property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunPivotReportRequest may either be unspecified or consistent with this property. Example: properties/1234
    pub fn batch_run_pivot_reports(&self, request: BatchRunPivotReportsRequest, property: &str) -> PropertyBatchRunPivotReportCall<'a> {
        PropertyBatchRunPivotReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns multiple reports in a batch. All reports must be for the same GA4 Property.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunReportRequest may either be unspecified or consistent with this property. Example: properties/1234
    pub fn batch_run_reports(&self, request: BatchRunReportsRequest, property: &str) -> PropertyBatchRunReportCall<'a> {
        PropertyBatchRunReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for dimensions and metrics available in reporting methods. Used to explore the dimensions and metrics. In this method, a Google Analytics GA4 Property Identifier is specified in the request, and the metadata response includes Custom dimensions and metrics as well as Universal metadata. For example if a custom metric with parameter name `levels_unlocked` is registered to a property, the Metadata response will contain `customEvent:levels_unlocked`. Universal metadata are dimensions and metrics applicable to any property such as `country` and `totalUsers`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the metadata to retrieve. This name field is specified in the URL path and not URL parameters. Property is a numeric Google Analytics GA4 Property identifier. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234/metadata Set the Property ID to 0 for dimensions and metrics common to all properties. In this special mode, this method will not return custom dimensions and metrics.
    pub fn get_metadata(&self, name: &str) -> PropertyGetMetadataCall<'a> {
        PropertyGetMetadataCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a customized pivot report of your Google Analytics event data. Pivot reports are more advanced and expressive formats than regular reports. In a pivot report, dimensions are only visible if they are included in a pivot. Multiple pivots can be specified to further dissect your data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    pub fn run_pivot_report(&self, request: RunPivotReportRequest, property: &str) -> PropertyRunPivotReportCall<'a> {
        PropertyRunPivotReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The Google Analytics Realtime API returns a customized report of realtime event data for your property. These reports show events and usage from the last 30 minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234
    pub fn run_realtime_report(&self, request: RunRealtimeReportRequest, property: &str) -> PropertyRunRealtimeReportCall<'a> {
        PropertyRunRealtimeReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a customized report of your Google Analytics event data. Reports contain statistics derived from data collected by the Google Analytics tracking code. The data returned from the API is as a table with columns for the requested dimensions and metrics. Metrics are individual measurements of user activity on your property, such as active users or event count. Dimensions break down metrics across some common criteria, such as country or event name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `property` - A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    pub fn run_report(&self, request: RunReportRequest, property: &str) -> PropertyRunReportCall<'a> {
        PropertyRunReportCall {
            hub: self.hub,
            _request: request,
            _property: property.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns multiple pivot reports in a batch. All reports must be for the same GA4 Property.
///
/// A builder for the *batchRunPivotReports* method supported by a *property* resource.
/// It is not used directly, but through a `PropertyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// use analyticsdata1_beta::api::BatchRunPivotReportsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use analyticsdata1_beta::AnalyticsData;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchRunPivotReportsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().batch_run_pivot_reports(req, "property")
///              .doit().await;
/// # }
/// ```
pub struct PropertyBatchRunPivotReportCall<'a>
    where  {

    hub: &'a AnalyticsData<>,
    _request: BatchRunPivotReportsRequest,
    _property: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PropertyBatchRunPivotReportCall<'a> {}

impl<'a> PropertyBatchRunPivotReportCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchRunPivotReportsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "analyticsdata.properties.batchRunPivotReports",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("property", self._property.to_string()));
        for &field in ["alt", "property"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+property}:batchRunPivotReports";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+property}", "property")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["property"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchRunPivotReportsRequest) -> PropertyBatchRunPivotReportCall<'a> {
        self._request = new_value;
        self
    }
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunPivotReportRequest may either be unspecified or consistent with this property. Example: properties/1234
    ///
    /// Sets the *property* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn property(mut self, new_value: &str) -> PropertyBatchRunPivotReportCall<'a> {
        self._property = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PropertyBatchRunPivotReportCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PropertyBatchRunPivotReportCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PropertyBatchRunPivotReportCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns multiple reports in a batch. All reports must be for the same GA4 Property.
///
/// A builder for the *batchRunReports* method supported by a *property* resource.
/// It is not used directly, but through a `PropertyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// use analyticsdata1_beta::api::BatchRunReportsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use analyticsdata1_beta::AnalyticsData;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchRunReportsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().batch_run_reports(req, "property")
///              .doit().await;
/// # }
/// ```
pub struct PropertyBatchRunReportCall<'a>
    where  {

    hub: &'a AnalyticsData<>,
    _request: BatchRunReportsRequest,
    _property: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PropertyBatchRunReportCall<'a> {}

impl<'a> PropertyBatchRunReportCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchRunReportsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "analyticsdata.properties.batchRunReports",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("property", self._property.to_string()));
        for &field in ["alt", "property"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+property}:batchRunReports";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+property}", "property")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["property"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchRunReportsRequest) -> PropertyBatchRunReportCall<'a> {
        self._request = new_value;
        self
    }
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). This property must be specified for the batch. The property within RunReportRequest may either be unspecified or consistent with this property. Example: properties/1234
    ///
    /// Sets the *property* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn property(mut self, new_value: &str) -> PropertyBatchRunReportCall<'a> {
        self._property = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PropertyBatchRunReportCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PropertyBatchRunReportCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PropertyBatchRunReportCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns metadata for dimensions and metrics available in reporting methods. Used to explore the dimensions and metrics. In this method, a Google Analytics GA4 Property Identifier is specified in the request, and the metadata response includes Custom dimensions and metrics as well as Universal metadata. For example if a custom metric with parameter name `levels_unlocked` is registered to a property, the Metadata response will contain `customEvent:levels_unlocked`. Universal metadata are dimensions and metrics applicable to any property such as `country` and `totalUsers`.
///
/// A builder for the *getMetadata* method supported by a *property* resource.
/// It is not used directly, but through a `PropertyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use analyticsdata1_beta::AnalyticsData;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().get_metadata("name")
///              .doit().await;
/// # }
/// ```
pub struct PropertyGetMetadataCall<'a>
    where  {

    hub: &'a AnalyticsData<>,
    _name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PropertyGetMetadataCall<'a> {}

impl<'a> PropertyGetMetadataCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Metadata)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "analyticsdata.properties.getMetadata",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::AnalyticReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. The resource name of the metadata to retrieve. This name field is specified in the URL path and not URL parameters. Property is a numeric Google Analytics GA4 Property identifier. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234/metadata Set the Property ID to 0 for dimensions and metrics common to all properties. In this special mode, this method will not return custom dimensions and metrics.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> PropertyGetMetadataCall<'a> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PropertyGetMetadataCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PropertyGetMetadataCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::AnalyticReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PropertyGetMetadataCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a customized pivot report of your Google Analytics event data. Pivot reports are more advanced and expressive formats than regular reports. In a pivot report, dimensions are only visible if they are included in a pivot. Multiple pivots can be specified to further dissect your data.
///
/// A builder for the *runPivotReport* method supported by a *property* resource.
/// It is not used directly, but through a `PropertyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// use analyticsdata1_beta::api::RunPivotReportRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use analyticsdata1_beta::AnalyticsData;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RunPivotReportRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().run_pivot_report(req, "property")
///              .doit().await;
/// # }
/// ```
pub struct PropertyRunPivotReportCall<'a>
    where  {

    hub: &'a AnalyticsData<>,
    _request: RunPivotReportRequest,
    _property: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PropertyRunPivotReportCall<'a> {}

impl<'a> PropertyRunPivotReportCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, RunPivotReportResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "analyticsdata.properties.runPivotReport",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("property", self._property.to_string()));
        for &field in ["alt", "property"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+property}:runPivotReport";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+property}", "property")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["property"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: RunPivotReportRequest) -> PropertyRunPivotReportCall<'a> {
        self._request = new_value;
        self
    }
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    ///
    /// Sets the *property* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn property(mut self, new_value: &str) -> PropertyRunPivotReportCall<'a> {
        self._property = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PropertyRunPivotReportCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PropertyRunPivotReportCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PropertyRunPivotReportCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// The Google Analytics Realtime API returns a customized report of realtime event data for your property. These reports show events and usage from the last 30 minutes.
///
/// A builder for the *runRealtimeReport* method supported by a *property* resource.
/// It is not used directly, but through a `PropertyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// use analyticsdata1_beta::api::RunRealtimeReportRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use analyticsdata1_beta::AnalyticsData;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RunRealtimeReportRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().run_realtime_report(req, "property")
///              .doit().await;
/// # }
/// ```
pub struct PropertyRunRealtimeReportCall<'a>
    where  {

    hub: &'a AnalyticsData<>,
    _request: RunRealtimeReportRequest,
    _property: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PropertyRunRealtimeReportCall<'a> {}

impl<'a> PropertyRunRealtimeReportCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, RunRealtimeReportResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "analyticsdata.properties.runRealtimeReport",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("property", self._property.to_string()));
        for &field in ["alt", "property"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+property}:runRealtimeReport";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+property}", "property")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["property"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: RunRealtimeReportRequest) -> PropertyRunRealtimeReportCall<'a> {
        self._request = new_value;
        self
    }
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Example: properties/1234
    ///
    /// Sets the *property* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn property(mut self, new_value: &str) -> PropertyRunRealtimeReportCall<'a> {
        self._property = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PropertyRunRealtimeReportCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PropertyRunRealtimeReportCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PropertyRunRealtimeReportCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a customized report of your Google Analytics event data. Reports contain statistics derived from data collected by the Google Analytics tracking code. The data returned from the API is as a table with columns for the requested dimensions and metrics. Metrics are individual measurements of user activity on your property, such as active users or event count. Dimensions break down metrics across some common criteria, such as country or event name.
///
/// A builder for the *runReport* method supported by a *property* resource.
/// It is not used directly, but through a `PropertyMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_analyticsdata1_beta as analyticsdata1_beta;
/// use analyticsdata1_beta::api::RunReportRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use analyticsdata1_beta::AnalyticsData;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = AnalyticsData::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = RunReportRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.properties().run_report(req, "property")
///              .doit().await;
/// # }
/// ```
pub struct PropertyRunReportCall<'a>
    where  {

    hub: &'a AnalyticsData<>,
    _request: RunReportRequest,
    _property: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for PropertyRunReportCall<'a> {}

impl<'a> PropertyRunReportCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, RunReportResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "analyticsdata.properties.runReport",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("property", self._property.to_string()));
        for &field in ["alt", "property"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta/{+property}:runReport";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Analytic.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+property}", "property")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["property"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: RunReportRequest) -> PropertyRunReportCall<'a> {
        self._request = new_value;
        self
    }
    /// A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234
    ///
    /// Sets the *property* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn property(mut self, new_value: &str) -> PropertyRunReportCall<'a> {
        self._property = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PropertyRunReportCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> PropertyRunReportCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Analytic`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> PropertyRunReportCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


