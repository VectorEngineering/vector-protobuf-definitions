package lead_scraper_servicev1

// GetDatabaseSchemas returns a suite of database schemas
func GetDatabaseSchemas() []interface{} {
	models := []interface{}{
		ScrapingJobORM{},
		AccountORM{},
		WorkspaceORM{},
		ScrapingJobORM{},
		ScrapingWorkflowORM{},
		LeadORM{},
		ReviewORM{},
		BusinessHoursORM{},
		AccountSettingsORM{},
		APIKeyORM{},
		ResultORM{},
		TenantAPIKeyORM{},
		TenantORM{},
		OrganizationORM{},
	}

	return models
}
