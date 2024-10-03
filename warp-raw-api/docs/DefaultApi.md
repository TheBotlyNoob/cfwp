# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account**](DefaultApi.md#get_account) | **GET** /{apiVersion}/reg/{sourceDeviceId}/account | GetAccount
[**get_bound_devices**](DefaultApi.md#get_bound_devices) | **GET** /{apiVersion}/reg/{sourceDeviceId}/account/devices | GetBoundDevices
[**get_client_config**](DefaultApi.md#get_client_config) | **GET** /{apiVersion}/client_config | GetClientConfig
[**get_source_device**](DefaultApi.md#get_source_device) | **GET** /{apiVersion}/reg/{sourceDeviceId} | GetSourceDevice
[**register**](DefaultApi.md#register) | **POST** /{apiVersion}/reg | Register
[**reset_account_license**](DefaultApi.md#reset_account_license) | **POST** /{apiVersion}/reg/{sourceDeviceId}/account/license | ResetAccountLicense
[**update_account**](DefaultApi.md#update_account) | **PUT** /{apiVersion}/reg/{sourceDeviceId}/account | UpdateAccount
[**update_bound_device**](DefaultApi.md#update_bound_device) | **PATCH** /{apiVersion}/reg/{sourceDeviceId}/account/reg/{boundDeviceId} | UpdateBoundDevice
[**update_source_device**](DefaultApi.md#update_source_device) | **PATCH** /{apiVersion}/reg/{sourceDeviceId} | UpdateSourceDevice



## get_account

> models::GetAccount200Response get_account(source_device_id, api_version)
GetAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_device_id** | **String** |  | [required] |
**api_version** | **String** |  | [required] |

### Return type

[**models::GetAccount200Response**](GetAccount_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bound_devices

> Vec<models::GetBoundDevices200Response> get_bound_devices(source_device_id, api_version)
GetBoundDevices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_device_id** | **String** |  | [required] |
**api_version** | **String** |  | [required] |

### Return type

[**Vec<models::GetBoundDevices200Response>**](GetBoundDevices_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config

> models::GetClientConfig200Response get_client_config(api_version)
GetClientConfig

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** |  | [required] |

### Return type

[**models::GetClientConfig200Response**](GetClientConfig_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source_device

> models::GetSourceDevice200Response get_source_device(api_version, source_device_id)
GetSourceDevice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** |  | [required] |
**source_device_id** | **String** |  | [required] |

### Return type

[**models::GetSourceDevice200Response**](GetSourceDevice_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::Register200Response register(api_version, register_request)
Register

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** |  | [required] |
**register_request** | Option<[**RegisterRequest**](RegisterRequest.md)> |  |  |

### Return type

[**models::Register200Response**](Register_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_account_license

> models::ResetAccountLicense200Response reset_account_license(source_device_id, api_version)
ResetAccountLicense

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_device_id** | **String** |  | [required] |
**api_version** | **String** |  | [required] |

### Return type

[**models::ResetAccountLicense200Response**](ResetAccountLicense_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account

> models::UpdateAccount200Response update_account(source_device_id, api_version, update_account_request)
UpdateAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_device_id** | **String** |  | [required] |
**api_version** | **String** |  | [required] |
**update_account_request** | Option<[**UpdateAccountRequest**](UpdateAccountRequest.md)> |  |  |

### Return type

[**models::UpdateAccount200Response**](UpdateAccount_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bound_device

> Vec<models::UpdateBoundDevice200Response> update_bound_device(source_device_id, api_version, bound_device_id, update_bound_device_request)
UpdateBoundDevice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_device_id** | **String** |  | [required] |
**api_version** | **String** |  | [required] |
**bound_device_id** | **String** |  | [required] |
**update_bound_device_request** | Option<[**UpdateBoundDeviceRequest**](UpdateBoundDeviceRequest.md)> |  |  |

### Return type

[**Vec<models::UpdateBoundDevice200Response>**](UpdateBoundDevice_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_source_device

> models::UpdateSourceDevice200Response update_source_device(api_version, source_device_id, update_source_device_request)
UpdateSourceDevice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** |  | [required] |
**source_device_id** | **String** |  | [required] |
**update_source_device_request** | Option<[**UpdateSourceDeviceRequest**](UpdateSourceDeviceRequest.md)> |  |  |

### Return type

[**models::UpdateSourceDevice200Response**](UpdateSourceDevice_200_Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

