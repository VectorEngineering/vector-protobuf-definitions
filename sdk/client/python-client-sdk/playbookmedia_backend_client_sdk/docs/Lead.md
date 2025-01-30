# Lead


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**name** | **str** |  | [optional] 
**website** | **str** |  | [optional] 
**phone** | **str** |  | [optional] 
**address** | **str** |  | [optional] 
**city** | **str** |  | [optional] 
**state** | **str** |  | [optional] 
**country** | **str** |  | [optional] 
**latitude** | **float** |  | [optional] 
**longitude** | **float** |  | [optional] 
**google_rating** | **float** |  | [optional] 
**review_count** | **int** |  | [optional] 
**industry** | **str** |  | [optional] 
**employee_count** | **int** |  | [optional] 
**estimated_revenue** | **str** |  | [optional] 
**org_id** | **str** |  | [optional] 
**tenant_id** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**job** | [**ScrapingJob**](ScrapingJob.md) |  | [optional] 
**workspace** | [**Workspace**](Workspace.md) |  | [optional] 
**place_id** | **str** | Google Places ID | [optional] 
**google_maps_url** | **str** |  | [optional] 
**business_status** | **str** |  | [optional] 
**regular_hours** | [**List[BusinessHours]**](BusinessHours.md) |  | [optional] 
**special_hours** | [**List[BusinessHours]**](BusinessHours.md) |  | [optional] 
**photo_references** | **List[str]** | Google Places photo IDs | [optional] 
**main_photo_url** | **str** |  | [optional] 
**reviews** | [**List[Review]**](Review.md) |  | [optional] 
**types** | **List[str]** | Google Places types | [optional] 
**amenities** | **List[str]** |  | [optional] 
**serves_vegetarian_food** | **bool** |  | [optional] 
**outdoor_seating** | **bool** |  | [optional] 
**payment_methods** | **List[str]** | visa, amex, cash, etc | [optional] 
**wheelchair_accessible** | **bool** |  | [optional] 
**parking_available** | **bool** |  | [optional] 
**social_media** | **Dict[str, str]** | platform -&gt; url | [optional] 
**rating_category** | **str** |  | [optional] 
**rating** | **float** |  | [optional] 
**count** | **int** |  | [optional] 
**last_updated** | **datetime** |  | [optional] 
**data_source_version** | **str** |  | [optional] 
**scraping_session_id** | **str** |  | [optional] 
**alternate_phones** | **List[str]** |  | [optional] 
**contact_person_name** | **str** |  | [optional] 
**contact_person_title** | **str** |  | [optional] 
**contact_email** | **str** |  | [optional] 
**founded_year** | **int** |  | [optional] 
**business_type** | **str** |  | [optional] 
**certifications** | **List[str]** |  | [optional] 
**license_number** | **str** |  | [optional] 
**revenue_range** | [**RevenueRange**](RevenueRange.md) |  | [optional] [default to RevenueRange.UNSPECIFIED]
**funding_stage** | **str** |  | [optional] 
**is_public_company** | **bool** |  | [optional] 
**website_load_speed** | **float** | In seconds | [optional] 
**has_ssl_certificate** | **bool** |  | [optional] 
**cms_used** | **str** |  | [optional] 
**ecommerce_platforms** | **List[str]** |  | [optional] 
**timezone** | **str** |  | [optional] 
**neighborhood** | **str** |  | [optional] 
**nearby_landmarks** | **List[str]** |  | [optional] 
**transportation_access** | **str** |  | [optional] 
**employee_benefits** | [**List[EmployeeBenefit]**](EmployeeBenefit.md) |  | [optional] 
**parent_company** | **str** |  | [optional] 
**subsidiaries** | **List[str]** |  | [optional] 
**is_franchise** | **bool** |  | [optional] 
**seo_keywords** | **List[str]** |  | [optional] 
**uses_google_ads** | **bool** |  | [optional] 
**google_my_business_category** | **str** |  | [optional] 
**naics_code** | **str** |  | [optional] 
**sic_code** | **str** |  | [optional] 
**unspsc_code** | **str** |  | [optional] 
**is_green_certified** | **bool** |  | [optional] 
**energy_sources** | **List[str]** |  | [optional] 
**sustainability_rating** | **str** |  | [optional] 
**recent_announcements** | **List[str]** |  | [optional] 
**last_product_launch** | **datetime** |  | [optional] 
**has_litigation_history** | **bool** |  | [optional] 
**export_control_status** | **str** |  | [optional] 

## Example

```python
from playbookmedia_backend_client_sdk.models.lead import Lead

# TODO update the JSON string below
json = "{}"
# create an instance of Lead from a JSON string
lead_instance = Lead.from_json(json)
# print the JSON string representation of the object
print(Lead.to_json())

# convert the object into a dict
lead_dict = lead_instance.to_dict()
# create an instance of Lead from a dict
lead_from_dict = Lead.from_dict(lead_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


