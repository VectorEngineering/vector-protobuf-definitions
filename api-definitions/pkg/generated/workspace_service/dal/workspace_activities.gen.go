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

func newWorkspaceActivityORM(db *gorm.DB, opts ...gen.DOOption) workspaceActivityORM {
	_workspaceActivityORM := workspaceActivityORM{}

	_workspaceActivityORM.workspaceActivityORMDo.UseDB(db, opts...)
	_workspaceActivityORM.workspaceActivityORMDo.UseModel(&workspace_servicev1.WorkspaceActivityORM{})

	tableName := _workspaceActivityORM.workspaceActivityORMDo.TableName()
	_workspaceActivityORM.ALL = field.NewAsterisk(tableName)
	_workspaceActivityORM.ActivityType = field.NewString(tableName, "activity_type")
	_workspaceActivityORM.CreatedAt = field.NewTime(tableName, "created_at")
	_workspaceActivityORM.Description = field.NewString(tableName, "description")
	_workspaceActivityORM.Id = field.NewUint64(tableName, "id")
	_workspaceActivityORM.UserId = field.NewString(tableName, "user_id")
	_workspaceActivityORM.WorkspaceId = field.NewUint64(tableName, "workspace_id")

	_workspaceActivityORM.fillFieldMap()

	return _workspaceActivityORM
}

type workspaceActivityORM struct {
	workspaceActivityORMDo

	ALL          field.Asterisk
	ActivityType field.String
	CreatedAt    field.Time
	Description  field.String
	Id           field.Uint64
	UserId       field.String
	WorkspaceId  field.Uint64

	fieldMap map[string]field.Expr
}

func (w workspaceActivityORM) Table(newTableName string) *workspaceActivityORM {
	w.workspaceActivityORMDo.UseTable(newTableName)
	return w.updateTableName(newTableName)
}

func (w workspaceActivityORM) As(alias string) *workspaceActivityORM {
	w.workspaceActivityORMDo.DO = *(w.workspaceActivityORMDo.As(alias).(*gen.DO))
	return w.updateTableName(alias)
}

func (w *workspaceActivityORM) updateTableName(table string) *workspaceActivityORM {
	w.ALL = field.NewAsterisk(table)
	w.ActivityType = field.NewString(table, "activity_type")
	w.CreatedAt = field.NewTime(table, "created_at")
	w.Description = field.NewString(table, "description")
	w.Id = field.NewUint64(table, "id")
	w.UserId = field.NewString(table, "user_id")
	w.WorkspaceId = field.NewUint64(table, "workspace_id")

	w.fillFieldMap()

	return w
}

func (w *workspaceActivityORM) GetFieldByName(fieldName string) (field.OrderExpr, bool) {
	_f, ok := w.fieldMap[fieldName]
	if !ok || _f == nil {
		return nil, false
	}
	_oe, ok := _f.(field.OrderExpr)
	return _oe, ok
}

func (w *workspaceActivityORM) fillFieldMap() {
	w.fieldMap = make(map[string]field.Expr, 6)
	w.fieldMap["activity_type"] = w.ActivityType
	w.fieldMap["created_at"] = w.CreatedAt
	w.fieldMap["description"] = w.Description
	w.fieldMap["id"] = w.Id
	w.fieldMap["user_id"] = w.UserId
	w.fieldMap["workspace_id"] = w.WorkspaceId
}

func (w workspaceActivityORM) clone(db *gorm.DB) workspaceActivityORM {
	w.workspaceActivityORMDo.ReplaceConnPool(db.Statement.ConnPool)
	return w
}

func (w workspaceActivityORM) replaceDB(db *gorm.DB) workspaceActivityORM {
	w.workspaceActivityORMDo.ReplaceDB(db)
	return w
}

type workspaceActivityORMDo struct{ gen.DO }

type IWorkspaceActivityORMDo interface {
	gen.SubQuery
	Debug() IWorkspaceActivityORMDo
	WithContext(ctx context.Context) IWorkspaceActivityORMDo
	WithResult(fc func(tx gen.Dao)) gen.ResultInfo
	ReplaceDB(db *gorm.DB)
	ReadDB() IWorkspaceActivityORMDo
	WriteDB() IWorkspaceActivityORMDo
	As(alias string) gen.Dao
	Session(config *gorm.Session) IWorkspaceActivityORMDo
	Columns(cols ...field.Expr) gen.Columns
	Clauses(conds ...clause.Expression) IWorkspaceActivityORMDo
	Not(conds ...gen.Condition) IWorkspaceActivityORMDo
	Or(conds ...gen.Condition) IWorkspaceActivityORMDo
	Select(conds ...field.Expr) IWorkspaceActivityORMDo
	Where(conds ...gen.Condition) IWorkspaceActivityORMDo
	Order(conds ...field.Expr) IWorkspaceActivityORMDo
	Distinct(cols ...field.Expr) IWorkspaceActivityORMDo
	Omit(cols ...field.Expr) IWorkspaceActivityORMDo
	Join(table schema.Tabler, on ...field.Expr) IWorkspaceActivityORMDo
	LeftJoin(table schema.Tabler, on ...field.Expr) IWorkspaceActivityORMDo
	RightJoin(table schema.Tabler, on ...field.Expr) IWorkspaceActivityORMDo
	Group(cols ...field.Expr) IWorkspaceActivityORMDo
	Having(conds ...gen.Condition) IWorkspaceActivityORMDo
	Limit(limit int) IWorkspaceActivityORMDo
	Offset(offset int) IWorkspaceActivityORMDo
	Count() (count int64, err error)
	Scopes(funcs ...func(gen.Dao) gen.Dao) IWorkspaceActivityORMDo
	Unscoped() IWorkspaceActivityORMDo
	Create(values ...*workspace_servicev1.WorkspaceActivityORM) error
	CreateInBatches(values []*workspace_servicev1.WorkspaceActivityORM, batchSize int) error
	Save(values ...*workspace_servicev1.WorkspaceActivityORM) error
	First() (*workspace_servicev1.WorkspaceActivityORM, error)
	Take() (*workspace_servicev1.WorkspaceActivityORM, error)
	Last() (*workspace_servicev1.WorkspaceActivityORM, error)
	Find() ([]*workspace_servicev1.WorkspaceActivityORM, error)
	FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.WorkspaceActivityORM, err error)
	FindInBatches(result *[]*workspace_servicev1.WorkspaceActivityORM, batchSize int, fc func(tx gen.Dao, batch int) error) error
	Pluck(column field.Expr, dest interface{}) error
	Delete(...*workspace_servicev1.WorkspaceActivityORM) (info gen.ResultInfo, err error)
	Update(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	Updates(value interface{}) (info gen.ResultInfo, err error)
	UpdateColumn(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateColumnSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	UpdateColumns(value interface{}) (info gen.ResultInfo, err error)
	UpdateFrom(q gen.SubQuery) gen.Dao
	Attrs(attrs ...field.AssignExpr) IWorkspaceActivityORMDo
	Assign(attrs ...field.AssignExpr) IWorkspaceActivityORMDo
	Joins(fields ...field.RelationField) IWorkspaceActivityORMDo
	Preload(fields ...field.RelationField) IWorkspaceActivityORMDo
	FirstOrInit() (*workspace_servicev1.WorkspaceActivityORM, error)
	FirstOrCreate() (*workspace_servicev1.WorkspaceActivityORM, error)
	FindByPage(offset int, limit int) (result []*workspace_servicev1.WorkspaceActivityORM, count int64, err error)
	ScanByPage(result interface{}, offset int, limit int) (count int64, err error)
	Scan(result interface{}) (err error)
	Returning(value interface{}, columns ...string) IWorkspaceActivityORMDo
	UnderlyingDB() *gorm.DB
	schema.Tabler

	GetRecordByID(id uint64) (result workspace_servicev1.WorkspaceActivityORM, err error)
	GetRecordByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	DeleteRecordByID(id uint64) (err error)
	GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	CountAll() (result int, err error)
	GetByID(id uint64) (result workspace_servicev1.WorkspaceActivityORM, err error)
	GetByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	GetDeleted() (result []workspace_servicev1.WorkspaceActivityORM, err error)
	SoftDelete(id uint64) (err error)
	Restore(id uint64) (err error)
	DeleteInBatch(ids []uint64, batchSize int) (err error)
	GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	FindByPattern(columnName string, pattern string) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	GetDistinct(columnName string) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	Touch(id uint64) (err error)
	Exists(column string, value interface{}) (result bool, err error)
	ExistsById(id uint64) (result bool, err error)
	FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.WorkspaceActivityORM, err error)
	ExistsAndGet(column string, value interface{}) (result workspace_servicev1.WorkspaceActivityORM, err error)
	ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.WorkspaceActivityORM, err error)
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (w workspaceActivityORMDo) GetRecordByID(id uint64) (result workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetRecordByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) DeleteRecordByID(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("SELECT * FROM workspace_activities ORDER BY " + w.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = w.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// Additional Operations
// SELECT COUNT(*) FROM @@table
func (w workspaceActivityORMDo) CountAll() (result int, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("Additional Operations SELECT COUNT(*) FROM workspace_activities ")

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
func (w workspaceActivityORMDo) GetByID(id uint64) (result workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetByIDs(ids []uint64) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetDeleted() (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) SoftDelete(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_activities SET deleted_at=CURRENT_TIMESTAMP ")
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
func (w workspaceActivityORMDo) Restore(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_activities SET deleted_at=NULL ")
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
func (w workspaceActivityORMDo) DeleteInBatch(ids []uint64, batchSize int) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) FindByPattern(columnName string, pattern string) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) GetDistinct(columnName string) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT DISTINCT column_name FROM workspace_activities ")

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
func (w workspaceActivityORMDo) FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) Touch(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE workspace_activities SET updated_at=CURRENT_TIMESTAMP ")
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
func (w workspaceActivityORMDo) Exists(column string, value interface{}) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_activities ")
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
func (w workspaceActivityORMDo) ExistsById(id uint64) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_activities ")
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
func (w workspaceActivityORMDo) FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM workspace_activities ")
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
func (w workspaceActivityORMDo) ExistsAndGet(column string, value interface{}) (result workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_activities ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM workspace_activities ")
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
func (w workspaceActivityORMDo) ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.WorkspaceActivityORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM workspace_activities ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM workspace_activities ")
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

func (w workspaceActivityORMDo) Debug() IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Debug())
}

func (w workspaceActivityORMDo) WithContext(ctx context.Context) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.WithContext(ctx))
}

func (w workspaceActivityORMDo) ReadDB() IWorkspaceActivityORMDo {
	return w.Clauses(dbresolver.Read)
}

func (w workspaceActivityORMDo) WriteDB() IWorkspaceActivityORMDo {
	return w.Clauses(dbresolver.Write)
}

func (w workspaceActivityORMDo) Session(config *gorm.Session) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Session(config))
}

func (w workspaceActivityORMDo) Clauses(conds ...clause.Expression) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Clauses(conds...))
}

func (w workspaceActivityORMDo) Returning(value interface{}, columns ...string) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Returning(value, columns...))
}

func (w workspaceActivityORMDo) Not(conds ...gen.Condition) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Not(conds...))
}

func (w workspaceActivityORMDo) Or(conds ...gen.Condition) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Or(conds...))
}

func (w workspaceActivityORMDo) Select(conds ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Select(conds...))
}

func (w workspaceActivityORMDo) Where(conds ...gen.Condition) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Where(conds...))
}

func (w workspaceActivityORMDo) Order(conds ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Order(conds...))
}

func (w workspaceActivityORMDo) Distinct(cols ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Distinct(cols...))
}

func (w workspaceActivityORMDo) Omit(cols ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Omit(cols...))
}

func (w workspaceActivityORMDo) Join(table schema.Tabler, on ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Join(table, on...))
}

func (w workspaceActivityORMDo) LeftJoin(table schema.Tabler, on ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.LeftJoin(table, on...))
}

func (w workspaceActivityORMDo) RightJoin(table schema.Tabler, on ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.RightJoin(table, on...))
}

func (w workspaceActivityORMDo) Group(cols ...field.Expr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Group(cols...))
}

func (w workspaceActivityORMDo) Having(conds ...gen.Condition) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Having(conds...))
}

func (w workspaceActivityORMDo) Limit(limit int) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Limit(limit))
}

func (w workspaceActivityORMDo) Offset(offset int) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Offset(offset))
}

func (w workspaceActivityORMDo) Scopes(funcs ...func(gen.Dao) gen.Dao) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Scopes(funcs...))
}

func (w workspaceActivityORMDo) Unscoped() IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Unscoped())
}

func (w workspaceActivityORMDo) Create(values ...*workspace_servicev1.WorkspaceActivityORM) error {
	if len(values) == 0 {
		return nil
	}
	return w.DO.Create(values)
}

func (w workspaceActivityORMDo) CreateInBatches(values []*workspace_servicev1.WorkspaceActivityORM, batchSize int) error {
	return w.DO.CreateInBatches(values, batchSize)
}

// Save : !!! underlying implementation is different with GORM
// The method is equivalent to executing the statement: db.Clauses(clause.OnConflict{UpdateAll: true}).Create(values)
func (w workspaceActivityORMDo) Save(values ...*workspace_servicev1.WorkspaceActivityORM) error {
	if len(values) == 0 {
		return nil
	}
	return w.DO.Save(values)
}

func (w workspaceActivityORMDo) First() (*workspace_servicev1.WorkspaceActivityORM, error) {
	if result, err := w.DO.First(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceActivityORM), nil
	}
}

func (w workspaceActivityORMDo) Take() (*workspace_servicev1.WorkspaceActivityORM, error) {
	if result, err := w.DO.Take(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceActivityORM), nil
	}
}

func (w workspaceActivityORMDo) Last() (*workspace_servicev1.WorkspaceActivityORM, error) {
	if result, err := w.DO.Last(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceActivityORM), nil
	}
}

func (w workspaceActivityORMDo) Find() ([]*workspace_servicev1.WorkspaceActivityORM, error) {
	result, err := w.DO.Find()
	return result.([]*workspace_servicev1.WorkspaceActivityORM), err
}

func (w workspaceActivityORMDo) FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.WorkspaceActivityORM, err error) {
	buf := make([]*workspace_servicev1.WorkspaceActivityORM, 0, batchSize)
	err = w.DO.FindInBatches(&buf, batchSize, func(tx gen.Dao, batch int) error {
		defer func() { results = append(results, buf...) }()
		return fc(tx, batch)
	})
	return results, err
}

func (w workspaceActivityORMDo) FindInBatches(result *[]*workspace_servicev1.WorkspaceActivityORM, batchSize int, fc func(tx gen.Dao, batch int) error) error {
	return w.DO.FindInBatches(result, batchSize, fc)
}

func (w workspaceActivityORMDo) Attrs(attrs ...field.AssignExpr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Attrs(attrs...))
}

func (w workspaceActivityORMDo) Assign(attrs ...field.AssignExpr) IWorkspaceActivityORMDo {
	return w.withDO(w.DO.Assign(attrs...))
}

func (w workspaceActivityORMDo) Joins(fields ...field.RelationField) IWorkspaceActivityORMDo {
	for _, _f := range fields {
		w = *w.withDO(w.DO.Joins(_f))
	}
	return &w
}

func (w workspaceActivityORMDo) Preload(fields ...field.RelationField) IWorkspaceActivityORMDo {
	for _, _f := range fields {
		w = *w.withDO(w.DO.Preload(_f))
	}
	return &w
}

func (w workspaceActivityORMDo) FirstOrInit() (*workspace_servicev1.WorkspaceActivityORM, error) {
	if result, err := w.DO.FirstOrInit(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceActivityORM), nil
	}
}

func (w workspaceActivityORMDo) FirstOrCreate() (*workspace_servicev1.WorkspaceActivityORM, error) {
	if result, err := w.DO.FirstOrCreate(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.WorkspaceActivityORM), nil
	}
}

func (w workspaceActivityORMDo) FindByPage(offset int, limit int) (result []*workspace_servicev1.WorkspaceActivityORM, count int64, err error) {
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

func (w workspaceActivityORMDo) ScanByPage(result interface{}, offset int, limit int) (count int64, err error) {
	count, err = w.Count()
	if err != nil {
		return
	}

	err = w.Offset(offset).Limit(limit).Scan(result)
	return
}

func (w workspaceActivityORMDo) Scan(result interface{}) (err error) {
	return w.DO.Scan(result)
}

func (w workspaceActivityORMDo) Delete(models ...*workspace_servicev1.WorkspaceActivityORM) (result gen.ResultInfo, err error) {
	return w.DO.Delete(models)
}

func (w *workspaceActivityORMDo) withDO(do gen.Dao) *workspaceActivityORMDo {
	w.DO = *do.(*gen.DO)
	return w
}
