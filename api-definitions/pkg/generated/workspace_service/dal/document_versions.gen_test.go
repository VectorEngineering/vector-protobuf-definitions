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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.DocumentVersionORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.DocumentVersionORM{}) fail: %s", err)
	}
}

func Test_documentVersionORMQuery(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	documentVersionORM = *documentVersionORM.As(documentVersionORM.TableName())
	_do := documentVersionORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(documentVersionORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <document_versions> fail:", err)
		return
	}

	_, ok := documentVersionORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from documentVersionORM success")
	}

	err = _do.Create(&workspace_servicev1.DocumentVersionORM{})
	if err != nil {
		t.Error("create item in table <document_versions> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.DocumentVersionORM{})
	if err != nil {
		t.Error("create item in table <document_versions> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.DocumentVersionORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <document_versions> fail:", err)
	}

	_, err = _do.Select(documentVersionORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <document_versions> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <document_versions> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <document_versions> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <document_versions> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.DocumentVersionORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <document_versions> fail:", err)
	}

	_, err = _do.Select(documentVersionORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <document_versions> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <document_versions> fail:", err)
	}

	_, err = _do.Select(documentVersionORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <document_versions> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <document_versions> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <document_versions> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <document_versions> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.DocumentVersionORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <document_versions> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <document_versions> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <document_versions> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <document_versions> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <document_versions> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <document_versions> fail:", err)
	}
}

var DocumentVersionORMGetRecordByIDTestCase = []TestCase{}

func Test_documentVersionORM_GetRecordByID(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMGetRecordByIDsTestCase = []TestCase{}

func Test_documentVersionORM_GetRecordByIDs(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMDeleteRecordByIDTestCase = []TestCase{}

func Test_documentVersionORM_DeleteRecordByID(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentVersionORMGetAllRecordsTestCase = []TestCase{}

func Test_documentVersionORM_GetAllRecords(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMCountAllTestCase = []TestCase{}

func Test_documentVersionORM_CountAll(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMGetByIDTestCase = []TestCase{}

func Test_documentVersionORM_GetByID(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMGetByIDsTestCase = []TestCase{}

func Test_documentVersionORM_GetByIDs(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMGetActivePaginatedTestCase = []TestCase{}

func Test_documentVersionORM_GetActivePaginated(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMGetDeletedTestCase = []TestCase{}

func Test_documentVersionORM_GetDeleted(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMSoftDeleteTestCase = []TestCase{}

func Test_documentVersionORM_SoftDelete(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentVersionORMRestoreTestCase = []TestCase{}

func Test_documentVersionORM_Restore(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentVersionORMDeleteInBatchTestCase = []TestCase{}

func Test_documentVersionORM_DeleteInBatch(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentVersionORMGetByTimeRangeTestCase = []TestCase{}

func Test_documentVersionORM_GetByTimeRange(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMFindByTestCase = []TestCase{}

func Test_documentVersionORM_FindBy(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMFindByPatternTestCase = []TestCase{}

func Test_documentVersionORM_FindByPattern(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMGetDistinctTestCase = []TestCase{}

func Test_documentVersionORM_GetDistinct(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMFindWithFiltersTestCase = []TestCase{}

func Test_documentVersionORM_FindWithFilters(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMTouchTestCase = []TestCase{}

func Test_documentVersionORM_Touch(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var DocumentVersionORMExistsTestCase = []TestCase{}

func Test_documentVersionORM_Exists(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMExistsByIdTestCase = []TestCase{}

func Test_documentVersionORM_ExistsById(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMFindBySubqueryTestCase = []TestCase{}

func Test_documentVersionORM_FindBySubquery(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMExistsAndGetTestCase = []TestCase{}

func Test_documentVersionORM_ExistsAndGet(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var DocumentVersionORMExistsAndGetActiveTestCase = []TestCase{}

func Test_documentVersionORM_ExistsAndGetActive(t *testing.T) {
	documentVersionORM := newDocumentVersionORM(_gen_test_db)
	do := documentVersionORM.WithContext(context.Background()).Debug()

	for i, tt := range DocumentVersionORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
