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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.AppPermissionORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.AppPermissionORM{}) fail: %s", err)
	}
}

func Test_appPermissionORMQuery(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	appPermissionORM = *appPermissionORM.As(appPermissionORM.TableName())
	_do := appPermissionORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(appPermissionORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <app_permissions> fail:", err)
		return
	}

	_, ok := appPermissionORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from appPermissionORM success")
	}

	err = _do.Create(&workspace_servicev1.AppPermissionORM{})
	if err != nil {
		t.Error("create item in table <app_permissions> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.AppPermissionORM{})
	if err != nil {
		t.Error("create item in table <app_permissions> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.AppPermissionORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <app_permissions> fail:", err)
	}

	_, err = _do.Select(appPermissionORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <app_permissions> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <app_permissions> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <app_permissions> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <app_permissions> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.AppPermissionORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <app_permissions> fail:", err)
	}

	_, err = _do.Select(appPermissionORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <app_permissions> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <app_permissions> fail:", err)
	}

	_, err = _do.Select(appPermissionORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <app_permissions> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <app_permissions> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <app_permissions> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <app_permissions> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.AppPermissionORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <app_permissions> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <app_permissions> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <app_permissions> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <app_permissions> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <app_permissions> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <app_permissions> fail:", err)
	}
}

var AppPermissionORMGetRecordByIDTestCase = []TestCase{}

func Test_appPermissionORM_GetRecordByID(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMGetRecordByIDsTestCase = []TestCase{}

func Test_appPermissionORM_GetRecordByIDs(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMDeleteRecordByIDTestCase = []TestCase{}

func Test_appPermissionORM_DeleteRecordByID(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppPermissionORMGetAllRecordsTestCase = []TestCase{}

func Test_appPermissionORM_GetAllRecords(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMCountAllTestCase = []TestCase{}

func Test_appPermissionORM_CountAll(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMGetByIDTestCase = []TestCase{}

func Test_appPermissionORM_GetByID(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMGetByIDsTestCase = []TestCase{}

func Test_appPermissionORM_GetByIDs(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMGetActivePaginatedTestCase = []TestCase{}

func Test_appPermissionORM_GetActivePaginated(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMGetDeletedTestCase = []TestCase{}

func Test_appPermissionORM_GetDeleted(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMSoftDeleteTestCase = []TestCase{}

func Test_appPermissionORM_SoftDelete(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppPermissionORMRestoreTestCase = []TestCase{}

func Test_appPermissionORM_Restore(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppPermissionORMDeleteInBatchTestCase = []TestCase{}

func Test_appPermissionORM_DeleteInBatch(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppPermissionORMGetByTimeRangeTestCase = []TestCase{}

func Test_appPermissionORM_GetByTimeRange(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMFindByTestCase = []TestCase{}

func Test_appPermissionORM_FindBy(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMFindByPatternTestCase = []TestCase{}

func Test_appPermissionORM_FindByPattern(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMGetDistinctTestCase = []TestCase{}

func Test_appPermissionORM_GetDistinct(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMFindWithFiltersTestCase = []TestCase{}

func Test_appPermissionORM_FindWithFilters(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMTouchTestCase = []TestCase{}

func Test_appPermissionORM_Touch(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppPermissionORMExistsTestCase = []TestCase{}

func Test_appPermissionORM_Exists(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMExistsByIdTestCase = []TestCase{}

func Test_appPermissionORM_ExistsById(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMFindBySubqueryTestCase = []TestCase{}

func Test_appPermissionORM_FindBySubquery(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMExistsAndGetTestCase = []TestCase{}

func Test_appPermissionORM_ExistsAndGet(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppPermissionORMExistsAndGetActiveTestCase = []TestCase{}

func Test_appPermissionORM_ExistsAndGetActive(t *testing.T) {
	appPermissionORM := newAppPermissionORM(_gen_test_db)
	do := appPermissionORM.WithContext(context.Background()).Debug()

	for i, tt := range AppPermissionORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
