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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.WorkspaceComplianceORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.WorkspaceComplianceORM{}) fail: %s", err)
	}
}

func Test_workspaceComplianceORMQuery(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	workspaceComplianceORM = *workspaceComplianceORM.As(workspaceComplianceORM.TableName())
	_do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(workspaceComplianceORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <workspace_compliance> fail:", err)
		return
	}

	_, ok := workspaceComplianceORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from workspaceComplianceORM success")
	}

	err = _do.Create(&workspace_servicev1.WorkspaceComplianceORM{})
	if err != nil {
		t.Error("create item in table <workspace_compliance> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.WorkspaceComplianceORM{})
	if err != nil {
		t.Error("create item in table <workspace_compliance> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.WorkspaceComplianceORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <workspace_compliance> fail:", err)
	}

	_, err = _do.Select(workspaceComplianceORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <workspace_compliance> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.WorkspaceComplianceORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Select(workspaceComplianceORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Select(workspaceComplianceORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <workspace_compliance> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.WorkspaceComplianceORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <workspace_compliance> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <workspace_compliance> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <workspace_compliance> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <workspace_compliance> fail:", err)
	}
}

var WorkspaceComplianceORMGetRecordByIDTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetRecordByID(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMGetRecordByIDsTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetRecordByIDs(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMDeleteRecordByIDTestCase = []TestCase{}

func Test_workspaceComplianceORM_DeleteRecordByID(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var WorkspaceComplianceORMGetAllRecordsTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetAllRecords(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMCountAllTestCase = []TestCase{}

func Test_workspaceComplianceORM_CountAll(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMGetByIDTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetByID(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMGetByIDsTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetByIDs(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMGetActivePaginatedTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetActivePaginated(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMGetDeletedTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetDeleted(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMSoftDeleteTestCase = []TestCase{}

func Test_workspaceComplianceORM_SoftDelete(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var WorkspaceComplianceORMRestoreTestCase = []TestCase{}

func Test_workspaceComplianceORM_Restore(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var WorkspaceComplianceORMDeleteInBatchTestCase = []TestCase{}

func Test_workspaceComplianceORM_DeleteInBatch(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var WorkspaceComplianceORMGetByTimeRangeTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetByTimeRange(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMFindByTestCase = []TestCase{}

func Test_workspaceComplianceORM_FindBy(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMFindByPatternTestCase = []TestCase{}

func Test_workspaceComplianceORM_FindByPattern(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMGetDistinctTestCase = []TestCase{}

func Test_workspaceComplianceORM_GetDistinct(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMFindWithFiltersTestCase = []TestCase{}

func Test_workspaceComplianceORM_FindWithFilters(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMTouchTestCase = []TestCase{}

func Test_workspaceComplianceORM_Touch(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var WorkspaceComplianceORMExistsTestCase = []TestCase{}

func Test_workspaceComplianceORM_Exists(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMExistsByIdTestCase = []TestCase{}

func Test_workspaceComplianceORM_ExistsById(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMFindBySubqueryTestCase = []TestCase{}

func Test_workspaceComplianceORM_FindBySubquery(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMExistsAndGetTestCase = []TestCase{}

func Test_workspaceComplianceORM_ExistsAndGet(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var WorkspaceComplianceORMExistsAndGetActiveTestCase = []TestCase{}

func Test_workspaceComplianceORM_ExistsAndGetActive(t *testing.T) {
	workspaceComplianceORM := newWorkspaceComplianceORM(_gen_test_db)
	do := workspaceComplianceORM.WithContext(context.Background()).Debug()

	for i, tt := range WorkspaceComplianceORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
