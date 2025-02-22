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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.ComplianceCheckORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.ComplianceCheckORM{}) fail: %s", err)
	}
}

func Test_complianceCheckORMQuery(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	complianceCheckORM = *complianceCheckORM.As(complianceCheckORM.TableName())
	_do := complianceCheckORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(complianceCheckORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <compliance_checks> fail:", err)
		return
	}

	_, ok := complianceCheckORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from complianceCheckORM success")
	}

	err = _do.Create(&workspace_servicev1.ComplianceCheckORM{})
	if err != nil {
		t.Error("create item in table <compliance_checks> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.ComplianceCheckORM{})
	if err != nil {
		t.Error("create item in table <compliance_checks> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.ComplianceCheckORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <compliance_checks> fail:", err)
	}

	_, err = _do.Select(complianceCheckORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <compliance_checks> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <compliance_checks> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.ComplianceCheckORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Select(complianceCheckORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Select(complianceCheckORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <compliance_checks> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <compliance_checks> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.ComplianceCheckORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <compliance_checks> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <compliance_checks> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <compliance_checks> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <compliance_checks> fail:", err)
	}
}

var ComplianceCheckORMGetRecordByIDTestCase = []TestCase{}

func Test_complianceCheckORM_GetRecordByID(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMGetRecordByIDsTestCase = []TestCase{}

func Test_complianceCheckORM_GetRecordByIDs(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMDeleteRecordByIDTestCase = []TestCase{}

func Test_complianceCheckORM_DeleteRecordByID(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var ComplianceCheckORMGetAllRecordsTestCase = []TestCase{}

func Test_complianceCheckORM_GetAllRecords(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMCountAllTestCase = []TestCase{}

func Test_complianceCheckORM_CountAll(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMGetByIDTestCase = []TestCase{}

func Test_complianceCheckORM_GetByID(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMGetByIDsTestCase = []TestCase{}

func Test_complianceCheckORM_GetByIDs(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMGetActivePaginatedTestCase = []TestCase{}

func Test_complianceCheckORM_GetActivePaginated(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMGetDeletedTestCase = []TestCase{}

func Test_complianceCheckORM_GetDeleted(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMSoftDeleteTestCase = []TestCase{}

func Test_complianceCheckORM_SoftDelete(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var ComplianceCheckORMRestoreTestCase = []TestCase{}

func Test_complianceCheckORM_Restore(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var ComplianceCheckORMDeleteInBatchTestCase = []TestCase{}

func Test_complianceCheckORM_DeleteInBatch(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var ComplianceCheckORMGetByTimeRangeTestCase = []TestCase{}

func Test_complianceCheckORM_GetByTimeRange(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMFindByTestCase = []TestCase{}

func Test_complianceCheckORM_FindBy(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMFindByPatternTestCase = []TestCase{}

func Test_complianceCheckORM_FindByPattern(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMGetDistinctTestCase = []TestCase{}

func Test_complianceCheckORM_GetDistinct(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMFindWithFiltersTestCase = []TestCase{}

func Test_complianceCheckORM_FindWithFilters(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMTouchTestCase = []TestCase{}

func Test_complianceCheckORM_Touch(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var ComplianceCheckORMExistsTestCase = []TestCase{}

func Test_complianceCheckORM_Exists(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMExistsByIdTestCase = []TestCase{}

func Test_complianceCheckORM_ExistsById(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMFindBySubqueryTestCase = []TestCase{}

func Test_complianceCheckORM_FindBySubquery(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMExistsAndGetTestCase = []TestCase{}

func Test_complianceCheckORM_ExistsAndGet(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var ComplianceCheckORMExistsAndGetActiveTestCase = []TestCase{}

func Test_complianceCheckORM_ExistsAndGetActive(t *testing.T) {
	complianceCheckORM := newComplianceCheckORM(_gen_test_db)
	do := complianceCheckORM.WithContext(context.Background()).Debug()

	for i, tt := range ComplianceCheckORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
