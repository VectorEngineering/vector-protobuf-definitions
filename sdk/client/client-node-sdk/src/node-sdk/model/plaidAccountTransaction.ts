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
import { Any } from './any';
import { ApprovalStatus } from './approvalStatus';
import { Attachment } from './attachment';
import { RecurringFrequency } from './recurringFrequency';
import { RegulatoryComplianceStatus } from './regulatoryComplianceStatus';
import { SmartNote } from './smartNote';
import { TransactionSplit } from './transactionSplit';
import { TransactionStatus } from './transactionStatus';

/**
 * Message representing Plaid account transactions.
 */
export class PlaidAccountTransaction {
  /**
   * The bank account ID associated with the transaction.
   */
  'accountId'?: string;
  /**
   * The amount of the transaction.  @gotag: ch:\"amount\"
   */
  'amount'?: number;
  /**
   * The currency code of the transaction.  @gotag: ch:\"iso_currency_code\"
   */
  'isoCurrencyCode'?: string;
  /**
   * The unofficial currency code of the transaction.  @gotag: ch:\"unofficial_currency_code\"
   */
  'unofficialCurrencyCode'?: string;
  /**
   * The transaction ID of interest.  @gotag: ch:\"transaction_id\"
   */
  'transactionId'?: string;
  /**
   * The transaction code.  @gotag: ch:\"transaction_code\"
   */
  'transactionCode'?: string;
  /**
   * The date of the transaction.  @gotag: ch:\"date\"
   */
  'currentDate'?: Date;
  /**
   * The current datetime of the transaction.  @gotag: ch:\"datetime\"
   */
  'currentDatetime'?: Date;
  /**
   * The time at which the transaction was authorized.  @gotag: ch:\"authorized_date\"
   */
  'authorizedDate'?: Date;
  /**
   * The date-time when the transaction was authorized.  @gotag: ch:\"authorized_datetime\"
   */
  'authorizedDatetime'?: Date;
  /**
   * The category ID of the transaction.  @gotag: ch:\"category_id\"
   */
  'categoryId'?: string;
  /**
   * The set of categories that the transaction belongs to.
   */
  'categories'?: Array<string>;
  /**
   * The primary personal finance category of the transaction.  @gotag: ch:\"personal_finance_category_primary\"
   */
  'personalFinanceCategoryPrimary'?: string;
  /**
   * The detailed personal finance category of the transaction.  @gotag: ch:\"personal_finance_category_detailed\"
   */
  'personalFinanceCategoryDetailed'?: string;
  /**
   * The name of the transaction.  @gotag: ch:\"name\"
   */
  'transactionName'?: string;
  /**
   * The merchant name of the transaction.  @gotag: ch:\"merchant_name\"
   */
  'merchantName'?: string;
  /**
   * The check number associated with the transaction.  @gotag: ch:\"check_number\"
   */
  'checkNumber'?: string;
  /**
   * The payment channel for the transaction.  @gotag: ch:\"payment_channel\"
   */
  'paymentChannel'?: string;
  /**
   * Indicates whether the transaction is pending.  @gotag: ch:\"pending\"
   */
  'pending'?: boolean;
  /**
   * The ID of the pending transaction, if applicable.  @gotag: ch:\"pending_transaction_id\"
   */
  'pendingTransactionId'?: string;
  /**
   * The account owner associated with the transaction.  @gotag: ch:\"account_owner\"
   */
  'accountOwner'?: string;
  /**
   * Information about the entity to whom the payment is made (if available).
   */
  'paymentMetaByOrderOf'?: string;
  /**
   * Information about the payee (if available).
   */
  'paymentMetaPayee'?: string;
  /**
   * Information about the payer (if available).
   */
  'paymentMetaPayer'?: string;
  /**
   * The payment method used for the transaction (if available).
   */
  'paymentMetaPaymentMethod'?: string;
  /**
   * The payment processor involved in the transaction (if available).
   */
  'paymentMetaPaymentProcessor'?: string;
  /**
   * The Prearranged Payment and Deposit (PPD) ID (if available).
   */
  'paymentMetaPpdId'?: string;
  /**
   * The reason for the payment (if available).
   */
  'paymentMetaReason'?: string;
  /**
   * The reference number associated with the payment (if available).
   */
  'paymentMetaReferenceNumber'?: string;
  /**
   * The street address of the transaction location (if available).
   */
  'locationAddress'?: string;
  /**
   * The city of the transaction location (if available).
   */
  'locationCity'?: string;
  /**
   * The region or state of the transaction location (if available).
   */
  'locationRegion'?: string;
  /**
   * The postal code of the transaction location (if available).
   */
  'locationPostalCode'?: string;
  /**
   * The country of the transaction location (if available).
   */
  'locationCountry'?: string;
  /**
   * The latitude of the transaction location (if available).
   */
  'locationLat'?: number;
  /**
   * The longitude of the transaction location (if available).
   */
  'locationLon'?: number;
  /**
   * The store number associated with the transaction location (if available).
   */
  'locationStoreNumber'?: string;
  /**
   * The timestamp associated with the transaction.
   */
  'time'?: Date;
  'additionalProperties'?: Any;
  /**
   * The unique ID for this transaction.
   */
  'id'?: string;
  /**
   * The user ID associated with this transaction.
   */
  'userId'?: string;
  /**
   * The link ID associated with this transaction.
   */
  'linkId'?: string;
  /**
   * Indicates whether this transaction needs review.
   */
  'needsReview'?: boolean;
  /**
   * Indicates whether this transaction should be hidden.
   */
  'hideTransaction'?: boolean;
  /**
   * Tags associated with this transaction.
   */
  'tags'?: Array<string>;
  /**
   * Notes associated with this transaction.
   */
  'notes'?: Array<SmartNote>;
  /**
   * The number of splits associated with this transaction.
   */
  'splits'?: Array<TransactionSplit>;
  'deletedAt'?: Date;
  /**
   * The cost center associated with this transaction.
   */
  'costCenter'?: string;
  /**
   * The project associated with this transaction.
   */
  'project'?: string;
  /**
   * The tax amount associated with this transaction.
   */
  'taxAmount'?: number;
  /**
   * The tax rate applied to this transaction.
   */
  'taxRate'?: number;
  /**
   * The tax code associated with this transaction.
   */
  'taxCode'?: string;
  /**
   * The tax jurisdiction.
   */
  'taxJurisdiction'?: string;
  /**
   * The type of tax (e.g., VAT, GST, Sales Tax).
   */
  'taxType'?: string;
  /**
   * The invoice number associated with this transaction.
   */
  'invoiceNumber'?: string;
  'billingReference'?: string;
  /**
   * The payment terms associated with this transaction.
   */
  'paymentTerms'?: string;
  /**
   * The vendor ID associated with this transaction.
   */
  'vendorId'?: string;
  /**
   * The vendor name associated with this transaction.
   */
  'vendorName'?: string;
  /**
   * The customer name associated with this transaction.
   */
  'customerName'?: string;
  'approvalStatus'?: ApprovalStatus;
  /**
   * The email of the user who approved this transaction.
   */
  'approvedByEmail'?: string;
  /**
   * The date when this transaction was approved.
   */
  'approvedDate'?: Date;
  'transactionStatus'?: TransactionStatus;
  /**
   * The attachments associated with this transaction.
   */
  'attachments'?: Array<Attachment>;
  /**
   * Indicates whether this transaction is recurring.
   */
  'isRecurring'?: boolean;
  'recurringFrequency'?: RecurringFrequency;
  /**
   * The exchange rate applied to this transaction.
   */
  'exchangeRate'?: number;
  /**
   * The base currency amount of the transaction.
   */
  'baseCurrencyAmount'?: number;
  /**
   * Indicates whether this transaction is enabled for regulatory compliance.
   */
  'enableRegulatoryCompliance'?: boolean;
  'regulatoryComplianceStatus'?: RegulatoryComplianceStatus;
  /**
   * The payment ID associated with this transaction.
   */
  'paymentId'?: string;
  /**
   * The settlement date of the transaction.
   */
  'settlementDate'?: Date;
  /**
   * The risk score of the transaction.
   */
  'riskScore'?: number;
  /**
   * The risk flags of the transaction.
   */
  'riskFlags'?: Array<string>;
  /**
   * Indicates whether this transaction is subject to SOX compliance.
   */
  'soxCompliant'?: boolean;
  /**
   * Indicates whether this transaction is subject to GDPR compliance.
   */
  'gdprCompliant'?: boolean;
  /**
   * The email of the user assigned to this transaction.
   */
  'assignedToUserId'?: string;

  static discriminator: string | undefined = undefined;

  static attributeTypeMap: Array<{
    name: string;
    baseName: string;
    type: string;
  }> = [
    {
      name: 'accountId',
      baseName: 'accountId',
      type: 'string',
    },
    {
      name: 'amount',
      baseName: 'amount',
      type: 'number',
    },
    {
      name: 'isoCurrencyCode',
      baseName: 'isoCurrencyCode',
      type: 'string',
    },
    {
      name: 'unofficialCurrencyCode',
      baseName: 'unofficialCurrencyCode',
      type: 'string',
    },
    {
      name: 'transactionId',
      baseName: 'transactionId',
      type: 'string',
    },
    {
      name: 'transactionCode',
      baseName: 'transactionCode',
      type: 'string',
    },
    {
      name: 'currentDate',
      baseName: 'currentDate',
      type: 'Date',
    },
    {
      name: 'currentDatetime',
      baseName: 'currentDatetime',
      type: 'Date',
    },
    {
      name: 'authorizedDate',
      baseName: 'authorizedDate',
      type: 'Date',
    },
    {
      name: 'authorizedDatetime',
      baseName: 'authorizedDatetime',
      type: 'Date',
    },
    {
      name: 'categoryId',
      baseName: 'categoryId',
      type: 'string',
    },
    {
      name: 'categories',
      baseName: 'categories',
      type: 'Array<string>',
    },
    {
      name: 'personalFinanceCategoryPrimary',
      baseName: 'personalFinanceCategoryPrimary',
      type: 'string',
    },
    {
      name: 'personalFinanceCategoryDetailed',
      baseName: 'personalFinanceCategoryDetailed',
      type: 'string',
    },
    {
      name: 'transactionName',
      baseName: 'transactionName',
      type: 'string',
    },
    {
      name: 'merchantName',
      baseName: 'merchantName',
      type: 'string',
    },
    {
      name: 'checkNumber',
      baseName: 'checkNumber',
      type: 'string',
    },
    {
      name: 'paymentChannel',
      baseName: 'paymentChannel',
      type: 'string',
    },
    {
      name: 'pending',
      baseName: 'pending',
      type: 'boolean',
    },
    {
      name: 'pendingTransactionId',
      baseName: 'pendingTransactionId',
      type: 'string',
    },
    {
      name: 'accountOwner',
      baseName: 'accountOwner',
      type: 'string',
    },
    {
      name: 'paymentMetaByOrderOf',
      baseName: 'paymentMetaByOrderOf',
      type: 'string',
    },
    {
      name: 'paymentMetaPayee',
      baseName: 'paymentMetaPayee',
      type: 'string',
    },
    {
      name: 'paymentMetaPayer',
      baseName: 'paymentMetaPayer',
      type: 'string',
    },
    {
      name: 'paymentMetaPaymentMethod',
      baseName: 'paymentMetaPaymentMethod',
      type: 'string',
    },
    {
      name: 'paymentMetaPaymentProcessor',
      baseName: 'paymentMetaPaymentProcessor',
      type: 'string',
    },
    {
      name: 'paymentMetaPpdId',
      baseName: 'paymentMetaPpdId',
      type: 'string',
    },
    {
      name: 'paymentMetaReason',
      baseName: 'paymentMetaReason',
      type: 'string',
    },
    {
      name: 'paymentMetaReferenceNumber',
      baseName: 'paymentMetaReferenceNumber',
      type: 'string',
    },
    {
      name: 'locationAddress',
      baseName: 'locationAddress',
      type: 'string',
    },
    {
      name: 'locationCity',
      baseName: 'locationCity',
      type: 'string',
    },
    {
      name: 'locationRegion',
      baseName: 'locationRegion',
      type: 'string',
    },
    {
      name: 'locationPostalCode',
      baseName: 'locationPostalCode',
      type: 'string',
    },
    {
      name: 'locationCountry',
      baseName: 'locationCountry',
      type: 'string',
    },
    {
      name: 'locationLat',
      baseName: 'locationLat',
      type: 'number',
    },
    {
      name: 'locationLon',
      baseName: 'locationLon',
      type: 'number',
    },
    {
      name: 'locationStoreNumber',
      baseName: 'locationStoreNumber',
      type: 'string',
    },
    {
      name: 'time',
      baseName: 'time',
      type: 'Date',
    },
    {
      name: 'additionalProperties',
      baseName: 'additionalProperties',
      type: 'Any',
    },
    {
      name: 'id',
      baseName: 'id',
      type: 'string',
    },
    {
      name: 'userId',
      baseName: 'userId',
      type: 'string',
    },
    {
      name: 'linkId',
      baseName: 'linkId',
      type: 'string',
    },
    {
      name: 'needsReview',
      baseName: 'needsReview',
      type: 'boolean',
    },
    {
      name: 'hideTransaction',
      baseName: 'hideTransaction',
      type: 'boolean',
    },
    {
      name: 'tags',
      baseName: 'tags',
      type: 'Array<string>',
    },
    {
      name: 'notes',
      baseName: 'notes',
      type: 'Array<SmartNote>',
    },
    {
      name: 'splits',
      baseName: 'splits',
      type: 'Array<TransactionSplit>',
    },
    {
      name: 'deletedAt',
      baseName: 'deletedAt',
      type: 'Date',
    },
    {
      name: 'costCenter',
      baseName: 'costCenter',
      type: 'string',
    },
    {
      name: 'project',
      baseName: 'project',
      type: 'string',
    },
    {
      name: 'taxAmount',
      baseName: 'taxAmount',
      type: 'number',
    },
    {
      name: 'taxRate',
      baseName: 'taxRate',
      type: 'number',
    },
    {
      name: 'taxCode',
      baseName: 'taxCode',
      type: 'string',
    },
    {
      name: 'taxJurisdiction',
      baseName: 'taxJurisdiction',
      type: 'string',
    },
    {
      name: 'taxType',
      baseName: 'taxType',
      type: 'string',
    },
    {
      name: 'invoiceNumber',
      baseName: 'invoiceNumber',
      type: 'string',
    },
    {
      name: 'billingReference',
      baseName: 'billingReference',
      type: 'string',
    },
    {
      name: 'paymentTerms',
      baseName: 'paymentTerms',
      type: 'string',
    },
    {
      name: 'vendorId',
      baseName: 'vendorId',
      type: 'string',
    },
    {
      name: 'vendorName',
      baseName: 'vendorName',
      type: 'string',
    },
    {
      name: 'customerName',
      baseName: 'customerName',
      type: 'string',
    },
    {
      name: 'approvalStatus',
      baseName: 'approvalStatus',
      type: 'ApprovalStatus',
    },
    {
      name: 'approvedByEmail',
      baseName: 'approvedByEmail',
      type: 'string',
    },
    {
      name: 'approvedDate',
      baseName: 'approvedDate',
      type: 'Date',
    },
    {
      name: 'transactionStatus',
      baseName: 'transactionStatus',
      type: 'TransactionStatus',
    },
    {
      name: 'attachments',
      baseName: 'attachments',
      type: 'Array<Attachment>',
    },
    {
      name: 'isRecurring',
      baseName: 'isRecurring',
      type: 'boolean',
    },
    {
      name: 'recurringFrequency',
      baseName: 'recurringFrequency',
      type: 'RecurringFrequency',
    },
    {
      name: 'exchangeRate',
      baseName: 'exchangeRate',
      type: 'number',
    },
    {
      name: 'baseCurrencyAmount',
      baseName: 'baseCurrencyAmount',
      type: 'number',
    },
    {
      name: 'enableRegulatoryCompliance',
      baseName: 'enableRegulatoryCompliance',
      type: 'boolean',
    },
    {
      name: 'regulatoryComplianceStatus',
      baseName: 'regulatoryComplianceStatus',
      type: 'RegulatoryComplianceStatus',
    },
    {
      name: 'paymentId',
      baseName: 'paymentId',
      type: 'string',
    },
    {
      name: 'settlementDate',
      baseName: 'settlementDate',
      type: 'Date',
    },
    {
      name: 'riskScore',
      baseName: 'riskScore',
      type: 'number',
    },
    {
      name: 'riskFlags',
      baseName: 'riskFlags',
      type: 'Array<string>',
    },
    {
      name: 'soxCompliant',
      baseName: 'soxCompliant',
      type: 'boolean',
    },
    {
      name: 'gdprCompliant',
      baseName: 'gdprCompliant',
      type: 'boolean',
    },
    {
      name: 'assignedToUserId',
      baseName: 'assignedToUserId',
      type: 'string',
    },
  ];

  static getAttributeTypeMap() {
    return PlaidAccountTransaction.attributeTypeMap;
  }
}

export namespace PlaidAccountTransaction {}
