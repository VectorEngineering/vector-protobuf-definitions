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
import { DocumentSnapshot } from './documentSnapshot';

/**
 * CreateDocumentSnapshotResponse represents the response containing a newly created document snapshot. Contains the full DocumentSnapshot object with metadata about the snapshot.
 */
export class CreateDocumentSnapshotResponse {
  'snapshot'?: DocumentSnapshot;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'snapshot',
      baseName: 'snapshot',
      type: 'DocumentSnapshot',
    },
  ];

  static getAttributeTypeMap() {
    return CreateDocumentSnapshotResponse.attributeTypeMap;
  }
}
