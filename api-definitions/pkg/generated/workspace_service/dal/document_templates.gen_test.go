// Code generated by gorm.io/gen. DO NOT EDIT.
// Code generated by gorm.io/gen. DO NOT EDIT.
// Code generated by gorm.io/gen. DO NOT EDIT.

package dal

import (
	"context"
	"fmt"
	"strconv"
	"testing"
	"time"

	workspace_servicev1 "github.com/VectorEngineering/vector-protobuf-definitions/api-definitions/pkg/generated/workspace_service/v1"
	"gorm.io/gen"
	"gorm.io/gen/field"
	"gorm.io/gorm/clause"
)

func init() {
	InitializeDB()
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.DocumentTemplateORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.DocumentTemplateORM{}) fail: %s", err)
	}
}

func Test_documentTemplateORMQuery(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	documentTemplateORM = *documentTemplateORM.As(documentTemplateORM.TableName())
	_do := documentTemplateORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(documentTemplateORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <document_templates> fail:", err)
		return
	}

	_, ok := documentTemplateORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from documentTemplateORM success")
	}

	err = _do.Create(&workspace_servicev1.DocumentTemplateORM{})
	if err != nil {
		t.Error("create item in table <document_templates> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.DocumentTemplateORM{})
	if err != nil {
		t.Error("create item in table <document_templates> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.DocumentTemplateORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <document_templates> fail:", err)
	}

	_, err = _do.Select(documentTemplateORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <document_templates> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <document_templates> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <document_templates> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <document_templates> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.DocumentTemplateORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <document_templates> fail:", err)
	}

	_, err = _do.Select(documentTemplateORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <document_templates> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <document_templates> fail:", err)
	}

	_, err = _do.Select(documentTemplateORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <document_templates> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <document_templates> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <document_templates> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <document_templates> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.DocumentTemplateORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <document_templates> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <document_templates> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <document_templates> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <document_templates> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <document_templates> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <document_templates> fail:", err)
	}
}

var DocumentTemplateORMGetRecordByIDTestCase = []TestCase{}

func Test_documentTemplateORM_GetRecordByID(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMGetRecordByIDsTestCase = []TestCase{}

func Test_documentTemplateORM_GetRecordByIDs(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMDeleteRecordByIDTestCase = []TestCase{}

func Test_documentTemplateORM_DeleteRecordByID(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentTemplateORMGetAllRecordsTestCase = []TestCase{}

func Test_documentTemplateORM_GetAllRecords(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMCountAllTestCase = []TestCase{}

func Test_documentTemplateORM_CountAll(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMGetByIDTestCase = []TestCase{}

func Test_documentTemplateORM_GetByID(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMGetByIDsTestCase = []TestCase{}

func Test_documentTemplateORM_GetByIDs(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMGetActivePaginatedTestCase = []TestCase{}

func Test_documentTemplateORM_GetActivePaginated(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMGetDeletedTestCase = []TestCase{}

func Test_documentTemplateORM_GetDeleted(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMSoftDeleteTestCase = []TestCase{}

func Test_documentTemplateORM_SoftDelete(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentTemplateORMRestoreTestCase = []TestCase{}

func Test_documentTemplateORM_Restore(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentTemplateORMDeleteInBatchTestCase = []TestCase{}

func Test_documentTemplateORM_DeleteInBatch(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentTemplateORMGetByTimeRangeTestCase = []TestCase{}

func Test_documentTemplateORM_GetByTimeRange(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMFindByTestCase = []TestCase{}

func Test_documentTemplateORM_FindBy(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMFindByPatternTestCase = []TestCase{}

func Test_documentTemplateORM_FindByPattern(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMGetDistinctTestCase = []TestCase{}

func Test_documentTemplateORM_GetDistinct(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMFindWithFiltersTestCase = []TestCase{}

func Test_documentTemplateORM_FindWithFilters(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMTouchTestCase = []TestCase{}

func Test_documentTemplateORM_Touch(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentTemplateORMExistsTestCase = []TestCase{}

func Test_documentTemplateORM_Exists(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMExistsByIdTestCase = []TestCase{}

func Test_documentTemplateORM_ExistsById(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMFindBySubqueryTestCase = []TestCase{}

func Test_documentTemplateORM_FindBySubquery(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMExistsAndGetTestCase = []TestCase{}

func Test_documentTemplateORM_ExistsAndGet(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentTemplateORMExistsAndGetActiveTestCase = []TestCase{}

func Test_documentTemplateORM_ExistsAndGetActive(t *testing.T) {
	documentTemplateORM := newDocumentTemplateORM(_gen_test_db)
	do := documentTemplateORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentTemplateORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
