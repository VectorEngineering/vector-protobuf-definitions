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

func newSignatureBlockORM(db *gorm.DB, opts ...gen.DOOption) signatureBlockORM {
	_signatureBlockORM := signatureBlockORM{}

	_signatureBlockORM.signatureBlockORMDo.UseDB(db, opts...)
	_signatureBlockORM.signatureBlockORMDo.UseModel(&workspace_servicev1.SignatureBlockORM{})

	tableName := _signatureBlockORM.signatureBlockORMDo.TableName()
	_signatureBlockORM.ALL = field.NewAsterisk(tableName)
	_signatureBlockORM.BlockType = field.NewString(tableName, "block_type")
	_signatureBlockORM.Height = field.NewFloat32(tableName, "height")
	_signatureBlockORM.Id = field.NewUint64(tableName, "id")
	_signatureBlockORM.IsRequired = field.NewBool(tableName, "is_required")
	_signatureBlockORM.PageNumber = field.NewInt32(tableName, "page_number")
	_signatureBlockORM.RequestId = field.NewUint64(tableName, "request_id")
	_signatureBlockORM.SignatureData = field.NewString(tableName, "signature_data")
	_signatureBlockORM.SignatureRequestId = field.NewUint64(tableName, "signature_request_id")
	_signatureBlockORM.SignedAt = field.NewTime(tableName, "signed_at")
	_signatureBlockORM.Width = field.NewFloat32(tableName, "width")
	_signatureBlockORM.XPosition = field.NewFloat32(tableName, "x_position")
	_signatureBlockORM.YPosition = field.NewFloat32(tableName, "y_position")

	_signatureBlockORM.fillFieldMap()

	return _signatureBlockORM
}

type signatureBlockORM struct {
	signatureBlockORMDo

	ALL                field.Asterisk
	BlockType          field.String
	Height             field.Float32
	Id                 field.Uint64
	IsRequired         field.Bool
	PageNumber         field.Int32
	RequestId          field.Uint64
	SignatureData      field.String
	SignatureRequestId field.Uint64
	SignedAt           field.Time
	Width              field.Float32
	XPosition          field.Float32
	YPosition          field.Float32

	fieldMap map[string]field.Expr
}

func (s signatureBlockORM) Table(newTableName string) *signatureBlockORM {
	s.signatureBlockORMDo.UseTable(newTableName)
	return s.updateTableName(newTableName)
}

func (s signatureBlockORM) As(alias string) *signatureBlockORM {
	s.signatureBlockORMDo.DO = *(s.signatureBlockORMDo.As(alias).(*gen.DO))
	return s.updateTableName(alias)
}

func (s *signatureBlockORM) updateTableName(table string) *signatureBlockORM {
	s.ALL = field.NewAsterisk(table)
	s.BlockType = field.NewString(table, "block_type")
	s.Height = field.NewFloat32(table, "height")
	s.Id = field.NewUint64(table, "id")
	s.IsRequired = field.NewBool(table, "is_required")
	s.PageNumber = field.NewInt32(table, "page_number")
	s.RequestId = field.NewUint64(table, "request_id")
	s.SignatureData = field.NewString(table, "signature_data")
	s.SignatureRequestId = field.NewUint64(table, "signature_request_id")
	s.SignedAt = field.NewTime(table, "signed_at")
	s.Width = field.NewFloat32(table, "width")
	s.XPosition = field.NewFloat32(table, "x_position")
	s.YPosition = field.NewFloat32(table, "y_position")

	s.fillFieldMap()

	return s
}

func (s *signatureBlockORM) GetFieldByName(fieldName string) (field.OrderExpr, bool) {
	_f, ok := s.fieldMap[fieldName]
	if !ok || _f == nil {
		return nil, false
	}
	_oe, ok := _f.(field.OrderExpr)
	return _oe, ok
}

func (s *signatureBlockORM) fillFieldMap() {
	s.fieldMap = make(map[string]field.Expr, 12)
	s.fieldMap["block_type"] = s.BlockType
	s.fieldMap["height"] = s.Height
	s.fieldMap["id"] = s.Id
	s.fieldMap["is_required"] = s.IsRequired
	s.fieldMap["page_number"] = s.PageNumber
	s.fieldMap["request_id"] = s.RequestId
	s.fieldMap["signature_data"] = s.SignatureData
	s.fieldMap["signature_request_id"] = s.SignatureRequestId
	s.fieldMap["signed_at"] = s.SignedAt
	s.fieldMap["width"] = s.Width
	s.fieldMap["x_position"] = s.XPosition
	s.fieldMap["y_position"] = s.YPosition
}

func (s signatureBlockORM) clone(db *gorm.DB) signatureBlockORM {
	s.signatureBlockORMDo.ReplaceConnPool(db.Statement.ConnPool)
	return s
}

func (s signatureBlockORM) replaceDB(db *gorm.DB) signatureBlockORM {
	s.signatureBlockORMDo.ReplaceDB(db)
	return s
}

type signatureBlockORMDo struct{ gen.DO }

type ISignatureBlockORMDo interface {
	gen.SubQuery
	Debug() ISignatureBlockORMDo
	WithContext(ctx context.Context) ISignatureBlockORMDo
	WithResult(fc func(tx gen.Dao)) gen.ResultInfo
	ReplaceDB(db *gorm.DB)
	ReadDB() ISignatureBlockORMDo
	WriteDB() ISignatureBlockORMDo
	As(alias string) gen.Dao
	Session(config *gorm.Session) ISignatureBlockORMDo
	Columns(cols ...field.Expr) gen.Columns
	Clauses(conds ...clause.Expression) ISignatureBlockORMDo
	Not(conds ...gen.Condition) ISignatureBlockORMDo
	Or(conds ...gen.Condition) ISignatureBlockORMDo
	Select(conds ...field.Expr) ISignatureBlockORMDo
	Where(conds ...gen.Condition) ISignatureBlockORMDo
	Order(conds ...field.Expr) ISignatureBlockORMDo
	Distinct(cols ...field.Expr) ISignatureBlockORMDo
	Omit(cols ...field.Expr) ISignatureBlockORMDo
	Join(table schema.Tabler, on ...field.Expr) ISignatureBlockORMDo
	LeftJoin(table schema.Tabler, on ...field.Expr) ISignatureBlockORMDo
	RightJoin(table schema.Tabler, on ...field.Expr) ISignatureBlockORMDo
	Group(cols ...field.Expr) ISignatureBlockORMDo
	Having(conds ...gen.Condition) ISignatureBlockORMDo
	Limit(limit int) ISignatureBlockORMDo
	Offset(offset int) ISignatureBlockORMDo
	Count() (count int64, err error)
	Scopes(funcs ...func(gen.Dao) gen.Dao) ISignatureBlockORMDo
	Unscoped() ISignatureBlockORMDo
	Create(values ...*workspace_servicev1.SignatureBlockORM) error
	CreateInBatches(values []*workspace_servicev1.SignatureBlockORM, batchSize int) error
	Save(values ...*workspace_servicev1.SignatureBlockORM) error
	First() (*workspace_servicev1.SignatureBlockORM, error)
	Take() (*workspace_servicev1.SignatureBlockORM, error)
	Last() (*workspace_servicev1.SignatureBlockORM, error)
	Find() ([]*workspace_servicev1.SignatureBlockORM, error)
	FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.SignatureBlockORM, err error)
	FindInBatches(result *[]*workspace_servicev1.SignatureBlockORM, batchSize int, fc func(tx gen.Dao, batch int) error) error
	Pluck(column field.Expr, dest interface{}) error
	Delete(...*workspace_servicev1.SignatureBlockORM) (info gen.ResultInfo, err error)
	Update(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	Updates(value interface{}) (info gen.ResultInfo, err error)
	UpdateColumn(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateColumnSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	UpdateColumns(value interface{}) (info gen.ResultInfo, err error)
	UpdateFrom(q gen.SubQuery) gen.Dao
	Attrs(attrs ...field.AssignExpr) ISignatureBlockORMDo
	Assign(attrs ...field.AssignExpr) ISignatureBlockORMDo
	Joins(fields ...field.RelationField) ISignatureBlockORMDo
	Preload(fields ...field.RelationField) ISignatureBlockORMDo
	FirstOrInit() (*workspace_servicev1.SignatureBlockORM, error)
	FirstOrCreate() (*workspace_servicev1.SignatureBlockORM, error)
	FindByPage(offset int, limit int) (result []*workspace_servicev1.SignatureBlockORM, count int64, err error)
	ScanByPage(result interface{}, offset int, limit int) (count int64, err error)
	Scan(result interface{}) (err error)
	Returning(value interface{}, columns ...string) ISignatureBlockORMDo
	UnderlyingDB() *gorm.DB
	schema.Tabler

	GetRecordByID(id uint64) (result workspace_servicev1.SignatureBlockORM, err error)
	GetRecordByIDs(ids []uint64) (result []workspace_servicev1.SignatureBlockORM, err error)
	DeleteRecordByID(id uint64) (err error)
	GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.SignatureBlockORM, err error)
	CountAll() (result int, err error)
	GetByID(id uint64) (result workspace_servicev1.SignatureBlockORM, err error)
	GetByIDs(ids []uint64) (result []workspace_servicev1.SignatureBlockORM, err error)
	GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.SignatureBlockORM, err error)
	GetDeleted() (result []workspace_servicev1.SignatureBlockORM, err error)
	SoftDelete(id uint64) (err error)
	Restore(id uint64) (err error)
	DeleteInBatch(ids []uint64, batchSize int) (err error)
	GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.SignatureBlockORM, err error)
	FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.SignatureBlockORM, err error)
	FindByPattern(columnName string, pattern string) (result []workspace_servicev1.SignatureBlockORM, err error)
	GetDistinct(columnName string) (result []workspace_servicev1.SignatureBlockORM, err error)
	FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.SignatureBlockORM, err error)
	Touch(id uint64) (err error)
	Exists(column string, value interface{}) (result bool, err error)
	ExistsById(id uint64) (result bool, err error)
	FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.SignatureBlockORM, err error)
	ExistsAndGet(column string, value interface{}) (result workspace_servicev1.SignatureBlockORM, err error)
	ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.SignatureBlockORM, err error)
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (s signatureBlockORMDo) GetRecordByID(id uint64) (result workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (s signatureBlockORMDo) GetRecordByIDs(ids []uint64) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// DELETE FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (s signatureBlockORMDo) DeleteRecordByID(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// ORDER BY @@orderColumn
// LIMIT @limit
// OFFSET @offset
func (s signatureBlockORMDo) GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("SELECT * FROM signature_blocks ORDER BY " + s.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// Additional Operations
// SELECT COUNT(*) FROM @@table
func (s signatureBlockORMDo) CountAll() (result int, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("Additional Operations SELECT COUNT(*) FROM signature_blocks ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String()).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (s signatureBlockORMDo) GetByID(id uint64) (result workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (s signatureBlockORMDo) GetByIDs(ids []uint64) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
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
func (s signatureBlockORMDo) GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY " + s.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	deleted_at IS NOT NULL
//
// {{end}}
func (s signatureBlockORMDo) GetDeleted() (result []workspace_servicev1.SignatureBlockORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NOT NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (s signatureBlockORMDo) SoftDelete(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE signature_blocks SET deleted_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=NULL
// {{where}}
//
//	id=@id
//
// {{end}}
func (s signatureBlockORMDo) Restore(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE signature_blocks SET deleted_at=NULL ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
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
func (s signatureBlockORMDo) DeleteInBatch(ids []uint64, batchSize int) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, batchSize)
	generateSQL.WriteString("LIMIT ? ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	created_at BETWEEN @startTime AND @endTime
//
// {{end}}
func (s signatureBlockORMDo) GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, startTime)
	params = append(params, endTime)
	whereSQL0.WriteString("created_at BETWEEN ? AND ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name @operator @value
//
// {{end}}
func (s signatureBlockORMDo) FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, operator)
	params = append(params, value)
	whereSQL0.WriteString("column_name ?? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name LIKE @pattern
//
// {{end}}
func (s signatureBlockORMDo) FindByPattern(columnName string, pattern string) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, pattern)
	whereSQL0.WriteString("column_name LIKE ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT DISTINCT column_name FROM @@table
func (s signatureBlockORMDo) GetDistinct(columnName string) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT DISTINCT column_name FROM signature_blocks ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
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
func (s signatureBlockORMDo) FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, conditions)
	whereSQL0.WriteString("? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, orderBy)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY ? LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET updated_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (s signatureBlockORMDo) Touch(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE signature_blocks SET updated_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
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
func (s signatureBlockORMDo) Exists(column string, value interface{}) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
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
func (s signatureBlockORMDo) ExistsById(id uint64) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	@column IN (SELECT @foreignKey FROM @foreignTable)
//
// {{end}}
func (s signatureBlockORMDo) FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, foreignKey)
	params = append(params, foreignTable)
	whereSQL0.WriteString("? IN (SELECT ? FROM ?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
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
func (s signatureBlockORMDo) ExistsAndGet(column string, value interface{}) (result workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM signature_blocks ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
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
func (s signatureBlockORMDo) ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.SignatureBlockORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM signature_blocks ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM signature_blocks ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = s.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

func (s signatureBlockORMDo) Debug() ISignatureBlockORMDo {
	return s.withDO(s.DO.Debug())
}

func (s signatureBlockORMDo) WithContext(ctx context.Context) ISignatureBlockORMDo {
	return s.withDO(s.DO.WithContext(ctx))
}

func (s signatureBlockORMDo) ReadDB() ISignatureBlockORMDo {
	return s.Clauses(dbresolver.Read)
}

func (s signatureBlockORMDo) WriteDB() ISignatureBlockORMDo {
	return s.Clauses(dbresolver.Write)
}

func (s signatureBlockORMDo) Session(config *gorm.Session) ISignatureBlockORMDo {
	return s.withDO(s.DO.Session(config))
}

func (s signatureBlockORMDo) Clauses(conds ...clause.Expression) ISignatureBlockORMDo {
	return s.withDO(s.DO.Clauses(conds...))
}

func (s signatureBlockORMDo) Returning(value interface{}, columns ...string) ISignatureBlockORMDo {
	return s.withDO(s.DO.Returning(value, columns...))
}

func (s signatureBlockORMDo) Not(conds ...gen.Condition) ISignatureBlockORMDo {
	return s.withDO(s.DO.Not(conds...))
}

func (s signatureBlockORMDo) Or(conds ...gen.Condition) ISignatureBlockORMDo {
	return s.withDO(s.DO.Or(conds...))
}

func (s signatureBlockORMDo) Select(conds ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Select(conds...))
}

func (s signatureBlockORMDo) Where(conds ...gen.Condition) ISignatureBlockORMDo {
	return s.withDO(s.DO.Where(conds...))
}

func (s signatureBlockORMDo) Order(conds ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Order(conds...))
}

func (s signatureBlockORMDo) Distinct(cols ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Distinct(cols...))
}

func (s signatureBlockORMDo) Omit(cols ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Omit(cols...))
}

func (s signatureBlockORMDo) Join(table schema.Tabler, on ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Join(table, on...))
}

func (s signatureBlockORMDo) LeftJoin(table schema.Tabler, on ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.LeftJoin(table, on...))
}

func (s signatureBlockORMDo) RightJoin(table schema.Tabler, on ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.RightJoin(table, on...))
}

func (s signatureBlockORMDo) Group(cols ...field.Expr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Group(cols...))
}

func (s signatureBlockORMDo) Having(conds ...gen.Condition) ISignatureBlockORMDo {
	return s.withDO(s.DO.Having(conds...))
}

func (s signatureBlockORMDo) Limit(limit int) ISignatureBlockORMDo {
	return s.withDO(s.DO.Limit(limit))
}

func (s signatureBlockORMDo) Offset(offset int) ISignatureBlockORMDo {
	return s.withDO(s.DO.Offset(offset))
}

func (s signatureBlockORMDo) Scopes(funcs ...func(gen.Dao) gen.Dao) ISignatureBlockORMDo {
	return s.withDO(s.DO.Scopes(funcs...))
}

func (s signatureBlockORMDo) Unscoped() ISignatureBlockORMDo {
	return s.withDO(s.DO.Unscoped())
}

func (s signatureBlockORMDo) Create(values ...*workspace_servicev1.SignatureBlockORM) error {
	if len(values) == 0 {
		return nil
	}
	return s.DO.Create(values)
}

func (s signatureBlockORMDo) CreateInBatches(values []*workspace_servicev1.SignatureBlockORM, batchSize int) error {
	return s.DO.CreateInBatches(values, batchSize)
}

// Save : !!! underlying implementation is different with GORM
// The method is equivalent to executing the statement: db.Clauses(clause.OnConflict{UpdateAll: true}).Create(values)
func (s signatureBlockORMDo) Save(values ...*workspace_servicev1.SignatureBlockORM) error {
	if len(values) == 0 {
		return nil
	}
	return s.DO.Save(values)
}

func (s signatureBlockORMDo) First() (*workspace_servicev1.SignatureBlockORM, error) {
	if result, err := s.DO.First(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.SignatureBlockORM), nil
	}
}

func (s signatureBlockORMDo) Take() (*workspace_servicev1.SignatureBlockORM, error) {
	if result, err := s.DO.Take(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.SignatureBlockORM), nil
	}
}

func (s signatureBlockORMDo) Last() (*workspace_servicev1.SignatureBlockORM, error) {
	if result, err := s.DO.Last(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.SignatureBlockORM), nil
	}
}

func (s signatureBlockORMDo) Find() ([]*workspace_servicev1.SignatureBlockORM, error) {
	result, err := s.DO.Find()
	return result.([]*workspace_servicev1.SignatureBlockORM), err
}

func (s signatureBlockORMDo) FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.SignatureBlockORM, err error) {
	buf := make([]*workspace_servicev1.SignatureBlockORM, 0, batchSize)
	err = s.DO.FindInBatches(&buf, batchSize, func(tx gen.Dao, batch int) error {
		defer func() { results = append(results, buf...) }()
		return fc(tx, batch)
	})
	return results, err
}

func (s signatureBlockORMDo) FindInBatches(result *[]*workspace_servicev1.SignatureBlockORM, batchSize int, fc func(tx gen.Dao, batch int) error) error {
	return s.DO.FindInBatches(result, batchSize, fc)
}

func (s signatureBlockORMDo) Attrs(attrs ...field.AssignExpr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Attrs(attrs...))
}

func (s signatureBlockORMDo) Assign(attrs ...field.AssignExpr) ISignatureBlockORMDo {
	return s.withDO(s.DO.Assign(attrs...))
}

func (s signatureBlockORMDo) Joins(fields ...field.RelationField) ISignatureBlockORMDo {
	for _, _f := range fields {
		s = *s.withDO(s.DO.Joins(_f))
	}
	return &s
}

func (s signatureBlockORMDo) Preload(fields ...field.RelationField) ISignatureBlockORMDo {
	for _, _f := range fields {
		s = *s.withDO(s.DO.Preload(_f))
	}
	return &s
}

func (s signatureBlockORMDo) FirstOrInit() (*workspace_servicev1.SignatureBlockORM, error) {
	if result, err := s.DO.FirstOrInit(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.SignatureBlockORM), nil
	}
}

func (s signatureBlockORMDo) FirstOrCreate() (*workspace_servicev1.SignatureBlockORM, error) {
	if result, err := s.DO.FirstOrCreate(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.SignatureBlockORM), nil
	}
}

func (s signatureBlockORMDo) FindByPage(offset int, limit int) (result []*workspace_servicev1.SignatureBlockORM, count int64, err error) {
	result, err = s.Offset(offset).Limit(limit).Find()
	if err != nil {
		return
	}

	if size := len(result); 0 < limit && 0 < size && size < limit {
		count = int64(size + offset)
		return
	}

	count, err = s.Offset(-1).Limit(-1).Count()
	return
}

func (s signatureBlockORMDo) ScanByPage(result interface{}, offset int, limit int) (count int64, err error) {
	count, err = s.Count()
	if err != nil {
		return
	}

	err = s.Offset(offset).Limit(limit).Scan(result)
	return
}

func (s signatureBlockORMDo) Scan(result interface{}) (err error) {
	return s.DO.Scan(result)
}

func (s signatureBlockORMDo) Delete(models ...*workspace_servicev1.SignatureBlockORM) (result gen.ResultInfo, err error) {
	return s.DO.Delete(models)
}

func (s *signatureBlockORMDo) withDO(do gen.Dao) *signatureBlockORMDo {
	s.DO = *do.(*gen.DO)
	return s
}
