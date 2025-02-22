// Code generated by gorm.io/gen. DO NOT EDIT.
// Code generated by gorm.io/gen. DO NOT EDIT.
// Code generated by gorm.io/gen. DO NOT EDIT.

package dal

import (
	"context"
	"strings"

	workspace_servicev1 "github.com/VectorEngineering/vector-protobuf-definitions/api-definitions/pkg/generated/workspace_service/v1"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
	"gorm.io/gorm/schema"

	"gorm.io/gen"
	"gorm.io/gen/field"
	"gorm.io/gen/helper"

	"gorm.io/plugin/dbresolver"

	"time"
)

func newWorkspaceComplianceORM(db *gorm.DB, opts ...gen.DOOption) workspaceComplianceORM {
	_workspaceComplianceORM := workspaceComplianceORM{}

	_workspaceComplianceORM.workspaceComplianceORMDo.UseDB(db, opts...)
	_workspaceComplianceORM.workspaceComplianceORMDo.UseModel(&workspace_servicev1.WorkspaceComplianceORM{})

	tableName := _workspaceComplianceORM.workspaceComplianceORMDo.TableName()
	_workspaceComplianceORM.ALL = field.NewAsterisk(tableName)
	_workspaceComplianceORM.CertificationId = field.NewString(tableName, "certification_id")
	_workspaceComplianceORM.ComplianceType = field.NewString(tableName, "compliance_type")
	_workspaceComplianceORM.CreatedAt = field.NewTime(tableName, "created_at")
	_workspaceComplianceORM.Id = field.NewUint64(tableName, "id")
	_workspaceComplianceORM.Status = field.NewString(tableName, "status")
	_workspaceComplianceORM.UpdatedAt = field.NewTime(tableName, "updated_at")
	_workspaceComplianceORM.ValidUntil = field.NewTime(tableName, "valid_until")
	_workspaceComplianceORM.WorkspaceId = field.NewUint64(tableName, "workspace_id")

	_workspaceComplianceORM.fillFieldMap()

	return _workspaceComplianceORM
}

type workspaceComplianceORM struct {
	workspaceComplianceORMDo

	ALL             field.Asterisk
	CertificationId field.String
	ComplianceType  field.String
	CreatedAt       field.Time
	Id              field.Uint64
	Status          field.String
	UpdatedAt       field.Time
	ValidUntil      field.Time
	WorkspaceId     field.Uint64

	fieldMap map[string]field.Expr
}

func (w workspaceComplianceORM) Table(newTableName string) *workspaceComplianceORM {
	w.workspaceComplianceORMDo.UseTable(newTableName)
	return w.updateTableName(newTableName)
}

func (w workspaceComplianceORM) As(alias string) *workspaceComplianceORM {
	w.workspaceComplianceORMDo.DO = *(w.workspaceComplianceORMDo.As(alias).(*gen.DO))
	return w.updateTableName(alias)
}

func (w *workspaceComplianceORM) updateTableName(table string) *workspaceComplianceORM {
	w.ALL = field.NewAsterisk(table)
	w.CertificationId = field.NewString(table, "certification_id")
	w.ComplianceType = field.NewString(table, "compliance_type")
	w.CreatedAt = field.NewTime(table, "created_at")
	w.Id = field.NewUint64(table, "id")
	w.Status = field.NewString(table, "status")
	w.UpdatedAt = field.NewTime(table, "updated_at")
	w.ValidUntil = field.NewTime(table, "valid_until")
	w.WorkspaceId = field.NewUint64(table, "workspace_id")

	w.fillFieldMap()

	return w
}

func (w *workspaceComplianceORM) GetFieldByName(fieldName string) (field.OrderExpr, bool) {
	_f, ok := w.fieldMap[fieldName]
	if !ok || _f == nil {
		return nil, false
	}
	_oe, ok := _f.(field.OrderExpr)
	return _oe, ok
}

func (w *workspaceComplianceORM) fillFieldMap() {
	w.fieldMap = make(map[string]field.Expr, 8)
	w.fieldMap["certification_id"] = w.CertificationId
	w.fieldMap["compliance_type"] = w.ComplianceType
	w.fieldMap["created_at"] = w.CreatedAt
	w.fieldMap["id"] = w.Id
	w.fieldMap["status"] = w.Status
	w.fieldMap["updated_at"] = w.UpdatedAt
	w.fieldMap["valid_until"] = w.ValidUntil
	w.fieldMap["workspace_id"] = w.WorkspaceId
}

func (w workspaceComplianceORM) clone(db *gorm.DB) workspaceComplianceORM {
	w.workspaceComplianceORMDo.ReplaceConnPool(db.Statement.ConnPool)
	return w
}

func (w workspaceComplianceORM) replaceDB(db *gorm.DB) workspaceComplianceORM {
	w.workspaceComplianceORMDo.ReplaceDB(db)
	return w
}

type workspaceComplianceORMDo struct{ gen.DO }

type IWorkspaceComplianceORMDo interface {
	gen.SubQuery
	Debug() IWorkspaceComplianceORMDo
	WithContext(ctx context.Context) IWorkspaceComplianceORMDo
	WithResult(fc func(tx gen.Dao)) gen.ResultInfo
	ReplaceDB(db *gorm.DB)
	ReadDB() IWorkspaceComplianceORMDo
	WriteDB() IWorkspaceComplianceORMDo
	As(alias string) gen.Dao
	Session(config *gorm.Session) IWorkspaceComplianceORMDo
	Columns(cols ...field.Expr) gen.Columns
	Clauses(conds ...clause.Expression) IWorkspaceComplianceORMDo
	Not(conds ...gen.Condition) IWorkspaceComplianceORMDo
	Or(conds ...gen.Condition) IWorkspaceComplianceORMDo
	Select(conds ...field.Expr) IWorkspaceComplianceORMDo
	Where(conds ...gen.Condition) IWorkspaceComplianceORMDo
	Order(conds ...field.Expr) IWorkspaceComplianceORMDo
	Distinct(cols ...field.Expr) IWorkspaceComplianceORMDo
	Omit(cols ...field.Expr) IWorkspaceComplianceORMDo
	Join(table schema.Tabler, on ...field.Expr) IWorkspaceComplianceORMDo
	LeftJoin(table schema.Tabler, on ...field.Expr) IWorkspaceComplianceORMDo
	RightJoin(table schema.Tabler, on ...field.Expr) IWorkspaceComplianceORMDo
	Group(cols ...field.Expr) IWorkspaceComplianceORMDo
	Having(conds ...gen.Condition) IWorkspaceComplianceORMDo
	Limit(limit int) IWorkspaceComplianceORMDo
	Offset(offset int) IWorkspaceComplianceORMDo
	Count() (count int64, err error)
	Scopes(funcs ...func(gen.Dao) gen.Dao) IWorkspaceComplianceORMDo
	Unscoped() IWorkspaceComplianceORMDo
	Create(values ...*workspace_servicev1.WorkspaceComplianceORM) error
	CreateInBatches(values []*workspace_servicev1.WorkspaceComplianceORM, batchSize int) error
	Save(values ...*workspace_servicev1.WorkspaceComplianceORM) error
	First() (*workspace_servicev1.WorkspaceComplianceORM, error)
	Take() (*workspace_servicev1.WorkspaceComplianceORM, error)
	Last() (*workspace_servicev1.WorkspaceComplianceORM, error)
	Find() ([]*workspace_servicev1.WorkspaceComplianceORM, error)
	FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.WorkspaceComplianceORM, err error)
	FindInBatches(result *[]*workspace_servicev1.WorkspaceComplianceORM, batchSize int, fc func(tx gen.Dao, batch int) error) error
	Pluck(column field.Expr, dest interface{}) error
	Delete(...*workspace_servicev1.WorkspaceComplianceORM) (info gen.ResultInfo, err error)
	Update(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	Updates(value interface{}) (info gen.ResultInfo, err error)
	UpdateColumn(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateColumnSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	UpdateColumns(value interface{}) (info gen.ResultInfo, err error)
	UpdateFrom(q gen.SubQuery) gen.Dao
	Attrs(attrs ...field.AssignExpr) IWorkspaceComplianceORMDo
	Assign(attrs ...field.AssignExpr) IWorkspaceComplianceORMDo
	Joins(fields ...field.RelationField) IWorkspaceComplianceORMDo
	Preload(fields ...field.RelationField) IWorkspaceComplianceORMDo
	FirstOrInit() (*workspace_servicev1.WorkspaceComplianceORM, error)
	FirstOrCreate() (*workspace_servicev1.WorkspaceComplianceORM, error)
	FindByPage(offset int, limit int) (result []*workspace_servicev1.WorkspaceComplianceORM, count int64, err error)
	ScanByPage(result interface{}, offset int, limit int) (count int64, err error)
	Scan(result interface{}) (err error)
	Returning(value interface{}, columns ...string) IWorkspaceComplianceORMDo
	UnderlyingDB() *gorm.DB
	schema.Tabler

	GetRecordByID(id uint64) (result workspace_servicev1.WorkspaceComplianceORM, err error)
	GetRecordByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	DeleteRecordByID(id uint64) (err error)
	GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	CountAll() (result int, err error)
	GetByID(id uint64) (result workspace_servicev1.WorkspaceComplianceORM, err error)
	GetByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	GetDeleted() (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	SoftDelete(id uint64) (err error)
	Restore(id uint64) (err error)
	DeleteInBatch(ids []uint64, batchSize int) (err error)
	GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	FindByPattern(columnName string, pattern string) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	GetDistinct(columnName string) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	Touch(id uint64) (err error)
	Exists(column string, value interface{}) (result bool, err error)
	ExistsById(id uint64) (result bool, err error)
	FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.WorkspaceComplianceORM, err error)
	ExistsAndGet(column string, value interface{}) (result workspace_servicev1.WorkspaceComplianceORM, err error)
	ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.WorkspaceComplianceORM, err error)
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceComplianceORMDo) GetRecordByID(id uint64) (result workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (w workspaceComplianceORMDo) GetRecordByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// DELETE FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceComplianceORMDo) DeleteRecordByID(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// ORDER BY @@orderColumn
// LIMIT @limit
// OFFSET @offset
func (w workspaceComplianceORMDo) GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("SELECT * FROM workspace_compliance ORDER BY " + w.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// Additional Operations
// SELECT COUNT(*) FROM @@table
func (w workspaceComplianceORMDo) CountAll() (result int, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("Additional Operations SELECT COUNT(*) FROM workspace_compliance ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String()).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceComplianceORMDo) GetByID(id uint64) (result workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (w workspaceComplianceORMDo) GetByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	deleted_at IS NULL
//
// {{end}}
// ORDER BY @@orderColumn
// LIMIT @limit OFFSET @offset
func (w workspaceComplianceORMDo) GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY " + w.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	deleted_at IS NOT NULL
//
// {{end}}
func (w workspaceComplianceORMDo) GetDeleted() (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NOT NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceComplianceORMDo) SoftDelete(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_compliance SET deleted_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=NULL
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceComplianceORMDo) Restore(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_compliance SET deleted_at=NULL ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// DELETE FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
// LIMIT @batchSize
func (w workspaceComplianceORMDo) DeleteInBatch(ids []uint64, batchSize int) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, batchSize)
	generateSQL.WriteString("LIMIT ? ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	created_at BETWEEN @startTime AND @endTime
//
// {{end}}
func (w workspaceComplianceORMDo) GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, startTime)
	params = append(params, endTime)
	whereSQL0.WriteString("created_at BETWEEN ? AND ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name @operator @value
//
// {{end}}
func (w workspaceComplianceORMDo) FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, operator)
	params = append(params, value)
	whereSQL0.WriteString("column_name ?? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name LIKE @pattern
//
// {{end}}
func (w workspaceComplianceORMDo) FindByPattern(columnName string, pattern string) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, pattern)
	whereSQL0.WriteString("column_name LIKE ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT DISTINCT column_name FROM @@table
func (w workspaceComplianceORMDo) GetDistinct(columnName string) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT DISTINCT column_name FROM workspace_compliance ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	@conditions
//
// {{end}}
// ORDER BY @orderBy
// LIMIT @limit OFFSET @offset
func (w workspaceComplianceORMDo) FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, conditions)
	whereSQL0.WriteString("? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, orderBy)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY ? LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET updated_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceComplianceORMDo) Touch(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_compliance SET updated_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT EXISTS(
//
//	SELECT 1 FROM @@table
//	 {{where}}
//	     @column = @value
//	 {{end}}
//
// )
func (w workspaceComplianceORMDo) Exists(column string, value interface{}) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT EXISTS(
//
//	SELECT 1 FROM @@table
//	 {{where}}
//	     id = @id
//	 {{end}}
//
// )
func (w workspaceComplianceORMDo) ExistsById(id uint64) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	@column IN (SELECT @foreignKey FROM @foreignTable)
//
// {{end}}
func (w workspaceComplianceORMDo) FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, foreignKey)
	params = append(params, foreignTable)
	whereSQL0.WriteString("? IN (SELECT ? FROM ?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT EXISTS(
//
//	SELECT 1 FROM @@table
//	 {{where}}
//	     @column = @value
//	 {{end}}
//
// ), COALESCE((
//
//	SELECT @column FROM @@table
//	 {{where}}
//	     @column = @value
//	 {{end}}
//	LIMIT 1
//
// ), NULL)
func (w workspaceComplianceORMDo) ExistsAndGet(column string, value interface{}) (result workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM workspace_compliance ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT EXISTS(
//
//	SELECT 1 FROM @@table
//	 {{where}}
//	     @column = @value AND deleted_at IS NULL
//	 {{end}}
//
// ), COALESCE((
//
//	SELECT @column FROM @@table
//	 {{where}}
//	     @column = @value AND deleted_at IS NULL
//	 {{end}}
//	LIMIT 1
//
// ), NULL)
func (w workspaceComplianceORMDo) ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.WorkspaceComplianceORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_compliance ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM workspace_compliance ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

func (w workspaceComplianceORMDo) Debug() IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Debug())
}

func (w workspaceComplianceORMDo) WithContext(ctx context.Context) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.WithContext(ctx))
}

func (w workspaceComplianceORMDo) ReadDB() IWorkspaceComplianceORMDo {
	return w.Clauses(dbresolver.Read)
}

func (w workspaceComplianceORMDo) WriteDB() IWorkspaceComplianceORMDo {
	return w.Clauses(dbresolver.Write)
}

func (w workspaceComplianceORMDo) Session(config *gorm.Session) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Session(config))
}

func (w workspaceComplianceORMDo) Clauses(conds ...clause.Expression) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Clauses(conds...))
}

func (w workspaceComplianceORMDo) Returning(value interface{}, columns ...string) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Returning(value, columns...))
}

func (w workspaceComplianceORMDo) Not(conds ...gen.Condition) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Not(conds...))
}

func (w workspaceComplianceORMDo) Or(conds ...gen.Condition) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Or(conds...))
}

func (w workspaceComplianceORMDo) Select(conds ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Select(conds...))
}

func (w workspaceComplianceORMDo) Where(conds ...gen.Condition) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Where(conds...))
}

func (w workspaceComplianceORMDo) Order(conds ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Order(conds...))
}

func (w workspaceComplianceORMDo) Distinct(cols ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Distinct(cols...))
}

func (w workspaceComplianceORMDo) Omit(cols ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Omit(cols...))
}

func (w workspaceComplianceORMDo) Join(table schema.Tabler, on ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Join(table, on...))
}

func (w workspaceComplianceORMDo) LeftJoin(table schema.Tabler, on ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.LeftJoin(table, on...))
}

func (w workspaceComplianceORMDo) RightJoin(table schema.Tabler, on ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.RightJoin(table, on...))
}

func (w workspaceComplianceORMDo) Group(cols ...field.Expr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Group(cols...))
}

func (w workspaceComplianceORMDo) Having(conds ...gen.Condition) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Having(conds...))
}

func (w workspaceComplianceORMDo) Limit(limit int) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Limit(limit))
}

func (w workspaceComplianceORMDo) Offset(offset int) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Offset(offset))
}

func (w workspaceComplianceORMDo) Scopes(funcs ...func(gen.Dao) gen.Dao) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Scopes(funcs...))
}

func (w workspaceComplianceORMDo) Unscoped() IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Unscoped())
}

func (w workspaceComplianceORMDo) Create(values ...*workspace_servicev1.WorkspaceComplianceORM) error {
	if len(values) == 0 {
		return nil
	}
	return w.DO.Create(values)
}

func (w workspaceComplianceORMDo) CreateInBatches(values []*workspace_servicev1.WorkspaceComplianceORM, batchSize int) error {
	return w.DO.CreateInBatches(values, batchSize)
}

// Save : !!! underlying implementation is different with GORM
// The method is equivalent to executing the statement: db.Clauses(clause.OnConflict{UpdateAll: true}).Create(values)
func (w workspaceComplianceORMDo) Save(values ...*workspace_servicev1.WorkspaceComplianceORM) error {
	if len(values) == 0 {
		return nil
	}
	return w.DO.Save(values)
}

func (w workspaceComplianceORMDo) First() (*workspace_servicev1.WorkspaceComplianceORM, error) {
	if result, err := w.DO.First(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceComplianceORM), nil
	}
}

func (w workspaceComplianceORMDo) Take() (*workspace_servicev1.WorkspaceComplianceORM, error) {
	if result, err := w.DO.Take(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceComplianceORM), nil
	}
}

func (w workspaceComplianceORMDo) Last() (*workspace_servicev1.WorkspaceComplianceORM, error) {
	if result, err := w.DO.Last(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceComplianceORM), nil
	}
}

func (w workspaceComplianceORMDo) Find() ([]*workspace_servicev1.WorkspaceComplianceORM, error) {
	result, err := w.DO.Find()
	return result.([]*workspace_servicev1.WorkspaceComplianceORM), err
}

func (w workspaceComplianceORMDo) FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.WorkspaceComplianceORM, err error) {
	buf := make([]*workspace_servicev1.WorkspaceComplianceORM, 0, batchSize)
	err = w.DO.FindInBatches(&buf, batchSize, func(tx gen.Dao, batch int) error {
		defer func() { results = append(results, buf...) }()
		return fc(tx, batch)
	})
	return results, err
}

func (w workspaceComplianceORMDo) FindInBatches(result *[]*workspace_servicev1.WorkspaceComplianceORM, batchSize int, fc func(tx gen.Dao, batch int) error) error {
	return w.DO.FindInBatches(result, batchSize, fc)
}

func (w workspaceComplianceORMDo) Attrs(attrs ...field.AssignExpr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Attrs(attrs...))
}

func (w workspaceComplianceORMDo) Assign(attrs ...field.AssignExpr) IWorkspaceComplianceORMDo {
	return w.withDO(w.DO.Assign(attrs...))
}

func (w workspaceComplianceORMDo) Joins(fields ...field.RelationField) IWorkspaceComplianceORMDo {
	for _, _f := range fields {
		w = *w.withDO(w.DO.Joins(_f))
	}
	return &w
}

func (w workspaceComplianceORMDo) Preload(fields ...field.RelationField) IWorkspaceComplianceORMDo {
	for _, _f := range fields {
		w = *w.withDO(w.DO.Preload(_f))
	}
	return &w
}

func (w workspaceComplianceORMDo) FirstOrInit() (*workspace_servicev1.WorkspaceComplianceORM, error) {
	if result, err := w.DO.FirstOrInit(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceComplianceORM), nil
	}
}

func (w workspaceComplianceORMDo) FirstOrCreate() (*workspace_servicev1.WorkspaceComplianceORM, error) {
	if result, err := w.DO.FirstOrCreate(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceComplianceORM), nil
	}
}

func (w workspaceComplianceORMDo) FindByPage(offset int, limit int) (result []*workspace_servicev1.WorkspaceComplianceORM, count int64, err error) {
	result, err = w.Offset(offset).Limit(limit).Find()
	if err != nil {
		return
	}

	if size := len(result); 0 < limit && 0 < size && size < limit {
		count = int64(size + offset)
		return
	}

	count, err = w.Offset(-1).Limit(-1).Count()
	return
}

func (w workspaceComplianceORMDo) ScanByPage(result interface{}, offset int, limit int) (count int64, err error) {
	count, err = w.Count()
	if err != nil {
		return
	}

	err = w.Offset(offset).Limit(limit).Scan(result)
	return
}

func (w workspaceComplianceORMDo) Scan(result interface{}) (err error) {
	return w.DO.Scan(result)
}

func (w workspaceComplianceORMDo) Delete(models ...*workspace_servicev1.WorkspaceComplianceORM) (result gen.ResultInfo, err error) {
	return w.DO.Delete(models)
}

func (w *workspaceComplianceORMDo) withDO(do gen.Dao) *workspaceComplianceORMDo {
	w.DO = *do.(*gen.DO)
	return w
}
