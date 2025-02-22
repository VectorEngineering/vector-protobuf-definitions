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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.SignatureWorkflowORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.SignatureWorkflowORM{}) fail: %s", err)
	}
}

func Test_signatureWorkflowORMQuery(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	signatureWorkflowORM = *signatureWorkflowORM.As(signatureWorkflowORM.TableName())
	_do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(signatureWorkflowORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <signature_workflows> fail:", err)
		return
	}

	_, ok := signatureWorkflowORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from signatureWorkflowORM success")
	}

	err = _do.Create(&workspace_servicev1.SignatureWorkflowORM{})
	if err != nil {
		t.Error("create item in table <signature_workflows> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.SignatureWorkflowORM{})
	if err != nil {
		t.Error("create item in table <signature_workflows> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.SignatureWorkflowORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <signature_workflows> fail:", err)
	}

	_, err = _do.Select(signatureWorkflowORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <signature_workflows> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <signature_workflows> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.SignatureWorkflowORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Select(signatureWorkflowORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Select(signatureWorkflowORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <signature_workflows> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <signature_workflows> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.SignatureWorkflowORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <signature_workflows> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <signature_workflows> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <signature_workflows> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <signature_workflows> fail:", err)
	}
}

var SignatureWorkflowORMGetRecordByIDTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetRecordByID(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMGetRecordByIDsTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetRecordByIDs(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMDeleteRecordByIDTestCase = []TestCase{}

func Test_signatureWorkflowORM_DeleteRecordByID(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var SignatureWorkflowORMGetAllRecordsTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetAllRecords(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMCountAllTestCase = []TestCase{}

func Test_signatureWorkflowORM_CountAll(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMGetByIDTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetByID(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMGetByIDsTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetByIDs(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMGetActivePaginatedTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetActivePaginated(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMGetDeletedTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetDeleted(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMSoftDeleteTestCase = []TestCase{}

func Test_signatureWorkflowORM_SoftDelete(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var SignatureWorkflowORMRestoreTestCase = []TestCase{}

func Test_signatureWorkflowORM_Restore(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var SignatureWorkflowORMDeleteInBatchTestCase = []TestCase{}

func Test_signatureWorkflowORM_DeleteInBatch(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var SignatureWorkflowORMGetByTimeRangeTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetByTimeRange(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMFindByTestCase = []TestCase{}

func Test_signatureWorkflowORM_FindBy(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMFindByPatternTestCase = []TestCase{}

func Test_signatureWorkflowORM_FindByPattern(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMGetDistinctTestCase = []TestCase{}

func Test_signatureWorkflowORM_GetDistinct(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMFindWithFiltersTestCase = []TestCase{}

func Test_signatureWorkflowORM_FindWithFilters(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMTouchTestCase = []TestCase{}

func Test_signatureWorkflowORM_Touch(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var SignatureWorkflowORMExistsTestCase = []TestCase{}

func Test_signatureWorkflowORM_Exists(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMExistsByIdTestCase = []TestCase{}

func Test_signatureWorkflowORM_ExistsById(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMFindBySubqueryTestCase = []TestCase{}

func Test_signatureWorkflowORM_FindBySubquery(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMExistsAndGetTestCase = []TestCase{}

func Test_signatureWorkflowORM_ExistsAndGet(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var SignatureWorkflowORMExistsAndGetActiveTestCase = []TestCase{}

func Test_signatureWorkflowORM_ExistsAndGetActive(t *testing.T) {
	signatureWorkflowORM := newSignatureWorkflowORM(_gen_test_db)
	do := signatureWorkflowORM.WithContext(context.Background()).Debug()

	for i, tt := range SignatureWorkflowORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
