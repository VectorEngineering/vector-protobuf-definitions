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

	lead_scraper_servicev1 "github.com/VectorEngineering/vector-protobuf-definitions/api-definitions/pkg/generated/lead_scraper_service/v1"
	"gorm.io/gen"
	"gorm.io/gen/field"
	"gorm.io/gorm/clause"
)

func init() {
	InitializeDB()
	err := _gen_test_db.AutoMigrate(&lead_scraper_servicev1.TenantORM{})
	if err != nil {
		fmt.Printf("Error: AutoMigrate(&lead_scraper_servicev1.TenantORM{}) fail: %s", err)
	}
}

func Test_tenantORMQuery(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	tenantORM = *tenantORM.As(tenantORM.TableName())
	_do := tenantORM.WithContext(context.Background()).Debug()

	primaryKey := field.NewString(tenantORM.TableName(), clause.PrimaryKey)
	_, err := _do.Unscoped().Where(primaryKey.IsNotNull()).Delete()
	if err != nil {
		t.Error("clean table <tenants> fail:", err)
		return
	}

	_, ok := tenantORM.GetFieldByName("")
	if ok {
		t.Error("GetFieldByName(\"\") from tenantORM success")
	}

	err = _do.Create(&lead_scraper_servicev1.TenantORM{})
	if err != nil {
		t.Error("create item in table <tenants> fail:", err)
	}

	err = _do.Save(&lead_scraper_servicev1.TenantORM{})
	if err != nil {
		t.Error("create item in table <tenants> fail:", err)
	}

	err = _do.CreateInBatches([]*lead_scraper_servicev1.TenantORM{{}, {}}, 10)
	if err != nil {
		t.Error("create item in table <tenants> fail:", err)
	}

	_, err = _do.Select(tenantORM.ALL).Take()
	if err != nil {
		t.Error("Take() on table <tenants> fail:", err)
	}

	_, err = _do.First()
	if err != nil {
		t.Error("First() on table <tenants> fail:", err)
	}

	_, err = _do.Last()
	if err != nil {
		t.Error("First() on table <tenants> fail:", err)
	}

	_, err = _do.Where(primaryKey.IsNotNull()).FindInBatch(10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatch() on table <tenants> fail:", err)
	}

	err = _do.Where(primaryKey.IsNotNull()).FindInBatches(&[]*lead_scraper_servicev1.TenantORM{}, 10, func(tx gen.Dao, batch int) error { return nil })
	if err != nil {
		t.Error("FindInBatches() on table <tenants> fail:", err)
	}

	_, err = _do.Select(tenantORM.ALL).Where(primaryKey.IsNotNull()).Order(primaryKey.Desc()).Find()
	if err != nil {
		t.Error("Find() on table <tenants> fail:", err)
	}

	_, err = _do.Distinct(primaryKey).Take()
	if err != nil {
		t.Error("select Distinct() on table <tenants> fail:", err)
	}

	_, err = _do.Select(tenantORM.ALL).Omit(primaryKey).Take()
	if err != nil {
		t.Error("Omit() on table <tenants> fail:", err)
	}

	_, err = _do.Group(primaryKey).Find()
	if err != nil {
		t.Error("Group() on table <tenants> fail:", err)
	}

	_, err = _do.Scopes(func(dao gen.Dao) gen.Dao { return dao.Where(primaryKey.IsNotNull()) }).Find()
	if err != nil {
		t.Error("Scopes() on table <tenants> fail:", err)
	}

	_, _, err = _do.FindByPage(0, 1)
	if err != nil {
		t.Error("FindByPage() on table <tenants> fail:", err)
	}

	_, err = _do.ScanByPage(&lead_scraper_servicev1.TenantORM{}, 0, 1)
	if err != nil {
		t.Error("ScanByPage() on table <tenants> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrInit()
	if err != nil {
		t.Error("FirstOrInit() on table <tenants> fail:", err)
	}

	_, err = _do.Attrs(primaryKey).Assign(primaryKey).FirstOrCreate()
	if err != nil {
		t.Error("FirstOrCreate() on table <tenants> fail:", err)
	}

	var _a _another
	var _aPK = field.NewString(_a.TableName(), "id")

	err = _do.Join(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("Join() on table <tenants> fail:", err)
	}

	err = _do.LeftJoin(&_a, primaryKey.EqCol(_aPK)).Scan(map[string]interface{}{})
	if err != nil {
		t.Error("LeftJoin() on table <tenants> fail:", err)
	}

	_, err = _do.Not().Or().Clauses().Take()
	if err != nil {
		t.Error("Not/Or/Clauses on table <tenants> fail:", err)
	}
}

var TenantORMGetRecordByIDTestCase = []TestCase{}

func Test_tenantORM_GetRecordByID(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetRecordByIDTestCase {
		t.Run("GetRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "GetRecordByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMGetRecordByIDsTestCase = []TestCase{}

func Test_tenantORM_GetRecordByIDs(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetRecordByIDsTestCase {
		t.Run("GetRecordByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetRecordByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetRecordByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetRecordByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMDeleteRecordByIDTestCase = []TestCase{}

func Test_tenantORM_DeleteRecordByID(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMDeleteRecordByIDTestCase {
		t.Run("DeleteRecordByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteRecordByID(tt.Input.Args[0].(uint64))
			assert(t, "DeleteRecordByID", res1, tt.Expectation.Ret[0])
		})
	}
}

var TenantORMGetAllRecordsTestCase = []TestCase{}

func Test_tenantORM_GetAllRecords(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetAllRecordsTestCase {
		t.Run("GetAllRecords_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetAllRecords(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetAllRecords", res1, tt.Expectation.Ret[0])
			assert(t, "GetAllRecords", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMCountAllTestCase = []TestCase{}

func Test_tenantORM_CountAll(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMCountAllTestCase {
		t.Run("CountAll_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.CountAll()
			assert(t, "CountAll", res1, tt.Expectation.Ret[0])
			assert(t, "CountAll", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMGetByIDTestCase = []TestCase{}

func Test_tenantORM_GetByID(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetByIDTestCase {
		t.Run("GetByID_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByID(tt.Input.Args[0].(uint64))
			assert(t, "GetByID", res1, tt.Expectation.Ret[0])
			assert(t, "GetByID", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMGetByIDsTestCase = []TestCase{}

func Test_tenantORM_GetByIDs(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetByIDsTestCase {
		t.Run("GetByIDs_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByIDs(tt.Input.Args[0].([]uint64))
			assert(t, "GetByIDs", res1, tt.Expectation.Ret[0])
			assert(t, "GetByIDs", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMGetActivePaginatedTestCase = []TestCase{}

func Test_tenantORM_GetActivePaginated(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetActivePaginatedTestCase {
		t.Run("GetActivePaginated_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetActivePaginated(tt.Input.Args[0].(string), tt.Input.Args[1].(int), tt.Input.Args[2].(int))
			assert(t, "GetActivePaginated", res1, tt.Expectation.Ret[0])
			assert(t, "GetActivePaginated", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMGetDeletedTestCase = []TestCase{}

func Test_tenantORM_GetDeleted(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetDeletedTestCase {
		t.Run("GetDeleted_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDeleted()
			assert(t, "GetDeleted", res1, tt.Expectation.Ret[0])
			assert(t, "GetDeleted", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMSoftDeleteTestCase = []TestCase{}

func Test_tenantORM_SoftDelete(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMSoftDeleteTestCase {
		t.Run("SoftDelete_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.SoftDelete(tt.Input.Args[0].(uint64))
			assert(t, "SoftDelete", res1, tt.Expectation.Ret[0])
		})
	}
}

var TenantORMRestoreTestCase = []TestCase{}

func Test_tenantORM_Restore(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMRestoreTestCase {
		t.Run("Restore_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Restore(tt.Input.Args[0].(uint64))
			assert(t, "Restore", res1, tt.Expectation.Ret[0])
		})
	}
}

var TenantORMDeleteInBatchTestCase = []TestCase{}

func Test_tenantORM_DeleteInBatch(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMDeleteInBatchTestCase {
		t.Run("DeleteInBatch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.DeleteInBatch(tt.Input.Args[0].([]uint64), tt.Input.Args[1].(int))
			assert(t, "DeleteInBatch", res1, tt.Expectation.Ret[0])
		})
	}
}

var TenantORMGetByTimeRangeTestCase = []TestCase{}

func Test_tenantORM_GetByTimeRange(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetByTimeRangeTestCase {
		t.Run("GetByTimeRange_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetByTimeRange(tt.Input.Args[0].(time.Time), tt.Input.Args[1].(time.Time))
			assert(t, "GetByTimeRange", res1, tt.Expectation.Ret[0])
			assert(t, "GetByTimeRange", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMFindByTestCase = []TestCase{}

func Test_tenantORM_FindBy(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMFindByTestCase {
		t.Run("FindBy_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBy(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(interface{}))
			assert(t, "FindBy", res1, tt.Expectation.Ret[0])
			assert(t, "FindBy", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMFindByPatternTestCase = []TestCase{}

func Test_tenantORM_FindByPattern(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMFindByPatternTestCase {
		t.Run("FindByPattern_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindByPattern(tt.Input.Args[0].(string), tt.Input.Args[1].(string))
			assert(t, "FindByPattern", res1, tt.Expectation.Ret[0])
			assert(t, "FindByPattern", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMGetDistinctTestCase = []TestCase{}

func Test_tenantORM_GetDistinct(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMGetDistinctTestCase {
		t.Run("GetDistinct_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.GetDistinct(tt.Input.Args[0].(string))
			assert(t, "GetDistinct", res1, tt.Expectation.Ret[0])
			assert(t, "GetDistinct", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMFindWithFiltersTestCase = []TestCase{}

func Test_tenantORM_FindWithFilters(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMFindWithFiltersTestCase {
		t.Run("FindWithFilters_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindWithFilters(tt.Input.Args[0].(map[string]interface{}), tt.Input.Args[1].(string), tt.Input.Args[2].(int), tt.Input.Args[3].(int))
			assert(t, "FindWithFilters", res1, tt.Expectation.Ret[0])
			assert(t, "FindWithFilters", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMTouchTestCase = []TestCase{}

func Test_tenantORM_Touch(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMTouchTestCase {
		t.Run("Touch_"+strconv.Itoa(i), func(t *testing.T) {
			res1 := do.Touch(tt.Input.Args[0].(uint64))
			assert(t, "Touch", res1, tt.Expectation.Ret[0])
		})
	}
}

var TenantORMExistsTestCase = []TestCase{}

func Test_tenantORM_Exists(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMExistsTestCase {
		t.Run("Exists_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.Exists(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "Exists", res1, tt.Expectation.Ret[0])
			assert(t, "Exists", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMExistsByIdTestCase = []TestCase{}

func Test_tenantORM_ExistsById(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMExistsByIdTestCase {
		t.Run("ExistsById_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsById(tt.Input.Args[0].(uint64))
			assert(t, "ExistsById", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsById", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMFindBySubqueryTestCase = []TestCase{}

func Test_tenantORM_FindBySubquery(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMFindBySubqueryTestCase {
		t.Run("FindBySubquery_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.FindBySubquery(tt.Input.Args[0].(string), tt.Input.Args[1].(string), tt.Input.Args[2].(string))
			assert(t, "FindBySubquery", res1, tt.Expectation.Ret[0])
			assert(t, "FindBySubquery", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMExistsAndGetTestCase = []TestCase{}

func Test_tenantORM_ExistsAndGet(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMExistsAndGetTestCase {
		t.Run("ExistsAndGet_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGet(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGet", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGet", res2, tt.Expectation.Ret[1])
		})
	}
}

var TenantORMExistsAndGetActiveTestCase = []TestCase{}

func Test_tenantORM_ExistsAndGetActive(t *testing.T) {
	tenantORM := newTenantORM(_gen_test_db)
	do := tenantORM.WithContext(context.Background()).Debug()

	for i, tt := range TenantORMExistsAndGetActiveTestCase {
		t.Run("ExistsAndGetActive_"+strconv.Itoa(i), func(t *testing.T) {
			res1, res2 := do.ExistsAndGetActive(tt.Input.Args[0].(string), tt.Input.Args[1].(interface{}))
			assert(t, "ExistsAndGetActive", res1, tt.Expectation.Ret[0])
			assert(t, "ExistsAndGetActive", res2, tt.Expectation.Ret[1])
		})
	}
}
