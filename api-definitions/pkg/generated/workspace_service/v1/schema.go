package workspace_servicev1

// GetDatabaseSchemas returns a suite of database schemas
func GetDatabaseSchemas() []interface{} {
	models := []interface{}{
		AccountORM{},
		DataProfileORM{},
		WorkspaceORM{},
		WorkspaceSharingORM{},
		WorkspaceActivityORM{},
		WorkspaceComplianceORM{},
		FolderMetadataORM{},
		FileEmbeddingsORM{},
		FileVersionORM{},
		CommentThreadORM{},
		FileSharingORM{},
		DocumentTemplateORM{},
		TemplateVersionORM{},
		TemplateVariableORM{},
		DocumentInstanceORM{},
		DocumentVersionORM{},
		DocumentBranchORM{},
		SignatureRequestORM{},
		ContractIntelligenceORM{},
		MarketplaceAppORM{},
		DocumentSnapshotORM{},
		ExplanationBlockORM{},
		AIAssistanceLogORM{},
		NegotiationHistoryORM{},
		NegotiationRoundORM{},
		ChangeSetORM{},
		ContextualSummaryORM{},
		BranchMergeORM{},
		SignatureBlockORM{},
		SignatureWorkflowORM{},
		RiskAssessmentORM{},
		ComplianceCheckORM{},
		AppVersionORM{},
		AppInstallationORM{},
		AppAnalyticsORM{},
		AppDevelopmentInfoORM{},
		AppWebhookORM{},
		AppPermissionORM{},
		BranchPolicyORM{},
		MergeRequestORM{},
	}

	return models
}