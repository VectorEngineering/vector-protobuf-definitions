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

func newWorkspaceSharingORM(db *gorm.DB, opts ...gen.DOOption) workspaceSharingORM {
	_workspaceSharingORM := workspaceSharingORM{}

	_workspaceSharingORM.workspaceSharingORMDo.UseDB(db, opts...)
	_workspaceSharingORM.workspaceSharingORMDo.UseModel(&workspace_servicev1.WorkspaceSharingORM{})

	tableName := _workspaceSharingORM.workspaceSharingORMDo.TableName()
	_workspaceSharingORM.ALL = field.NewAsterisk(tableName)
	_workspaceSharingORM.CreatedAt = field.NewTime(tableName, "created_at")
	_workspaceSharingORM.ExpiresAt = field.NewTime(tableName, "expires_at")
	_workspaceSharingORM.Id = field.NewUint64(tableName, "id")
	_workspaceSharingORM.PermissionLevel = field.NewString(tableName, "permission_level")
	_workspaceSharingORM.SharedWithEmail = field.NewString(tableName, "shared_with_email")
	_workspaceSharingORM.UpdatedAt = field.NewTime(tableName, "updated_at")
	_workspaceSharingORM.WorkspaceId = field.NewUint64(tableName, "workspace_id")

	_workspaceSharingORM.fillFieldMap()

	return _workspaceSharingORM
}

type workspaceSharingORM struct {
	workspaceSharingORMDo

	ALL             field.Asterisk
	CreatedAt       field.Time
	ExpiresAt       field.Time
	Id              field.Uint64
	PermissionLevel field.String
	SharedWithEmail field.String
	UpdatedAt       field.Time
	WorkspaceId     field.Uint64

	fieldMap map[string]field.Expr
}

func (w workspaceSharingORM) Table(newTableName string) *workspaceSharingORM {
	w.workspaceSharingORMDo.UseTable(newTableName)
	return w.updateTableName(newTableName)
}

func (w workspaceSharingORM) As(alias string) *workspaceSharingORM {
	w.workspaceSharingORMDo.DO = *(w.workspaceSharingORMDo.As(alias).(*gen.DO))
	return w.updateTableName(alias)
}

func (w *workspaceSharingORM) updateTableName(table string) *workspaceSharingORM {
	w.ALL = field.NewAsterisk(table)
	w.CreatedAt = field.NewTime(table, "created_at")
	w.ExpiresAt = field.NewTime(table, "expires_at")
	w.Id = field.NewUint64(table, "id")
	w.PermissionLevel = field.NewString(table, "permission_level")
	w.SharedWithEmail = field.NewString(table, "shared_with_email")
	w.UpdatedAt = field.NewTime(table, "updated_at")
	w.WorkspaceId = field.NewUint64(table, "workspace_id")

	w.fillFieldMap()

	return w
}

func (w *workspaceSharingORM) GetFieldByName(fieldName string) (field.OrderExpr, bool) {
	_f, ok := w.fieldMap[fieldName]
	if !ok || _f == nil {
		return nil, false
	}
	_oe, ok := _f.(field.OrderExpr)
	return _oe, ok
}

func (w *workspaceSharingORM) fillFieldMap() {
	w.fieldMap = make(map[string]field.Expr, 7)
	w.fieldMap["created_at"] = w.CreatedAt
	w.fieldMap["expires_at"] = w.ExpiresAt
	w.fieldMap["id"] = w.Id
	w.fieldMap["permission_level"] = w.PermissionLevel
	w.fieldMap["shared_with_email"] = w.SharedWithEmail
	w.fieldMap["updated_at"] = w.UpdatedAt
	w.fieldMap["workspace_id"] = w.WorkspaceId
}

func (w workspaceSharingORM) clone(db *gorm.DB) workspaceSharingORM {
	w.workspaceSharingORMDo.ReplaceConnPool(db.Statement.ConnPool)
	return w
}

func (w workspaceSharingORM) replaceDB(db *gorm.DB) workspaceSharingORM {
	w.workspaceSharingORMDo.ReplaceDB(db)
	return w
}

type workspaceSharingORMDo struct{ gen.DO }

type IWorkspaceSharingORMDo interface {
	gen.SubQuery
	Debug() IWorkspaceSharingORMDo
	WithContext(ctx context.Context) IWorkspaceSharingORMDo
	WithResult(fc func(tx gen.Dao)) gen.ResultInfo
	ReplaceDB(db *gorm.DB)
	ReadDB() IWorkspaceSharingORMDo
	WriteDB() IWorkspaceSharingORMDo
	As(alias string) gen.Dao
	Session(config *gorm.Session) IWorkspaceSharingORMDo
	Columns(cols ...field.Expr) gen.Columns
	Clauses(conds ...clause.Expression) IWorkspaceSharingORMDo
	Not(conds ...gen.Condition) IWorkspaceSharingORMDo
	Or(conds ...gen.Condition) IWorkspaceSharingORMDo
	Select(conds ...field.Expr) IWorkspaceSharingORMDo
	Where(conds ...gen.Condition) IWorkspaceSharingORMDo
	Order(conds ...field.Expr) IWorkspaceSharingORMDo
	Distinct(cols ...field.Expr) IWorkspaceSharingORMDo
	Omit(cols ...field.Expr) IWorkspaceSharingORMDo
	Join(table schema.Tabler, on ...field.Expr) IWorkspaceSharingORMDo
	LeftJoin(table schema.Tabler, on ...field.Expr) IWorkspaceSharingORMDo
	RightJoin(table schema.Tabler, on ...field.Expr) IWorkspaceSharingORMDo
	Group(cols ...field.Expr) IWorkspaceSharingORMDo
	Having(conds ...gen.Condition) IWorkspaceSharingORMDo
	Limit(limit int) IWorkspaceSharingORMDo
	Offset(offset int) IWorkspaceSharingORMDo
	Count() (count int64, err error)
	Scopes(funcs ...func(gen.Dao) gen.Dao) IWorkspaceSharingORMDo
	Unscoped() IWorkspaceSharingORMDo
	Create(values ...*workspace_servicev1.WorkspaceSharingORM) error
	CreateInBatches(values []*workspace_servicev1.WorkspaceSharingORM, batchSize int) error
	Save(values ...*workspace_servicev1.WorkspaceSharingORM) error
	First() (*workspace_servicev1.WorkspaceSharingORM, error)
	Take() (*workspace_servicev1.WorkspaceSharingORM, error)
	Last() (*workspace_servicev1.WorkspaceSharingORM, error)
	Find() ([]*workspace_servicev1.WorkspaceSharingORM, error)
	FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.WorkspaceSharingORM, err error)
	FindInBatches(result *[]*workspace_servicev1.WorkspaceSharingORM, batchSize int, fc func(tx gen.Dao, batch int) error) error
	Pluck(column field.Expr, dest interface{}) error
	Delete(...*workspace_servicev1.WorkspaceSharingORM) (info gen.ResultInfo, err error)
	Update(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	Updates(value interface{}) (info gen.ResultInfo, err error)
	UpdateColumn(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateColumnSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	UpdateColumns(value interface{}) (info gen.ResultInfo, err error)
	UpdateFrom(q gen.SubQuery) gen.Dao
	Attrs(attrs ...field.AssignExpr) IWorkspaceSharingORMDo
	Assign(attrs ...field.AssignExpr) IWorkspaceSharingORMDo
	Joins(fields ...field.RelationField) IWorkspaceSharingORMDo
	Preload(fields ...field.RelationField) IWorkspaceSharingORMDo
	FirstOrInit() (*workspace_servicev1.WorkspaceSharingORM, error)
	FirstOrCreate() (*workspace_servicev1.WorkspaceSharingORM, error)
	FindByPage(offset int, limit int) (result []*workspace_servicev1.WorkspaceSharingORM, count int64, err error)
	ScanByPage(result interface{}, offset int, limit int) (count int64, err error)
	Scan(result interface{}) (err error)
	Returning(value interface{}, columns ...string) IWorkspaceSharingORMDo
	UnderlyingDB() *gorm.DB
	schema.Tabler

	GetRecordByID(id uint64) (result workspace_servicev1.WorkspaceSharingORM, err error)
	GetRecordByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	DeleteRecordByID(id uint64) (err error)
	GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	CountAll() (result int, err error)
	GetByID(id uint64) (result workspace_servicev1.WorkspaceSharingORM, err error)
	GetByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	GetDeleted() (result []workspace_servicev1.WorkspaceSharingORM, err error)
	SoftDelete(id uint64) (err error)
	Restore(id uint64) (err error)
	DeleteInBatch(ids []uint64, batchSize int) (err error)
	GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	FindByPattern(columnName string, pattern string) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	GetDistinct(columnName string) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	Touch(id uint64) (err error)
	Exists(column string, value interface{}) (result bool, err error)
	ExistsById(id uint64) (result bool, err error)
	FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.WorkspaceSharingORM, err error)
	ExistsAndGet(column string, value interface{}) (result workspace_servicev1.WorkspaceSharingORM, err error)
	ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.WorkspaceSharingORM, err error)
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceSharingORMDo) GetRecordByID(id uint64) (result workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetRecordByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) DeleteRecordByID(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("SELECT * FROM workspace_sharings ORDER BY " + w.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// Additional Operations
// SELECT COUNT(*) FROM @@table
func (w workspaceSharingORMDo) CountAll() (result int, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("Additional Operations SELECT COUNT(*) FROM workspace_sharings ")

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
func (w workspaceSharingORMDo) GetByID(id uint64) (result workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetDeleted() (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) SoftDelete(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_sharings SET deleted_at=CURRENT_TIMESTAMP ")
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
func (w workspaceSharingORMDo) Restore(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_sharings SET deleted_at=NULL ")
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
func (w workspaceSharingORMDo) DeleteInBatch(ids []uint64, batchSize int) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) FindByPattern(columnName string, pattern string) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) GetDistinct(columnName string) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT DISTINCT column_name FROM workspace_sharings ")

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
func (w workspaceSharingORMDo) FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) Touch(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_sharings SET updated_at=CURRENT_TIMESTAMP ")
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
func (w workspaceSharingORMDo) Exists(column string, value interface{}) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) ExistsById(id uint64) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) ExistsAndGet(column string, value interface{}) (result workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_sharings ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM workspace_sharings ")
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
func (w workspaceSharingORMDo) ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.WorkspaceSharingORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_sharings ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM workspace_sharings ")
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

func (w workspaceSharingORMDo) Debug() IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Debug())
}

func (w workspaceSharingORMDo) WithContext(ctx context.Context) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.WithContext(ctx))
}

func (w workspaceSharingORMDo) ReadDB() IWorkspaceSharingORMDo {
	return w.Clauses(dbresolver.Read)
}

func (w workspaceSharingORMDo) WriteDB() IWorkspaceSharingORMDo {
	return w.Clauses(dbresolver.Write)
}

func (w workspaceSharingORMDo) Session(config *gorm.Session) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Session(config))
}

func (w workspaceSharingORMDo) Clauses(conds ...clause.Expression) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Clauses(conds...))
}

func (w workspaceSharingORMDo) Returning(value interface{}, columns ...string) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Returning(value, columns...))
}

func (w workspaceSharingORMDo) Not(conds ...gen.Condition) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Not(conds...))
}

func (w workspaceSharingORMDo) Or(conds ...gen.Condition) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Or(conds...))
}

func (w workspaceSharingORMDo) Select(conds ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Select(conds...))
}

func (w workspaceSharingORMDo) Where(conds ...gen.Condition) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Where(conds...))
}

func (w workspaceSharingORMDo) Order(conds ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Order(conds...))
}

func (w workspaceSharingORMDo) Distinct(cols ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Distinct(cols...))
}

func (w workspaceSharingORMDo) Omit(cols ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Omit(cols...))
}

func (w workspaceSharingORMDo) Join(table schema.Tabler, on ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Join(table, on...))
}

func (w workspaceSharingORMDo) LeftJoin(table schema.Tabler, on ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.LeftJoin(table, on...))
}

func (w workspaceSharingORMDo) RightJoin(table schema.Tabler, on ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.RightJoin(table, on...))
}

func (w workspaceSharingORMDo) Group(cols ...field.Expr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Group(cols...))
}

func (w workspaceSharingORMDo) Having(conds ...gen.Condition) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Having(conds...))
}

func (w workspaceSharingORMDo) Limit(limit int) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Limit(limit))
}

func (w workspaceSharingORMDo) Offset(offset int) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Offset(offset))
}

func (w workspaceSharingORMDo) Scopes(funcs ...func(gen.Dao) gen.Dao) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Scopes(funcs...))
}

func (w workspaceSharingORMDo) Unscoped() IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Unscoped())
}

func (w workspaceSharingORMDo) Create(values ...*workspace_servicev1.WorkspaceSharingORM) error {
	if len(values) == 0 {
		return nil
	}
	return w.DO.Create(values)
}

func (w workspaceSharingORMDo) CreateInBatches(values []*workspace_servicev1.WorkspaceSharingORM, batchSize int) error {
	return w.DO.CreateInBatches(values, batchSize)
}

// Save : !!! underlying implementation is different with GORM
// The method is equivalent to executing the statement: db.Clauses(clause.OnConflict{UpdateAll: true}).Create(values)
func (w workspaceSharingORMDo) Save(values ...*workspace_servicev1.WorkspaceSharingORM) error {
	if len(values) == 0 {
		return nil
	}
	return w.DO.Save(values)
}

func (w workspaceSharingORMDo) First() (*workspace_servicev1.WorkspaceSharingORM, error) {
	if result, err := w.DO.First(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceSharingORM), nil
	}
}

func (w workspaceSharingORMDo) Take() (*workspace_servicev1.WorkspaceSharingORM, error) {
	if result, err := w.DO.Take(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceSharingORM), nil
	}
}

func (w workspaceSharingORMDo) Last() (*workspace_servicev1.WorkspaceSharingORM, error) {
	if result, err := w.DO.Last(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceSharingORM), nil
	}
}

func (w workspaceSharingORMDo) Find() ([]*workspace_servicev1.WorkspaceSharingORM, error) {
	result, err := w.DO.Find()
	return result.([]*workspace_servicev1.WorkspaceSharingORM), err
}

func (w workspaceSharingORMDo) FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.WorkspaceSharingORM, err error) {
	buf := make([]*workspace_servicev1.WorkspaceSharingORM, 0, batchSize)
	err = w.DO.FindInBatches(&buf, batchSize, func(tx gen.Dao, batch int) error {
		defer func() { results = append(results, buf...) }()
		return fc(tx, batch)
	})
	return results, err
}

func (w workspaceSharingORMDo) FindInBatches(result *[]*workspace_servicev1.WorkspaceSharingORM, batchSize int, fc func(tx gen.Dao, batch int) error) error {
	return w.DO.FindInBatches(result, batchSize, fc)
}

func (w workspaceSharingORMDo) Attrs(attrs ...field.AssignExpr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Attrs(attrs...))
}

func (w workspaceSharingORMDo) Assign(attrs ...field.AssignExpr) IWorkspaceSharingORMDo {
	return w.withDO(w.DO.Assign(attrs...))
}

func (w workspaceSharingORMDo) Joins(fields ...field.RelationField) IWorkspaceSharingORMDo {
	for _, _f := range fields {
		w = *w.withDO(w.DO.Joins(_f))
	}
	return &w
}

func (w workspaceSharingORMDo) Preload(fields ...field.RelationField) IWorkspaceSharingORMDo {
	for _, _f := range fields {
		w = *w.withDO(w.DO.Preload(_f))
	}
	return &w
}

func (w workspaceSharingORMDo) FirstOrInit() (*workspace_servicev1.WorkspaceSharingORM, error) {
	if result, err := w.DO.FirstOrInit(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceSharingORM), nil
	}
}

func (w workspaceSharingORMDo) FirstOrCreate() (*workspace_servicev1.WorkspaceSharingORM, error) {
	if result, err := w.DO.FirstOrCreate(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceSharingORM), nil
	}
}

func (w workspaceSharingORMDo) FindByPage(offset int, limit int) (result []*workspace_servicev1.WorkspaceSharingORM, count int64, err error) {
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

func (w workspaceSharingORMDo) ScanByPage(result interface{}, offset int, limit int) (count int64, err error) {
	count, err = w.Count()
	if err != nil {
		return
	}

	err = w.Offset(offset).Limit(limit).Scan(result)
	return
}

func (w workspaceSharingORMDo) Scan(result interface{}) (err error) {
	return w.DO.Scan(result)
}

func (w workspaceSharingORMDo) Delete(models ...*workspace_servicev1.WorkspaceSharingORM) (result gen.ResultInfo, err error) {
	return w.DO.Delete(models)
}

func (w *workspaceSharingORMDo) withDO(do gen.Dao) *workspaceSharingORMDo {
	w.DO = *do.(*gen.DO)
	return w
}
