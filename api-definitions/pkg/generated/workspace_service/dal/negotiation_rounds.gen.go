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

func newNegotiationRoundORM(db *gorm.DB, opts ...gen.DOOption) negotiationRoundORM {
	_negotiationRoundORM := negotiationRoundORM{}

	_negotiationRoundORM.negotiationRoundORMDo.UseDB(db, opts...)
	_negotiationRoundORM.negotiationRoundORMDo.UseModel(&workspace_servicev1.NegotiationRoundORM{})

	tableName := _negotiationRoundORM.negotiationRoundORMDo.TableName()
	_negotiationRoundORM.ALL = field.NewAsterisk(tableName)
	_negotiationRoundORM.CreatedAt = field.NewTime(tableName, "created_at")
	_negotiationRoundORM.HistoryId = field.NewUint64(tableName, "history_id")
	_negotiationRoundORM.Id = field.NewUint64(tableName, "id")
	_negotiationRoundORM.NegotiationHistoryId = field.NewUint64(tableName, "negotiation_history_id")
	_negotiationRoundORM.ProposalContent = field.NewString(tableName, "proposal_content")
	_negotiationRoundORM.ProposerId = field.NewString(tableName, "proposer_id")
	_negotiationRoundORM.ResponseContent = field.NewString(tableName, "response_content")
	_negotiationRoundORM.RoundNumber = field.NewInt32(tableName, "round_number")
	_negotiationRoundORM.Status = field.NewString(tableName, "status")

	_negotiationRoundORM.fillFieldMap()

	return _negotiationRoundORM
}

type negotiationRoundORM struct {
	negotiationRoundORMDo

	ALL                  field.Asterisk
	CreatedAt            field.Time
	HistoryId            field.Uint64
	Id                   field.Uint64
	NegotiationHistoryId field.Uint64
	ProposalContent      field.String
	ProposerId           field.String
	ResponseContent      field.String
	RoundNumber          field.Int32
	Status               field.String

	fieldMap map[string]field.Expr
}

func (n negotiationRoundORM) Table(newTableName string) *negotiationRoundORM {
	n.negotiationRoundORMDo.UseTable(newTableName)
	return n.updateTableName(newTableName)
}

func (n negotiationRoundORM) As(alias string) *negotiationRoundORM {
	n.negotiationRoundORMDo.DO = *(n.negotiationRoundORMDo.As(alias).(*gen.DO))
	return n.updateTableName(alias)
}

func (n *negotiationRoundORM) updateTableName(table string) *negotiationRoundORM {
	n.ALL = field.NewAsterisk(table)
	n.CreatedAt = field.NewTime(table, "created_at")
	n.HistoryId = field.NewUint64(table, "history_id")
	n.Id = field.NewUint64(table, "id")
	n.NegotiationHistoryId = field.NewUint64(table, "negotiation_history_id")
	n.ProposalContent = field.NewString(table, "proposal_content")
	n.ProposerId = field.NewString(table, "proposer_id")
	n.ResponseContent = field.NewString(table, "response_content")
	n.RoundNumber = field.NewInt32(table, "round_number")
	n.Status = field.NewString(table, "status")

	n.fillFieldMap()

	return n
}

func (n *negotiationRoundORM) GetFieldByName(fieldName string) (field.OrderExpr, bool) {
	_f, ok := n.fieldMap[fieldName]
	if !ok || _f == nil {
		return nil, false
	}
	_oe, ok := _f.(field.OrderExpr)
	return _oe, ok
}

func (n *negotiationRoundORM) fillFieldMap() {
	n.fieldMap = make(map[string]field.Expr, 9)
	n.fieldMap["created_at"] = n.CreatedAt
	n.fieldMap["history_id"] = n.HistoryId
	n.fieldMap["id"] = n.Id
	n.fieldMap["negotiation_history_id"] = n.NegotiationHistoryId
	n.fieldMap["proposal_content"] = n.ProposalContent
	n.fieldMap["proposer_id"] = n.ProposerId
	n.fieldMap["response_content"] = n.ResponseContent
	n.fieldMap["round_number"] = n.RoundNumber
	n.fieldMap["status"] = n.Status
}

func (n negotiationRoundORM) clone(db *gorm.DB) negotiationRoundORM {
	n.negotiationRoundORMDo.ReplaceConnPool(db.Statement.ConnPool)
	return n
}

func (n negotiationRoundORM) replaceDB(db *gorm.DB) negotiationRoundORM {
	n.negotiationRoundORMDo.ReplaceDB(db)
	return n
}

type negotiationRoundORMDo struct{ gen.DO }

type INegotiationRoundORMDo interface {
	gen.SubQuery
	Debug() INegotiationRoundORMDo
	WithContext(ctx context.Context) INegotiationRoundORMDo
	WithResult(fc func(tx gen.Dao)) gen.ResultInfo
	ReplaceDB(db *gorm.DB)
	ReadDB() INegotiationRoundORMDo
	WriteDB() INegotiationRoundORMDo
	As(alias string) gen.Dao
	Session(config *gorm.Session) INegotiationRoundORMDo
	Columns(cols ...field.Expr) gen.Columns
	Clauses(conds ...clause.Expression) INegotiationRoundORMDo
	Not(conds ...gen.Condition) INegotiationRoundORMDo
	Or(conds ...gen.Condition) INegotiationRoundORMDo
	Select(conds ...field.Expr) INegotiationRoundORMDo
	Where(conds ...gen.Condition) INegotiationRoundORMDo
	Order(conds ...field.Expr) INegotiationRoundORMDo
	Distinct(cols ...field.Expr) INegotiationRoundORMDo
	Omit(cols ...field.Expr) INegotiationRoundORMDo
	Join(table schema.Tabler, on ...field.Expr) INegotiationRoundORMDo
	LeftJoin(table schema.Tabler, on ...field.Expr) INegotiationRoundORMDo
	RightJoin(table schema.Tabler, on ...field.Expr) INegotiationRoundORMDo
	Group(cols ...field.Expr) INegotiationRoundORMDo
	Having(conds ...gen.Condition) INegotiationRoundORMDo
	Limit(limit int) INegotiationRoundORMDo
	Offset(offset int) INegotiationRoundORMDo
	Count() (count int64, err error)
	Scopes(funcs ...func(gen.Dao) gen.Dao) INegotiationRoundORMDo
	Unscoped() INegotiationRoundORMDo
	Create(values ...*workspace_servicev1.NegotiationRoundORM) error
	CreateInBatches(values []*workspace_servicev1.NegotiationRoundORM, batchSize int) error
	Save(values ...*workspace_servicev1.NegotiationRoundORM) error
	First() (*workspace_servicev1.NegotiationRoundORM, error)
	Take() (*workspace_servicev1.NegotiationRoundORM, error)
	Last() (*workspace_servicev1.NegotiationRoundORM, error)
	Find() ([]*workspace_servicev1.NegotiationRoundORM, error)
	FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.NegotiationRoundORM, err error)
	FindInBatches(result *[]*workspace_servicev1.NegotiationRoundORM, batchSize int, fc func(tx gen.Dao, batch int) error) error
	Pluck(column field.Expr, dest interface{}) error
	Delete(...*workspace_servicev1.NegotiationRoundORM) (info gen.ResultInfo, err error)
	Update(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	Updates(value interface{}) (info gen.ResultInfo, err error)
	UpdateColumn(column field.Expr, value interface{}) (info gen.ResultInfo, err error)
	UpdateColumnSimple(columns ...field.AssignExpr) (info gen.ResultInfo, err error)
	UpdateColumns(value interface{}) (info gen.ResultInfo, err error)
	UpdateFrom(q gen.SubQuery) gen.Dao
	Attrs(attrs ...field.AssignExpr) INegotiationRoundORMDo
	Assign(attrs ...field.AssignExpr) INegotiationRoundORMDo
	Joins(fields ...field.RelationField) INegotiationRoundORMDo
	Preload(fields ...field.RelationField) INegotiationRoundORMDo
	FirstOrInit() (*workspace_servicev1.NegotiationRoundORM, error)
	FirstOrCreate() (*workspace_servicev1.NegotiationRoundORM, error)
	FindByPage(offset int, limit int) (result []*workspace_servicev1.NegotiationRoundORM, count int64, err error)
	ScanByPage(result interface{}, offset int, limit int) (count int64, err error)
	Scan(result interface{}) (err error)
	Returning(value interface{}, columns ...string) INegotiationRoundORMDo
	UnderlyingDB() *gorm.DB
	schema.Tabler

	GetRecordByID(id uint64) (result workspace_servicev1.NegotiationRoundORM, err error)
	GetRecordByIDs(ids []uint64) (result []workspace_servicev1.NegotiationRoundORM, err error)
	DeleteRecordByID(id uint64) (err error)
	GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.NegotiationRoundORM, err error)
	CountAll() (result int, err error)
	GetByID(id uint64) (result workspace_servicev1.NegotiationRoundORM, err error)
	GetByIDs(ids []uint64) (result []workspace_servicev1.NegotiationRoundORM, err error)
	GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.NegotiationRoundORM, err error)
	GetDeleted() (result []workspace_servicev1.NegotiationRoundORM, err error)
	SoftDelete(id uint64) (err error)
	Restore(id uint64) (err error)
	DeleteInBatch(ids []uint64, batchSize int) (err error)
	GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.NegotiationRoundORM, err error)
	FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.NegotiationRoundORM, err error)
	FindByPattern(columnName string, pattern string) (result []workspace_servicev1.NegotiationRoundORM, err error)
	GetDistinct(columnName string) (result []workspace_servicev1.NegotiationRoundORM, err error)
	FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.NegotiationRoundORM, err error)
	Touch(id uint64) (err error)
	Exists(column string, value interface{}) (result bool, err error)
	ExistsById(id uint64) (result bool, err error)
	FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.NegotiationRoundORM, err error)
	ExistsAndGet(column string, value interface{}) (result workspace_servicev1.NegotiationRoundORM, err error)
	ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.NegotiationRoundORM, err error)
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (n negotiationRoundORMDo) GetRecordByID(id uint64) (result workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (n negotiationRoundORMDo) GetRecordByIDs(ids []uint64) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// DELETE FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (n negotiationRoundORMDo) DeleteRecordByID(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// ORDER BY @@orderColumn
// LIMIT @limit
// OFFSET @offset
func (n negotiationRoundORMDo) GetAllRecords(orderColumn string, limit int, offset int) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ORDER BY " + n.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// Additional Operations
// SELECT COUNT(*) FROM @@table
func (n negotiationRoundORMDo) CountAll() (result int, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("Additional Operations SELECT COUNT(*) FROM negotiation_rounds ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String()).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id=@id
//
// {{end}}
func (n negotiationRoundORMDo) GetByID(id uint64) (result workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	id IN (@ids)
//
// {{end}}
func (n negotiationRoundORMDo) GetByIDs(ids []uint64) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
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
func (n negotiationRoundORMDo) GetActivePaginated(orderColumn string, limit int, offset int) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY " + n.Quote(orderColumn) + " LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	deleted_at IS NOT NULL
//
// {{end}}
func (n negotiationRoundORMDo) GetDeleted() (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	whereSQL0.WriteString("deleted_at IS NOT NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (n negotiationRoundORMDo) SoftDelete(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE negotiation_rounds SET deleted_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET deleted_at=NULL
// {{where}}
//
//	id=@id
//
// {{end}}
func (n negotiationRoundORMDo) Restore(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE negotiation_rounds SET deleted_at=NULL ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
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
func (n negotiationRoundORMDo) DeleteInBatch(ids []uint64, batchSize int) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("DELETE FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, ids)
	whereSQL0.WriteString("id IN (?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, batchSize)
	generateSQL.WriteString("LIMIT ? ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	created_at BETWEEN @startTime AND @endTime
//
// {{end}}
func (n negotiationRoundORMDo) GetByTimeRange(startTime time.Time, endTime time.Time) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, startTime)
	params = append(params, endTime)
	whereSQL0.WriteString("created_at BETWEEN ? AND ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name @operator @value
//
// {{end}}
func (n negotiationRoundORMDo) FindBy(columnName string, operator string, value interface{}) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, operator)
	params = append(params, value)
	whereSQL0.WriteString("column_name ?? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	column_name LIKE @pattern
//
// {{end}}
func (n negotiationRoundORMDo) FindByPattern(columnName string, pattern string) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, pattern)
	whereSQL0.WriteString("column_name LIKE ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT DISTINCT column_name FROM @@table
func (n negotiationRoundORMDo) GetDistinct(columnName string) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT DISTINCT column_name FROM negotiation_rounds ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String()).Find(&result) // ignore_security_alert
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
func (n negotiationRoundORMDo) FindWithFilters(conditions map[string]interface{}, orderBy string, limit int, offset int) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, conditions)
	whereSQL0.WriteString("? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, orderBy)
	params = append(params, limit)
	params = append(params, offset)
	generateSQL.WriteString("ORDER BY ? LIMIT ? OFFSET ? ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// UPDATE @@table SET updated_at=CURRENT_TIMESTAMP
// {{where}}
//
//	id=@id
//
// {{end}}
func (n negotiationRoundORMDo) Touch(id uint64) (err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("UPDATE negotiation_rounds SET updated_at=CURRENT_TIMESTAMP ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id=? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Exec(generateSQL.String(), params...) // ignore_security_alert
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
func (n negotiationRoundORMDo) Exists(column string, value interface{}) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
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
func (n negotiationRoundORMDo) ExistsById(id uint64) (result bool, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, id)
	whereSQL0.WriteString("id = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	generateSQL.WriteString(") ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

// SELECT * FROM @@table
// {{where}}
//
//	@column IN (SELECT @foreignKey FROM @foreignTable)
//
// {{end}}
func (n negotiationRoundORMDo) FindBySubquery(column string, foreignTable string, foreignKey string) (result []workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT * FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, foreignKey)
	params = append(params, foreignTable)
	whereSQL0.WriteString("? IN (SELECT ? FROM ?) ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Find(&result) // ignore_security_alert
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
func (n negotiationRoundORMDo) ExistsAndGet(column string, value interface{}) (result workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM negotiation_rounds ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
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
func (n negotiationRoundORMDo) ExistsAndGetActive(column string, value interface{}) (result workspace_servicev1.NegotiationRoundORM, err error) {
	var params []interface{}

	var generateSQL strings.Builder
	generateSQL.WriteString("SELECT EXISTS( SELECT 1 FROM negotiation_rounds ")
	var whereSQL0 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL0.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL0)
	params = append(params, column)
	generateSQL.WriteString("), COALESCE(( SELECT ? FROM negotiation_rounds ")
	var whereSQL1 strings.Builder
	params = append(params, column)
	params = append(params, value)
	whereSQL1.WriteString("? = ? AND deleted_at IS NULL ")
	helper.JoinWhereBuilder(&generateSQL, whereSQL1)
	generateSQL.WriteString("LIMIT 1 ), NULL) ")

	var executeSQL *gorm.DB
	executeSQL = n.UnderlyingDB().Raw(generateSQL.String(), params...).Take(&result) // ignore_security_alert
	err = executeSQL.Error

	return
}

func (n negotiationRoundORMDo) Debug() INegotiationRoundORMDo {
	return n.withDO(n.DO.Debug())
}

func (n negotiationRoundORMDo) WithContext(ctx context.Context) INegotiationRoundORMDo {
	return n.withDO(n.DO.WithContext(ctx))
}

func (n negotiationRoundORMDo) ReadDB() INegotiationRoundORMDo {
	return n.Clauses(dbresolver.Read)
}

func (n negotiationRoundORMDo) WriteDB() INegotiationRoundORMDo {
	return n.Clauses(dbresolver.Write)
}

func (n negotiationRoundORMDo) Session(config *gorm.Session) INegotiationRoundORMDo {
	return n.withDO(n.DO.Session(config))
}

func (n negotiationRoundORMDo) Clauses(conds ...clause.Expression) INegotiationRoundORMDo {
	return n.withDO(n.DO.Clauses(conds...))
}

func (n negotiationRoundORMDo) Returning(value interface{}, columns ...string) INegotiationRoundORMDo {
	return n.withDO(n.DO.Returning(value, columns...))
}

func (n negotiationRoundORMDo) Not(conds ...gen.Condition) INegotiationRoundORMDo {
	return n.withDO(n.DO.Not(conds...))
}

func (n negotiationRoundORMDo) Or(conds ...gen.Condition) INegotiationRoundORMDo {
	return n.withDO(n.DO.Or(conds...))
}

func (n negotiationRoundORMDo) Select(conds ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Select(conds...))
}

func (n negotiationRoundORMDo) Where(conds ...gen.Condition) INegotiationRoundORMDo {
	return n.withDO(n.DO.Where(conds...))
}

func (n negotiationRoundORMDo) Order(conds ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Order(conds...))
}

func (n negotiationRoundORMDo) Distinct(cols ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Distinct(cols...))
}

func (n negotiationRoundORMDo) Omit(cols ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Omit(cols...))
}

func (n negotiationRoundORMDo) Join(table schema.Tabler, on ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Join(table, on...))
}

func (n negotiationRoundORMDo) LeftJoin(table schema.Tabler, on ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.LeftJoin(table, on...))
}

func (n negotiationRoundORMDo) RightJoin(table schema.Tabler, on ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.RightJoin(table, on...))
}

func (n negotiationRoundORMDo) Group(cols ...field.Expr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Group(cols...))
}

func (n negotiationRoundORMDo) Having(conds ...gen.Condition) INegotiationRoundORMDo {
	return n.withDO(n.DO.Having(conds...))
}

func (n negotiationRoundORMDo) Limit(limit int) INegotiationRoundORMDo {
	return n.withDO(n.DO.Limit(limit))
}

func (n negotiationRoundORMDo) Offset(offset int) INegotiationRoundORMDo {
	return n.withDO(n.DO.Offset(offset))
}

func (n negotiationRoundORMDo) Scopes(funcs ...func(gen.Dao) gen.Dao) INegotiationRoundORMDo {
	return n.withDO(n.DO.Scopes(funcs...))
}

func (n negotiationRoundORMDo) Unscoped() INegotiationRoundORMDo {
	return n.withDO(n.DO.Unscoped())
}

func (n negotiationRoundORMDo) Create(values ...*workspace_servicev1.NegotiationRoundORM) error {
	if len(values) == 0 {
		return nil
	}
	return n.DO.Create(values)
}

func (n negotiationRoundORMDo) CreateInBatches(values []*workspace_servicev1.NegotiationRoundORM, batchSize int) error {
	return n.DO.CreateInBatches(values, batchSize)
}

// Save : !!! underlying implementation is different with GORM
// The method is equivalent to executing the statement: db.Clauses(clause.OnConflict{UpdateAll: true}).Create(values)
func (n negotiationRoundORMDo) Save(values ...*workspace_servicev1.NegotiationRoundORM) error {
	if len(values) == 0 {
		return nil
	}
	return n.DO.Save(values)
}

func (n negotiationRoundORMDo) First() (*workspace_servicev1.NegotiationRoundORM, error) {
	if result, err := n.DO.First(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.NegotiationRoundORM), nil
	}
}

func (n negotiationRoundORMDo) Take() (*workspace_servicev1.NegotiationRoundORM, error) {
	if result, err := n.DO.Take(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.NegotiationRoundORM), nil
	}
}

func (n negotiationRoundORMDo) Last() (*workspace_servicev1.NegotiationRoundORM, error) {
	if result, err := n.DO.Last(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.NegotiationRoundORM), nil
	}
}

func (n negotiationRoundORMDo) Find() ([]*workspace_servicev1.NegotiationRoundORM, error) {
	result, err := n.DO.Find()
	return result.([]*workspace_servicev1.NegotiationRoundORM), err
}

func (n negotiationRoundORMDo) FindInBatch(batchSize int, fc func(tx gen.Dao, batch int) error) (results []*workspace_servicev1.NegotiationRoundORM, err error) {
	buf := make([]*workspace_servicev1.NegotiationRoundORM, 0, batchSize)
	err = n.DO.FindInBatches(&buf, batchSize, func(tx gen.Dao, batch int) error {
		defer func() { results = append(results, buf...) }()
		return fc(tx, batch)
	})
	return results, err
}

func (n negotiationRoundORMDo) FindInBatches(result *[]*workspace_servicev1.NegotiationRoundORM, batchSize int, fc func(tx gen.Dao, batch int) error) error {
	return n.DO.FindInBatches(result, batchSize, fc)
}

func (n negotiationRoundORMDo) Attrs(attrs ...field.AssignExpr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Attrs(attrs...))
}

func (n negotiationRoundORMDo) Assign(attrs ...field.AssignExpr) INegotiationRoundORMDo {
	return n.withDO(n.DO.Assign(attrs...))
}

func (n negotiationRoundORMDo) Joins(fields ...field.RelationField) INegotiationRoundORMDo {
	for _, _f := range fields {
		n = *n.withDO(n.DO.Joins(_f))
	}
	return &n
}

func (n negotiationRoundORMDo) Preload(fields ...field.RelationField) INegotiationRoundORMDo {
	for _, _f := range fields {
		n = *n.withDO(n.DO.Preload(_f))
	}
	return &n
}

func (n negotiationRoundORMDo) FirstOrInit() (*workspace_servicev1.NegotiationRoundORM, error) {
	if result, err := n.DO.FirstOrInit(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.NegotiationRoundORM), nil
	}
}

func (n negotiationRoundORMDo) FirstOrCreate() (*workspace_servicev1.NegotiationRoundORM, error) {
	if result, err := n.DO.FirstOrCreate(); err != nil {
		return nil, err
	} else {
		return result.(*workspace_servicev1.NegotiationRoundORM), nil
	}
}

func (n negotiationRoundORMDo) FindByPage(offset int, limit int) (result []*workspace_servicev1.NegotiationRoundORM, count int64, err error) {
	result, err = n.Offset(offset).Limit(limit).Find()
	if err != nil {
		return
	}

	if size := len(result); 0 < limit && 0 < size && size < limit {
		count = int64(size + offset)
		return
	}

	count, err = n.Offset(-1).Limit(-1).Count()
	return
}

func (n negotiationRoundORMDo) ScanByPage(result interface{}, offset int, limit int) (count int64, err error) {
	count, err = n.Count()
	if err != nil {
		return
	}

	err = n.Offset(offset).Limit(limit).Scan(result)
	return
}

func (n negotiationRoundORMDo) Scan(result interface{}) (err error) {
	return n.DO.Scan(result)
}

func (n negotiationRoundORMDo) Delete(models ...*workspace_servicev1.NegotiationRoundORM) (result gen.ResultInfo, err error) {
	return n.DO.Delete(models)
}

func (n *negotiationRoundORMDo) withDO(do gen.Dao) *negotiationRoundORMDo {
	n.DO = *do.(*gen.DO)
	return n
}
