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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.RiskAssessmentORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.RiskAssessmentORM{}) fail: %s", err)
	}
}

func Test_riskAssessmentORMQuery(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	riskAssessmentORM = *riskAssessmentORM.As(riskAssessmentORM.TableName())
	_do := riskAssessmentORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(riskAssessmentORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <risk_assessments> fail:", err)
		return
	}

	_, ok := riskAssessmentORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from riskAssessmentORM success")
	}

	err = _do.Create(&workspace_servicev1.RiskAssessmentORM{})
	if err != nil {
		t.Error("create item in table <risk_assessments> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.RiskAssessmentORM{})
	if err != nil {
		t.Error("create item in table <risk_assessments> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.RiskAssessmentORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <risk_assessments> fail:", err)
	}

	_, err = _do.Select(riskAssessmentORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <risk_assessments> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <risk_assessments> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.RiskAssessmentORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Select(riskAssessmentORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Select(riskAssessmentORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <risk_assessments> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <risk_assessments> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.RiskAssessmentORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <risk_assessments> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <risk_assessments> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <risk_assessments> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <risk_assessments> fail:", err)
	}
}

var RiskAssessmentORMGetRecordByIDTestCase = []TestCase{}

func Test_riskAssessmentORM_GetRecordByID(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMGetRecordByIDsTestCase = []TestCase{}

func Test_riskAssessmentORM_GetRecordByIDs(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMDeleteRecordByIDTestCase = []TestCase{}

func Test_riskAssessmentORM_DeleteRecordByID(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var RiskAssessmentORMGetAllRecordsTestCase = []TestCase{}

func Test_riskAssessmentORM_GetAllRecords(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMCountAllTestCase = []TestCase{}

func Test_riskAssessmentORM_CountAll(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMGetByIDTestCase = []TestCase{}

func Test_riskAssessmentORM_GetByID(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMGetByIDsTestCase = []TestCase{}

func Test_riskAssessmentORM_GetByIDs(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMGetActivePaginatedTestCase = []TestCase{}

func Test_riskAssessmentORM_GetActivePaginated(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMGetDeletedTestCase = []TestCase{}

func Test_riskAssessmentORM_GetDeleted(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMSoftDeleteTestCase = []TestCase{}

func Test_riskAssessmentORM_SoftDelete(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var RiskAssessmentORMRestoreTestCase = []TestCase{}

func Test_riskAssessmentORM_Restore(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var RiskAssessmentORMDeleteInBatchTestCase = []TestCase{}

func Test_riskAssessmentORM_DeleteInBatch(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var RiskAssessmentORMGetByTimeRangeTestCase = []TestCase{}

func Test_riskAssessmentORM_GetByTimeRange(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMFindByTestCase = []TestCase{}

func Test_riskAssessmentORM_FindBy(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMFindByPatternTestCase = []TestCase{}

func Test_riskAssessmentORM_FindByPattern(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMGetDistinctTestCase = []TestCase{}

func Test_riskAssessmentORM_GetDistinct(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMFindWithFiltersTestCase = []TestCase{}

func Test_riskAssessmentORM_FindWithFilters(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMTouchTestCase = []TestCase{}

func Test_riskAssessmentORM_Touch(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var RiskAssessmentORMExistsTestCase = []TestCase{}

func Test_riskAssessmentORM_Exists(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMExistsByIdTestCase = []TestCase{}

func Test_riskAssessmentORM_ExistsById(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMFindBySubqueryTestCase = []TestCase{}

func Test_riskAssessmentORM_FindBySubquery(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMExistsAndGetTestCase = []TestCase{}

func Test_riskAssessmentORM_ExistsAndGet(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var RiskAssessmentORMExistsAndGetActiveTestCase = []TestCase{}

func Test_riskAssessmentORM_ExistsAndGetActive(t *testing.T) {
	riskAssessmentORM := newRiskAssessmentORM(_gen_test_db)
	do := riskAssessmentORM.WithContext(context.Background()).Debug()

	for i, tt := range RiskAssessmentORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
