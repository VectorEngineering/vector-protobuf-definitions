// Code generated by gorm.io/gen. DO NOT EDIT.
// Code generated by gorm.io/gen. DO NOT EDIT.
// Code generated by gorm.io/gen. DO NOT EDIT.

package dal

import (
	"context"
	"strings"

	"gorm.io/gorm"
	"gorm.io/gorm/clause"
	"gorm.io/gorm/schema"

	"gorm.io/gen"
	"gorm.io/gen/field"
	"gorm.io/gen/helper"

	"gorm.io/plugin/dbresolver"

	workspace_servicev1 "github.com/VectorEngineering/vector-protobuf-definitions/api-definitions/pkg/generated/workspace_service/v1"

	"time"
)

func newDocumentSnapshotORM(db *gorm.DB, opts ...gen.DOOption) documentSnapshotORM {
	_documentSnapshotORM := documentSnapshotORM{}

	_documentSnapshotORM.documentSnapshotORMDo.UseDB(db, opts...)
	_documentSnapshotORM.documentSnapshotORMDo.UseModel(&workspace_servicev1.DocumentSnapshotORM{})

	tableName := _documentSnapshotORM.documentSnapshotORMDo.TableName()
	_documentSnapshotORM.ALL = field.NewAsterisk(tableName)
	_documentSnapshotORM.AuthorId = field.NewString(tableName, "author_id")
	_documentSnapshotORM.Content = field.NewBytes(tableName, "content")
	_documentSnapshotORM.CreatedAt = field.NewTime(tableName, "created_at")
	_documentSnapshotORM.FileId = field.NewUint64(tableName, "file_id")
	_documentSnapshotORM.Id = field.NewUint64(tableName, "id")
	_documentSnapshotORM.Reason = field.NewString(tableName, "reason")
	_documentSnapshotORM.SnapshotHash = field.NewString(tableName, "snapshot_hash")

	_documentSnapshotORM.fillFieldMap()

	return _documentSnapshotORM
}

type documentSnapshotORM struct {
	documentSnapshotORMDo

	ALL          field.Asterisk
	AuthorId     field.String
	Content      field.Bytes
	CreatedAt    field.Time
	FileId       field.Uint64
	Id           field.Uint64
	Reason       field.String
	SnapshotHash field.String

	fieldMap map[string]field.Expr
}

func (d documentSnapshotORM) Table(newTableName string) *documentSnapshotORM {
	d.documentSnapshotORMDo.UseTable(newTableName)
	return d.updateTableName(newTableName)
}

func (d documentSnapshotORM) As(alias string) *documentSnapshotORM {
	d.documentSnapshotORMDo.DO = *(d.documentSnapshotORMDo.As(alias).(*gen.DO))
	return d.updateTableName(alias)
}

func (d *documentSnapshotORM) updateTableName(table string) *documentSnapshotORM {
	d.ALL = field.NewAsterisk(table)
	d.AuthorId = field.NewString(table, "author_id")
	d.Content = field.NewBytes(table, "content")
	d.CreatedAt = field.NewTime(table, "created_at")
	d.FileId = field.NewUint64(table, "file_id")
	d.Id = field.NewUint64(table, "id")
	d.Reason = field.NewString(table, "reason")
	d.SnapshotHash = field.NewString(table, "snapshot_hash")

	d.fillFieldMap()

	return d
}

func (d *documentSnapshotORM) GetFieldByName(fieldName string) (field.OrderExpr, bool) {
	_f, ok := d.fieldMap[fieldName]
	if !ok || _f == nil {
		return nil, false
	}
	_oe, ok := _f.(field.OrderExpr)
	return _oe, ok
}

func (d *documentSnapshotORM) fillFieldMap() {
	d.fieldMap = make(map[string]field.Expr, 7)
	d.fieldMap["author_id"] = d.AuthorId
	d.fieldMap["content"] = d.Content
	d.fieldMap["created_at"] = d.CreatedAt
	d.fieldMap["file_id"] = d.FileId
	d.fieldMap["id"] = d.Id
	d.fieldMap["reason"] = d.Reason
	d.fieldMap["snapshot_hash"] = d.SnapshotHash
}

func (d documentSnapshotORM) clone(db *gorm.DB) documentSnapshotORM {
	d.documentSnapshotORMDo.ReplaceConnPool(db.Statement.ConnPool)
	return d
}

func (d documentSnapshotORM) replaceDB(db *gorm.DB) documentSnapshotORM {
	d.documentSnapshotORMDo.ReplaceDB(db)
	return d
}

type documentSnapshotORMDo struct{ gen.DO }

type IDocumentSnapshotORMDo interface {
	gen.SubQuery
	Debug() IDocumentSnapshotORMDo
	WithContext(ctx context.Context) IDocumentSnapshotORMDo
	WithResult(fc func(tx gen.Dao)) gen.ResultInfo
	ReplaceDB(db *gorm.DB)
	ReadDB() IDocumentSnapshotORMDo
	WriteDB() IDocumentSnapshotORMDo
	As(alias string) gen.Dao
	Session(config *gorm.Session) IDocumentSnapshotORMDo
	Columns(cols ...field.Expr) gen.Columns
	Clauses(conds ...clause.Expression) IDocumentSnapshotORMDo
	Not(conds ...gen.Condition) IDocumentSnapshotORMDo
	Or(conds ...gen.Condition) IDocumentSnapshotORMDo
	Select(conds ...field.Expr) IDocumentSnapshotORMDo
	Where(conds ...gen.Condition) IDocumentSnapshotORMDo
	Order(conds ...field.Expr) IDocumentSnapshotORMDo
	Distinct(cols ...field.Expr) IDocumentSnapshotORMDo
	Omit(cols ...field.Expr) IDocumentSnapshotORMDo
	Join(table schema.Tabler, on ...field.Expr) IDocumentSnapshotORMDo
	LeftJoin(table schema.Tabler, on ...field.Expr) IDocumentSnapshotORMDo
	RightJoin(table schema.Tabler, on ...field.Expr) IDocumentSnapshotORMDo
	Group(cols ...field.Expr) IDocumentSnapshotORMDo
	Having(conds ...gen.Condition) IDocumentSnapshotORMDo
	Limit(limit int) IDocumentSnapshotORMDo
	Offset(offset int) IDocumentSnapshotORMDo
	Count() (count int64, err error)
	Scopes(funcs ...func(gen.Dao) gen.Dao) IDocumentSnapshotORMDo
	Unscoped() IDocumentSnapshotORMDo
	Create(values ...*workspace_servicev1.DocumentSnapshotORM) error
	CreateInBatches(values []*workspace_servicev1.DocumentSnapshotORM, batchSize int) error
	Save(values ...*workspace_servicev1.DocumentSnapshotORM) error
	First() (*workspace_servicev1.DocumentSnapshotORM, error)
	Take() (*workspace_servicev1.DocumentSnapshotORM, error)
	Last() (*workspace_servicev1.DocumentSnapshotORM, error)
	Find() ([]*workspace_servicev1.DocumentSnapshotORM, error)
	FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.DocumentSnapshotORM, err error)
	FindInBatches(result *[]*workspace_servicev1.DocumentSnapshotORM, batchSize int, fc func(tx gen.Dao, batch int) error) error
	Pluck(column field.Expr, dest interface{}) error
	Delete(...*workspace_servicev1.DocumentSnapshotORM) (info gen.ResultInfo, err error)
	Update(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	Updates(value interface{}) (info gen.ResultInfo, err error)
	UpdateColumn(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateColumnSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	UpdateColumns(value interface{}) (info gen.ResultInfo, err error)
	UpdateFrom(q gen.SubQuery) gen.Dao
	Attrs(attrs ...field.AssignExpr) IDocumentSnapshotORMDo
	Assign(attrs ...field.AssignExpr) IDocumentSnapshotORMDo
	Joins(fields ...field.RelationField) IDocumentSnapshotORMDo
	Preload(fields ...field.RelationField) IDocumentSnapshotORMDo
	FirstOrInit() (*workspace_servicev1.DocumentSnapshotORM, error)
	FirstOrCreate() (*workspace_servicev1.DocumentSnapshotORM, error)
	FindByPage(offset int, limit int) (result []*workspace_servicev1.DocumentSnapshotORM, count int64, err error)
	ScanByPage(result interface{}, offset int, limit int) (count int64, err error)
	Scan(result interface{}) (err error)
	Returning(value interface{}, columns ...string) IDocumentSnapshotORMDo
	UnderlyingDB() *gorm.DB
	schema.Tabler

	GetRecordByID(id uint64) (result workspace_servicev1.DocumentSnapshotORM, err error)
	GetRecordByIDs(ids []uint64) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	DeleteRecordByID(id uint64) (err error)
	GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	CountAll() (result int, err error)
	GetByID(id uint64) (result workspace_servicev1.DocumentSnapshotORM, err error)
	GetByIDs(ids []uint64) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	GetDeleted() (result []workspace_servicev1.DocumentSnapshotORM, err error)
	SoftDelete(id uint64) (err error)
	Restore(id uint64) (err error)
	DeleteInBatch(ids []uint64, batchSize int) (err error)
	GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	FindByPattern(columnName string, pattern string) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	GetDistinct(columnName string) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	Touch(id uint64) (err error)
	Exists(column string, value interface{}) (result bool, err error)
	ExistsById(id uint64) (result bool, err error)
	FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.DocumentSnapshotORM, err error)
	ExistsAndGet(column string, value interface{}) (result workspace_servicev1.DocumentSnapshotORM, err error)
	ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.DocumentSnapshotORM, err error)
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (d documentSnapshotORMDo) GetRecordByID(id uint64) (result workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (d documentSnapshotORMDo) GetRecordByIDs(ids []uint64) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// DELETE FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (d documentSnapshotORMDo) DeleteRecordByID(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// ORDER BY @@orderColumn
// LIMIT @limit
// OFFSET @offset
func (d documentSnapshotORMDo) GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("SELECT * FROM document_snapshots ORDER BY " + d.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// Additional Operations
// SELECT COUNT(*) FROM @@table
func (d documentSnapshotORMDo) CountAll() (result int, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("Additional Operations SELECT COUNT(*) FROM document_snapshots ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String()).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (d documentSnapshotORMDo) GetByID(id uint64) (result workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (d documentSnapshotORMDo) GetByIDs(ids []uint64) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
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
func (d documentSnapshotORMDo) GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY " + d.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	deleted_at IS NOT NULL
//
// {{end}}
func (d documentSnapshotORMDo) GetDeleted() (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NOT NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (d documentSnapshotORMDo) SoftDelete(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE document_snapshots SET deleted_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=NULL
// {{where}}
//
//	id=@id
//
// {{end}}
func (d documentSnapshotORMDo) Restore(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE document_snapshots SET deleted_at=NULL ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
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
func (d documentSnapshotORMDo) DeleteInBatch(ids []uint64, batchSize int) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, batchSize)
	generateSQL.WriteString("LIMIT ? ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	created_at BETWEEN @startTime AND @endTime
//
// {{end}}
func (d documentSnapshotORMDo) GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, startTime)
	params = append(params, endTime)
	whereSQL0.WriteString("created_at BETWEEN ? AND ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name @operator @value
//
// {{end}}
func (d documentSnapshotORMDo) FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, operator)
	params = append(params, value)
	whereSQL0.WriteString("column_name ?? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name LIKE @pattern
//
// {{end}}
func (d documentSnapshotORMDo) FindByPattern(columnName string, pattern string) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, pattern)
	whereSQL0.WriteString("column_name LIKE ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT DISTINCT column_name FROM @@table
func (d documentSnapshotORMDo) GetDistinct(columnName string) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT DISTINCT column_name FROM document_snapshots ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
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
func (d documentSnapshotORMDo) FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, conditions)
	whereSQL0.WriteString("? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, orderBy)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY ? LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET updated_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (d documentSnapshotORMDo) Touch(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE document_snapshots SET updated_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
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
func (d documentSnapshotORMDo) Exists(column string, value interface{}) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
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
func (d documentSnapshotORMDo) ExistsById(id uint64) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	@column IN (SELECT @foreignKey FROM @foreignTable)
//
// {{end}}
func (d documentSnapshotORMDo) FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, foreignKey)
	params = append(params, foreignTable)
	whereSQL0.WriteString("? IN (SELECT ? FROM ?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
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
func (d documentSnapshotORMDo) ExistsAndGet(column string, value interface{}) (result workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM document_snapshots ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
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
func (d documentSnapshotORMDo) ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.DocumentSnapshotORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM document_snapshots ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM document_snapshots ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = d.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

func (d documentSnapshotORMDo) Debug() IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Debug())
}

func (d documentSnapshotORMDo) WithContext(ctx context.Context) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.WithContext(ctx))
}

func (d documentSnapshotORMDo) ReadDB() IDocumentSnapshotORMDo {
	return d.Clauses(dbresolver.Read)
}

func (d documentSnapshotORMDo) WriteDB() IDocumentSnapshotORMDo {
	return d.Clauses(dbresolver.Write)
}

func (d documentSnapshotORMDo) Session(config *gorm.Session) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Session(config))
}

func (d documentSnapshotORMDo) Clauses(conds ...clause.Expression) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Clauses(conds...))
}

func (d documentSnapshotORMDo) Returning(value interface{}, columns ...string) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Returning(value, columns...))
}

func (d documentSnapshotORMDo) Not(conds ...gen.Condition) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Not(conds...))
}

func (d documentSnapshotORMDo) Or(conds ...gen.Condition) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Or(conds...))
}

func (d documentSnapshotORMDo) Select(conds ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Select(conds...))
}

func (d documentSnapshotORMDo) Where(conds ...gen.Condition) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Where(conds...))
}

func (d documentSnapshotORMDo) Order(conds ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Order(conds...))
}

func (d documentSnapshotORMDo) Distinct(cols ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Distinct(cols...))
}

func (d documentSnapshotORMDo) Omit(cols ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Omit(cols...))
}

func (d documentSnapshotORMDo) Join(table schema.Tabler, on ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Join(table, on...))
}

func (d documentSnapshotORMDo) LeftJoin(table schema.Tabler, on ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.LeftJoin(table, on...))
}

func (d documentSnapshotORMDo) RightJoin(table schema.Tabler, on ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.RightJoin(table, on...))
}

func (d documentSnapshotORMDo) Group(cols ...field.Expr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Group(cols...))
}

func (d documentSnapshotORMDo) Having(conds ...gen.Condition) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Having(conds...))
}

func (d documentSnapshotORMDo) Limit(limit int) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Limit(limit))
}

func (d documentSnapshotORMDo) Offset(offset int) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Offset(offset))
}

func (d documentSnapshotORMDo) Scopes(funcs ...func(gen.Dao) gen.Dao) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Scopes(funcs...))
}

func (d documentSnapshotORMDo) Unscoped() IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Unscoped())
}

func (d documentSnapshotORMDo) Create(values ...*workspace_servicev1.DocumentSnapshotORM) error {
	if len(values) == 0 {
		return nil
	}
	return d.DO.Create(values)
}

func (d documentSnapshotORMDo) CreateInBatches(values []*workspace_servicev1.DocumentSnapshotORM, batchSize int) error {
	return d.DO.CreateInBatches(values, batchSize)
}

// Save : !!! underlying implementation is different with GORM
// The method is equivalent to executing the statement: db.Clauses(clause.OnConflict{UpdateAll: true}).Create(values)
func (d documentSnapshotORMDo) Save(values ...*workspace_servicev1.DocumentSnapshotORM) error {
	if len(values) == 0 {
		return nil
	}
	return d.DO.Save(values)
}

func (d documentSnapshotORMDo) First() (*workspace_servicev1.DocumentSnapshotORM, error) {
	if result, err := d.DO.First(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.DocumentSnapshotORM), nil
	}
}

func (d documentSnapshotORMDo) Take() (*workspace_servicev1.DocumentSnapshotORM, error) {
	if result, err := d.DO.Take(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.DocumentSnapshotORM), nil
	}
}

func (d documentSnapshotORMDo) Last() (*workspace_servicev1.DocumentSnapshotORM, error) {
	if result, err := d.DO.Last(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.DocumentSnapshotORM), nil
	}
}

func (d documentSnapshotORMDo) Find() ([]*workspace_servicev1.DocumentSnapshotORM, error) {
	result, err := d.DO.Find()
	return result.([]*workspace_servicev1.DocumentSnapshotORM), err
}

func (d documentSnapshotORMDo) FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.DocumentSnapshotORM, err error) {
	buf := make([]*workspace_servicev1.DocumentSnapshotORM, 0, batchSize)
	err = d.DO.FindInBatches(&buf, batchSize, func(tx gen.Dao, batch int) error {
		defer func() { results = append(results, buf...) }()
		return fc(tx, batch)
	})
	return results, err
}

func (d documentSnapshotORMDo) FindInBatches(result *[]*workspace_servicev1.DocumentSnapshotORM, batchSize int, fc func(tx gen.Dao, batch int) error) error {
	return d.DO.FindInBatches(result, batchSize, fc)
}

func (d documentSnapshotORMDo) Attrs(attrs ...field.AssignExpr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Attrs(attrs...))
}

func (d documentSnapshotORMDo) Assign(attrs ...field.AssignExpr) IDocumentSnapshotORMDo {
	return d.withDO(d.DO.Assign(attrs...))
}

func (d documentSnapshotORMDo) Joins(fields ...field.RelationField) IDocumentSnapshotORMDo {
	for _, _f := range fields {
		d = *d.withDO(d.DO.Joins(_f))
	}
	return &d
}

func (d documentSnapshotORMDo) Preload(fields ...field.RelationField) IDocumentSnapshotORMDo {
	for _, _f := range fields {
		d = *d.withDO(d.DO.Preload(_f))
	}
	return &d
}

func (d documentSnapshotORMDo) FirstOrInit() (*workspace_servicev1.DocumentSnapshotORM, error) {
	if result, err := d.DO.FirstOrInit(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.DocumentSnapshotORM), nil
	}
}

func (d documentSnapshotORMDo) FirstOrCreate() (*workspace_servicev1.DocumentSnapshotORM, error) {
	if result, err := d.DO.FirstOrCreate(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.DocumentSnapshotORM), nil
	}
}

func (d documentSnapshotORMDo) FindByPage(offset int, limit int) (result []*workspace_servicev1.DocumentSnapshotORM, count int64, err error) {
	result, err = d.Offset(offset).Limit(limit).Find()
	if err != nil {
		return
	}

	if size := len(result); 0 < limit && 0 < size && size < limit {
		count = int64(size + offset)
		return
	}

	count, err = d.Offset(-1).Limit(-1).Count()
	return
}

func (d documentSnapshotORMDo) ScanByPage(result interface{}, offset int, limit int) (count int64, err error) {
	count, err = d.Count()
	if err != nil {
		return
	}

	err = d.Offset(offset).Limit(limit).Scan(result)
	return
}

func (d documentSnapshotORMDo) Scan(result interface{}) (err error) {
	return d.DO.Scan(result)
}

func (d documentSnapshotORMDo) Delete(models ...*workspace_servicev1.DocumentSnapshotORM) (result gen.ResultInfo, err error) {
	return d.DO.Delete(models)
}

func (d *documentSnapshotORMDo) withDO(do gen.Dao) *documentSnapshotORMDo {
	d.DO = *do.(*gen.DO)
	return d
}
