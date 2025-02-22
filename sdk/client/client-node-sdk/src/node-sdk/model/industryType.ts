/**
 * User Service API
 * Solomon AI User Service API - Manages user profiles and authentication
 *
 * The version of the OpenAPI document: 1.0
 * Contact: yoanyomba@solomon-ai.co
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';

/**
 * Represents various industries a business can belong to.   - INDUSTRY_TYPE_UNSPECIFIED: Default unspecified industry.  - INDUSTRY_TYPE_TECHNOLOGY: Technology sector.  - INDUSTRY_TYPE_FINANCE: Finance sector.  - INDUSTRY_TYPE_HEALTHCARE: Healthcare sector.  - INDUSTRY_TYPE_EDUCATION: Education sector.  - INDUSTRY_TYPE_RETAIL: Retail sector.  - INDUSTRY_TYPE_MANUFACTURING: Manufacturing sector.  - INDUSTRY_TYPE_OTHER: Other or unlisted industry.
 */
export enum IndustryType {
  Unspecified = <any>'INDUSTRY_TYPE_UNSPECIFIED',
  Technology = <any>'INDUSTRY_TYPE_TECHNOLOGY',
  Finance = <any>'INDUSTRY_TYPE_FINANCE',
  Healthcare = <any>'INDUSTRY_TYPE_HEALTHCARE',
  Education = <any>'INDUSTRY_TYPE_EDUCATION',
  Retail = <any>'INDUSTRY_TYPE_RETAIL',
  Manufacturing = <any>'INDUSTRY_TYPE_MANUFACTURING',
  Other = <any>'INDUSTRY_TYPE_OTHER',
}
