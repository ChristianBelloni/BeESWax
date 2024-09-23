// This file is @generated by prost-build.
/// Region Identifier.
///
/// Identifies a region to look up.
///
/// One of place or unit_code must be specified. If none is specified,
/// an INVALID_ARGUMENT error is returned. region_code must also be specified
/// except when place_type is "country".
///
/// place and unit_code specify a location to match a Place ID to. For
/// example if place is "California" and region_code "US" the API
/// returns the following matched_place_id results when the following
/// place_types are specified:
///
/// place_type:                   matched_place_id results:
/// administrative_area_level_1   Place ID for The State of California
/// (All other supported types)   No Match
///
/// If unit_code is "6" (FIPs code for California) and region_code is "US
/// the API returns the following matched_place_id results when the
/// following place_types are specified:
///
/// place type:                   matched_place_id results:
/// administrative_area_level_1   Place ID for The State of California
/// (All other supported types)   No Match
///
/// or if unit_code is "US" the API returns the following results when
/// the following place_types are specified:
///
/// place type:                   matched_place_id results:
/// country                       Place ID for the United States
/// (All other supported types)   No Match
///
/// If no match is found, matched_place_id is not set.
///
/// Candidate Place IDs are returned when a lookup finds a region with a
/// different place_type then the one requested. For example if place is
/// "California" and place_type is "country" the Place ID for The State of
/// California is returned as a candidate in the candidate_place_ids field.
///
/// Next available tag: 10
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionIdentifier {
    /// Required. Place type to match.
    #[prost(enumeration = "region_identifier::PlaceType", tag = "6")]
    pub place_type: i32,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn", corresponding to
    /// the language in which the place name and address is requested. If none is
    /// requested, then it defaults to English.
    #[prost(string, tag = "7")]
    pub language_code: ::prost::alloc::string::String,
    /// Two-letter ISO-3166 country/region code for the location you're trying to
    /// match. region_code is optional if place_type is "country".
    #[prost(string, tag = "8")]
    pub region_code: ::prost::alloc::string::String,
    /// The location must be specified by one of the following:
    #[prost(oneof = "region_identifier::Location", tags = "4, 5")]
    pub location: ::core::option::Option<region_identifier::Location>,
}
/// Nested message and enum types in `RegionIdentifier`.
pub mod region_identifier {
    /// Possible place types to match to.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PlaceType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Postal code.
        PostalCode = 1,
        /// Administrative area level 1 (State in the US).
        AdministrativeAreaLevel1 = 2,
        /// Administrative area level 2 (County in the US).
        AdministrativeAreaLevel2 = 3,
        /// Locality (City).
        Locality = 4,
        /// Neighborhood.
        Neighborhood = 5,
        /// Country.
        Country = 6,
        /// Sublocality.
        Sublocality = 7,
        /// Administrative area level 3.
        AdministrativeAreaLevel3 = 8,
        /// Administrative area level 4.
        AdministrativeAreaLevel4 = 9,
        /// School district.
        SchoolDistrict = 10,
    }
    impl PlaceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "PLACE_TYPE_UNSPECIFIED",
                Self::PostalCode => "POSTAL_CODE",
                Self::AdministrativeAreaLevel1 => "ADMINISTRATIVE_AREA_LEVEL_1",
                Self::AdministrativeAreaLevel2 => "ADMINISTRATIVE_AREA_LEVEL_2",
                Self::Locality => "LOCALITY",
                Self::Neighborhood => "NEIGHBORHOOD",
                Self::Country => "COUNTRY",
                Self::Sublocality => "SUBLOCALITY",
                Self::AdministrativeAreaLevel3 => "ADMINISTRATIVE_AREA_LEVEL_3",
                Self::AdministrativeAreaLevel4 => "ADMINISTRATIVE_AREA_LEVEL_4",
                Self::SchoolDistrict => "SCHOOL_DISTRICT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PLACE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "POSTAL_CODE" => Some(Self::PostalCode),
                "ADMINISTRATIVE_AREA_LEVEL_1" => Some(Self::AdministrativeAreaLevel1),
                "ADMINISTRATIVE_AREA_LEVEL_2" => Some(Self::AdministrativeAreaLevel2),
                "LOCALITY" => Some(Self::Locality),
                "NEIGHBORHOOD" => Some(Self::Neighborhood),
                "COUNTRY" => Some(Self::Country),
                "SUBLOCALITY" => Some(Self::Sublocality),
                "ADMINISTRATIVE_AREA_LEVEL_3" => Some(Self::AdministrativeAreaLevel3),
                "ADMINISTRATIVE_AREA_LEVEL_4" => Some(Self::AdministrativeAreaLevel4),
                "SCHOOL_DISTRICT" => Some(Self::SchoolDistrict),
                _ => None,
            }
        }
    }
    /// The location must be specified by one of the following:
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        /// The name of the region to match to a Place ID.
        ///
        /// The place field is used in combination with place_type to look up
        /// the region Place ID.
        ///
        /// For example:
        /// If place_type is "locality", a valid place can be "Palo Alto, CA".
        ///
        /// If place_type is "postal_code", a valid place can be "94109".
        ///
        /// If place_type is "country", a valid place can be "United States".
        /// etc.
        ///
        /// region_code is required when place is specified except when
        /// place_type is "country".
        #[prost(string, tag = "4")]
        Place(::prost::alloc::string::String),
        /// The FIPs state or county codes (US only) or ISO-3166-1 country code to be
        /// matched.
        ///
        /// The unit_code field is used in combination with place_type to look up
        /// the region Place ID.
        ///
        /// For example:
        /// If place_type is "country", a valid unit_code can be "US" (ISO-3166-1
        /// Alpha-2 code for United States) or "BR" (ISO-3166-1 Alpha-2 code for
        /// Brazil).
        ///
        /// If place_type is "administrative_area_level_1" (state) and region_code is
        /// "US", a valid unit_code can be "6" (FIPs code for California) or
        /// "12"(FIPs code for Florida).
        ///
        /// If place_type is "administrative_area_level_2" (county) and region_code
        /// is "US", a valid unit_code can be "6001" (FIPs code for Alameda County in
        /// California) or "12086" (FIPs code for Miami-Dade County in Florida).
        ///
        /// region_code is required when specifying a FIPs code. region_code is
        /// ignored for ISO-3166-1 country codes.
        #[prost(string, tag = "5")]
        UnitCode(::prost::alloc::string::String),
    }
}
/// Region Match.
///
/// Next available tag: 5
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionMatch {
    /// Place ID of the region that is matched. If region is found, this field is
    /// not set.
    #[prost(string, tag = "1")]
    pub matched_place_id: ::prost::alloc::string::String,
    /// Region candidate IDs. Up to three candidates may be returned.
    #[prost(string, repeated, tag = "2")]
    pub candidate_place_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Matching debug information for when no match is found.
    #[prost(string, tag = "3")]
    pub debug_info: ::prost::alloc::string::String,
}
/// Region Search Values.
///
/// Desired search values of a single region.
///
/// Location must be specified by one of the following: address, latlng or
/// place_id. If none is specified, an INVALID_ARGUMENT error is returned.
/// region_code must also be provided when address is specified.
///
/// The fields address, latlng and place_id specify a location contained inside
/// the region to match. For example if address is "1600 Amphitheatre Pkwy,
/// Mountain View, CA 94043" the API returns the following matched_place_id
/// results when the following place_types are specified:
///
/// place_type:                   matched_place_id results:
/// postal_code                   Place ID for "94043"
/// administrative_area_level_1   Place ID for The State of California
/// administrative_area_level_2   Place ID for Santa Clara County
/// etc.
///
/// More Examples:
///
/// If latlng is "latitude: 37.4220656 longitude: -122.0862784" and place_type
/// is "locality", the result contains the Place ID (of type "locality") for
/// that location (the Place ID of Mountain View, CA, in this case).
///
/// If place_id is "ChIJj61dQgK6j4AR4GeTYWZsKWw" (Place ID for Google office in
/// Mountain view, CA) and place_type is "locality", the result contains the
/// Place ID (of type "locality") for that location (the Place ID of Mountain
/// View, CA, in this case).
///
/// If no match is found, matched_place_id is not set.
///
/// Candidates Place IDs are returned when a search finds multiple Place
/// IDs for the location specified. For example if the API is searching for
/// region Place IDs of type neighboorhood for a location that is contained
/// within multiple neighboords. The Place Ids will be returned as candidates in
/// the candidate_place_ids field.
///
/// Next available tag: 10
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSearchValue {
    /// Required. The type of the place to match.
    #[prost(enumeration = "region_search_value::PlaceType", tag = "6")]
    pub place_type: i32,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn", corresponding to
    /// the language in which the place name and address is requested. If none is
    /// requested, then it defaults to English.
    #[prost(string, tag = "7")]
    pub language_code: ::prost::alloc::string::String,
    /// Two-letter ISO-3166 country/region code for the location you're trying to
    /// match. region_code is required when address is specified.
    #[prost(string, tag = "8")]
    pub region_code: ::prost::alloc::string::String,
    /// The location must be specified by one of the following:
    #[prost(oneof = "region_search_value::Location", tags = "1, 2, 3")]
    pub location: ::core::option::Option<region_search_value::Location>,
}
/// Nested message and enum types in `RegionSearchValue`.
pub mod region_search_value {
    /// Possible place types to match to.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PlaceType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Postal code.
        PostalCode = 1,
        /// Administrative area level 1 (State in the US).
        AdministrativeAreaLevel1 = 2,
        /// Administrative area level 2 (County in the US).
        AdministrativeAreaLevel2 = 3,
        /// Locality (City).
        Locality = 4,
        /// Neighborhood.
        Neighborhood = 5,
        /// Country.
        Country = 6,
        /// Sublocality.
        Sublocality = 7,
        /// Administrative area level 3.
        AdministrativeAreaLevel3 = 8,
        /// Administrative area level 4.
        AdministrativeAreaLevel4 = 9,
        /// School district.
        SchoolDistrict = 10,
    }
    impl PlaceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "PLACE_TYPE_UNSPECIFIED",
                Self::PostalCode => "POSTAL_CODE",
                Self::AdministrativeAreaLevel1 => "ADMINISTRATIVE_AREA_LEVEL_1",
                Self::AdministrativeAreaLevel2 => "ADMINISTRATIVE_AREA_LEVEL_2",
                Self::Locality => "LOCALITY",
                Self::Neighborhood => "NEIGHBORHOOD",
                Self::Country => "COUNTRY",
                Self::Sublocality => "SUBLOCALITY",
                Self::AdministrativeAreaLevel3 => "ADMINISTRATIVE_AREA_LEVEL_3",
                Self::AdministrativeAreaLevel4 => "ADMINISTRATIVE_AREA_LEVEL_4",
                Self::SchoolDistrict => "SCHOOL_DISTRICT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PLACE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "POSTAL_CODE" => Some(Self::PostalCode),
                "ADMINISTRATIVE_AREA_LEVEL_1" => Some(Self::AdministrativeAreaLevel1),
                "ADMINISTRATIVE_AREA_LEVEL_2" => Some(Self::AdministrativeAreaLevel2),
                "LOCALITY" => Some(Self::Locality),
                "NEIGHBORHOOD" => Some(Self::Neighborhood),
                "COUNTRY" => Some(Self::Country),
                "SUBLOCALITY" => Some(Self::Sublocality),
                "ADMINISTRATIVE_AREA_LEVEL_3" => Some(Self::AdministrativeAreaLevel3),
                "ADMINISTRATIVE_AREA_LEVEL_4" => Some(Self::AdministrativeAreaLevel4),
                "SCHOOL_DISTRICT" => Some(Self::SchoolDistrict),
                _ => None,
            }
        }
    }
    /// The location must be specified by one of the following:
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        /// The unstructured street address that is contained inside a region to
        /// match. region_code is required when address is specified.
        #[prost(string, tag = "1")]
        Address(::prost::alloc::string::String),
        /// The latitude and longitude that is contained inside a region to match.
        #[prost(message, tag = "2")]
        Latlng(super::super::super::super::r#type::LatLng),
        /// The Place ID that is contained inside a region to match.
        #[prost(string, tag = "3")]
        PlaceId(::prost::alloc::string::String),
    }
}
/// Lookup Region Request.
///
/// Next available tag: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupRegionRequest {
    /// Each `RegionIdentifier` represents the desired fields used to lookup a
    /// single region. See `RegionIdentifier` proto for more details and examples.
    #[prost(message, repeated, tag = "1")]
    pub identifiers: ::prost::alloc::vec::Vec<RegionIdentifier>,
    /// The maximum number of matches to return. The service may return fewer than
    /// this value.
    ///
    /// If unspecified, at most 50 matches will be returned. The maximum value is
    /// 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `LookupRegion` call. Provide this to
    /// retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `LookupRegion` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Lookup Region Response.
///
/// Next available tag: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupRegionResponse {
    /// Lookup region matches, one for each `RegionIdentifier` in
    /// `LookupRegionRequest.identifiers`.
    #[prost(message, repeated, tag = "1")]
    pub matches: ::prost::alloc::vec::Vec<RegionMatch>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Search Region Request.
///
/// Next available tag: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRegionRequest {
    /// Each value represents desired search values of a single region to match.
    /// The API tries to match them to Place IDs. See `RegionSearchValue`
    /// proto for more info and examples.
    #[prost(message, repeated, tag = "1")]
    pub search_values: ::prost::alloc::vec::Vec<RegionSearchValue>,
    /// The maximum number of matches to return. The service may return fewer than
    /// this value.
    ///
    /// If unspecified, at most 50 matches will be returned. The maximum value is
    /// 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `SearchRegion` call. Provide this to
    /// retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `LookupRegion` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Match Region Response.
///
/// Next available tag: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRegionResponse {
    /// Search region matches, one for each `RegionSearchValue` in
    /// `SearchRegionRequest.search_values`.
    #[prost(message, repeated, tag = "1")]
    pub matches: ::prost::alloc::vec::Vec<RegionMatch>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod region_lookup_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Region Lookup API.
    #[derive(Debug, Clone)]
    pub struct RegionLookupClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RegionLookupClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RegionLookupClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            RegionLookupClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Lookup region RPC.
        ///
        /// Looks up a set of region Place IDs of types related to geographic
        /// boundaries.
        ///
        /// The API looks up a region Place ID using the `RegionIdentifier` proto. See
        /// `RegionIdentifier` for more details and examples.
        ///
        /// The following region place types are supported for look up: postal_code,
        /// administrative_area_level_1, administrative_area_level_2, locality,
        /// neighborhood, and country.
        pub async fn lookup_region(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupRegionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LookupRegionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.regionlookup.v1alpha.RegionLookup/LookupRegion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.regionlookup.v1alpha.RegionLookup",
                        "LookupRegion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Search region RPC.
        ///
        /// Searches for a set of region Place IDs of types related to geographic
        /// boundaries.
        ///
        /// Similar to `LookupRegion` RPC but instead of looking up Place IDs for the
        /// given `RegionIdentifier`, the API searches for Region Place IDs by
        /// considering all regions that are contained within a specified location. The
        /// `RegionSearchValue` is used to specify the search values. See
        /// `RegionSearchValue` for more details and examples.
        ///
        /// The following region place types are supported for searching: postal_code,
        /// administrative_area_level_1, administrative_area_level_2, locality,
        /// neighborhood, and country.
        pub async fn search_region(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRegionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchRegionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.regionlookup.v1alpha.RegionLookup/SearchRegion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.regionlookup.v1alpha.RegionLookup",
                        "SearchRegion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod region_lookup_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RegionLookupServer.
    #[async_trait]
    pub trait RegionLookup: std::marker::Send + std::marker::Sync + 'static {
        /// Lookup region RPC.
        ///
        /// Looks up a set of region Place IDs of types related to geographic
        /// boundaries.
        ///
        /// The API looks up a region Place ID using the `RegionIdentifier` proto. See
        /// `RegionIdentifier` for more details and examples.
        ///
        /// The following region place types are supported for look up: postal_code,
        /// administrative_area_level_1, administrative_area_level_2, locality,
        /// neighborhood, and country.
        async fn lookup_region(
            &self,
            request: tonic::Request<super::LookupRegionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LookupRegionResponse>,
            tonic::Status,
        >;
        /// Search region RPC.
        ///
        /// Searches for a set of region Place IDs of types related to geographic
        /// boundaries.
        ///
        /// Similar to `LookupRegion` RPC but instead of looking up Place IDs for the
        /// given `RegionIdentifier`, the API searches for Region Place IDs by
        /// considering all regions that are contained within a specified location. The
        /// `RegionSearchValue` is used to specify the search values. See
        /// `RegionSearchValue` for more details and examples.
        ///
        /// The following region place types are supported for searching: postal_code,
        /// administrative_area_level_1, administrative_area_level_2, locality,
        /// neighborhood, and country.
        async fn search_region(
            &self,
            request: tonic::Request<super::SearchRegionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchRegionResponse>,
            tonic::Status,
        >;
    }
    /// Service definition for the Region Lookup API.
    #[derive(Debug)]
    pub struct RegionLookupServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RegionLookupServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RegionLookupServer<T>
    where
        T: RegionLookup,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/google.maps.regionlookup.v1alpha.RegionLookup/LookupRegion" => {
                    #[allow(non_camel_case_types)]
                    struct LookupRegionSvc<T: RegionLookup>(pub Arc<T>);
                    impl<
                        T: RegionLookup,
                    > tonic::server::UnaryService<super::LookupRegionRequest>
                    for LookupRegionSvc<T> {
                        type Response = super::LookupRegionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupRegionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RegionLookup>::lookup_region(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LookupRegionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.maps.regionlookup.v1alpha.RegionLookup/SearchRegion" => {
                    #[allow(non_camel_case_types)]
                    struct SearchRegionSvc<T: RegionLookup>(pub Arc<T>);
                    impl<
                        T: RegionLookup,
                    > tonic::server::UnaryService<super::SearchRegionRequest>
                    for SearchRegionSvc<T> {
                        type Response = super::SearchRegionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchRegionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RegionLookup>::search_region(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SearchRegionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T> Clone for RegionLookupServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "google.maps.regionlookup.v1alpha.RegionLookup";
    impl<T> tonic::server::NamedService for RegionLookupServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
