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
	err := _gen_test_db.AutoMigrate(&workspace_servicev1.AppWebhookORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&workspace_servicev1.AppWebhookORM{}) fail: %s", err)
	}
}

func Test_appWebhookORMQuery(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	appWebhookORM = *appWebhookORM.As(appWebhookORM.TableName())
	_do := appWebhookORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(appWebhookORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <app_webhooks> fail:", err)
		return
	}

	_, ok := appWebhookORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from appWebhookORM success")
	}

	err = _do.Create(&workspace_servicev1.AppWebhookORM{})
	if err != nil {
		t.Error("create item in table <app_webhooks> fail:", err)
	}

	err = _do.Save(&workspace_servicev1.AppWebhookORM{})
	if err != nil {
		t.Error("create item in table <app_webhooks> fail:", err)
	}

	err = _do.CreateInBatches([]*workspace_servicev1.AppWebhookORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <app_webhooks> fail:", err)
	}

	_, err = _do.Select(appWebhookORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <app_webhooks> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <app_webhooks> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*workspace_servicev1.AppWebhookORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Select(appWebhookORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Select(appWebhookORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <app_webhooks> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <app_webhooks> fail:", err)
	}

	_, err = _do.ScanByPage(&workspace_servicev1.AppWebhookORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <app_webhooks> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <app_webhooks> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <app_webhooks> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <app_webhooks> fail:", err)
	}
}

var AppWebhookORMGetRecordByIDTestCase = []TestCase{}

func Test_appWebhookORM_GetRecordByID(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMGetRecordByIDsTestCase = []TestCase{}

func Test_appWebhookORM_GetRecordByIDs(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMDeleteRecordByIDTestCase = []TestCase{}

func Test_appWebhookORM_DeleteRecordByID(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppWebhookORMGetAllRecordsTestCase = []TestCase{}

func Test_appWebhookORM_GetAllRecords(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMCountAllTestCase = []TestCase{}

func Test_appWebhookORM_CountAll(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMGetByIDTestCase = []TestCase{}

func Test_appWebhookORM_GetByID(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMGetByIDsTestCase = []TestCase{}

func Test_appWebhookORM_GetByIDs(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMGetActivePaginatedTestCase = []TestCase{}

func Test_appWebhookORM_GetActivePaginated(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMGetDeletedTestCase = []TestCase{}

func Test_appWebhookORM_GetDeleted(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMSoftDeleteTestCase = []TestCase{}

func Test_appWebhookORM_SoftDelete(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppWebhookORMRestoreTestCase = []TestCase{}

func Test_appWebhookORM_Restore(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppWebhookORMDeleteInBatchTestCase = []TestCase{}

func Test_appWebhookORM_DeleteInBatch(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppWebhookORMGetByTimeRangeTestCase = []TestCase{}

func Test_appWebhookORM_GetByTimeRange(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMFindByTestCase = []TestCase{}

func Test_appWebhookORM_FindBy(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMFindByPatternTestCase = []TestCase{}

func Test_appWebhookORM_FindByPattern(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMGetDistinctTestCase = []TestCase{}

func Test_appWebhookORM_GetDistinct(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMFindWithFiltersTestCase = []TestCase{}

func Test_appWebhookORM_FindWithFilters(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMTouchTestCase = []TestCase{}

func Test_appWebhookORM_Touch(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var AppWebhookORMExistsTestCase = []TestCase{}

func Test_appWebhookORM_Exists(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMExistsByIdTestCase = []TestCase{}

func Test_appWebhookORM_ExistsById(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMFindBySubqueryTestCase = []TestCase{}

func Test_appWebhookORM_FindBySubquery(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMExistsAndGetTestCase = []TestCase{}

func Test_appWebhookORM_ExistsAndGet(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var AppWebhookORMExistsAndGetActiveTestCase = []TestCase{}

func Test_appWebhookORM_ExistsAndGetActive(t *testing.T) {
	appWebhookORM := newAppWebhookORM(_gen_test_db)
	do := appWebhookORM.WithContext(context.Background()).Debug()

	for i, tt := range AppWebhookORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
