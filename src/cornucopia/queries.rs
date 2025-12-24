// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod item
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(serde::Serialize, Debug, Clone, PartialEq,)] pub struct Item
{ pub id : String,pub created_at : time::PrimitiveDateTime,pub updated_at : Option<time::PrimitiveDateTime>,pub deleted_at : Option<time::PrimitiveDateTime>,pub business_id : String,pub r#type : String,pub item_status : String,pub ref_code : String,pub sku_code : String,pub bar_code : String,pub erp_code : Option<String>,pub name_mon : String,pub name_eng : Option<String>,pub name_bill : Option<String>,pub name_web : String,pub name_app : String,pub delivery_type_id : Option<String>,pub brand_id : Option<String>,pub supplier_id : Option<String>,pub manufacturer_id : Option<String>,pub origin_country : Option<String>,pub importer_country : Option<String>,pub distributor_id : Option<String>,pub item_type_id : String,pub classification_id : String,pub sub_classification_id : String,pub category_id : Option<String>,pub sub_category_id : Option<String>,pub tag_id : Option<String>,pub description : String,pub has_variant : bool,pub variant_selected : bool,pub has_additional_unit : bool,pub department_unit_id : Option<String>,pub department_sub_unit_id : Option<String>,pub supplier_type : Option<String>,pub return_allow : Option<bool>,pub return_type : Option<String>,pub image : Option<String>,pub package_type_id : Option<String>,pub is_completed : bool,pub is_fetched : bool,pub warehouse_id : Option<String>,pub merch_staff_id : Option<String>,pub fetched_item_id : Option<String>,}pub struct ItemBorrowed<'a> { pub id : &'a str,pub created_at : time::PrimitiveDateTime,pub updated_at : Option<time::PrimitiveDateTime>,pub deleted_at : Option<time::PrimitiveDateTime>,pub business_id : &'a str,pub r#type : &'a str,pub item_status : &'a str,pub ref_code : &'a str,pub sku_code : &'a str,pub bar_code : &'a str,pub erp_code : Option<&'a str>,pub name_mon : &'a str,pub name_eng : Option<&'a str>,pub name_bill : Option<&'a str>,pub name_web : &'a str,pub name_app : &'a str,pub delivery_type_id : Option<&'a str>,pub brand_id : Option<&'a str>,pub supplier_id : Option<&'a str>,pub manufacturer_id : Option<&'a str>,pub origin_country : Option<&'a str>,pub importer_country : Option<&'a str>,pub distributor_id : Option<&'a str>,pub item_type_id : &'a str,pub classification_id : &'a str,pub sub_classification_id : &'a str,pub category_id : Option<&'a str>,pub sub_category_id : Option<&'a str>,pub tag_id : Option<&'a str>,pub description : &'a str,pub has_variant : bool,pub variant_selected : bool,pub has_additional_unit : bool,pub department_unit_id : Option<&'a str>,pub department_sub_unit_id : Option<&'a str>,pub supplier_type : Option<&'a str>,pub return_allow : Option<bool>,pub return_type : Option<&'a str>,pub image : Option<&'a str>,pub package_type_id : Option<&'a str>,pub is_completed : bool,pub is_fetched : bool,pub warehouse_id : Option<&'a str>,pub merch_staff_id : Option<&'a str>,pub fetched_item_id : Option<&'a str>,}
impl<'a> From<ItemBorrowed<'a>> for Item
{
    fn from(ItemBorrowed { id,created_at,updated_at,deleted_at,business_id,r#type,item_status,ref_code,sku_code,bar_code,erp_code,name_mon,name_eng,name_bill,name_web,name_app,delivery_type_id,brand_id,supplier_id,manufacturer_id,origin_country,importer_country,distributor_id,item_type_id,classification_id,sub_classification_id,category_id,sub_category_id,tag_id,description,has_variant,variant_selected,has_additional_unit,department_unit_id,department_sub_unit_id,supplier_type,return_allow,return_type,image,package_type_id,is_completed,is_fetched,warehouse_id,merch_staff_id,fetched_item_id,}: ItemBorrowed<'a>) ->
    Self { Self { id: id.into(),created_at,updated_at,deleted_at,business_id: business_id.into(),r#type: r#type.into(),item_status: item_status.into(),ref_code: ref_code.into(),sku_code: sku_code.into(),bar_code: bar_code.into(),erp_code: erp_code.map(|v| v.into()),name_mon: name_mon.into(),name_eng: name_eng.map(|v| v.into()),name_bill: name_bill.map(|v| v.into()),name_web: name_web.into(),name_app: name_app.into(),delivery_type_id: delivery_type_id.map(|v| v.into()),brand_id: brand_id.map(|v| v.into()),supplier_id: supplier_id.map(|v| v.into()),manufacturer_id: manufacturer_id.map(|v| v.into()),origin_country: origin_country.map(|v| v.into()),importer_country: importer_country.map(|v| v.into()),distributor_id: distributor_id.map(|v| v.into()),item_type_id: item_type_id.into(),classification_id: classification_id.into(),sub_classification_id: sub_classification_id.into(),category_id: category_id.map(|v| v.into()),sub_category_id: sub_category_id.map(|v| v.into()),tag_id: tag_id.map(|v| v.into()),description: description.into(),has_variant,variant_selected,has_additional_unit,department_unit_id: department_unit_id.map(|v| v.into()),department_sub_unit_id: department_sub_unit_id.map(|v| v.into()),supplier_type: supplier_type.map(|v| v.into()),return_allow,return_type: return_type.map(|v| v.into()),image: image.map(|v| v.into()),package_type_id: package_type_id.map(|v| v.into()),is_completed,is_fetched,warehouse_id: warehouse_id.map(|v| v.into()),merch_staff_id: merch_staff_id.map(|v| v.into()),fetched_item_id: fetched_item_id.map(|v| v.into()),} }
}pub struct ItemQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> ItemBorrowed,
    mapper: fn(ItemBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> ItemQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(ItemBorrowed) -> R) ->
    ItemQuery<'a,C,R,N>
    {
        ItemQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn items() -> ItemsStmt
{ ItemsStmt(cornucopia_async::private::Stmt::new("select * from item")) } pub struct
ItemsStmt(cornucopia_async::private::Stmt); impl ItemsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> ItemQuery<'a,C,
Item, 0>
{
    ItemQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { ItemBorrowed { id: row.get(0),created_at: row.get(1),updated_at: row.get(2),deleted_at: row.get(3),business_id: row.get(4),r#type: row.get(5),item_status: row.get(6),ref_code: row.get(7),sku_code: row.get(8),bar_code: row.get(9),erp_code: row.get(10),name_mon: row.get(11),name_eng: row.get(12),name_bill: row.get(13),name_web: row.get(14),name_app: row.get(15),delivery_type_id: row.get(16),brand_id: row.get(17),supplier_id: row.get(18),manufacturer_id: row.get(19),origin_country: row.get(20),importer_country: row.get(21),distributor_id: row.get(22),item_type_id: row.get(23),classification_id: row.get(24),sub_classification_id: row.get(25),category_id: row.get(26),sub_category_id: row.get(27),tag_id: row.get(28),description: row.get(29),has_variant: row.get(30),variant_selected: row.get(31),has_additional_unit: row.get(32),department_unit_id: row.get(33),department_sub_unit_id: row.get(34),supplier_type: row.get(35),return_allow: row.get(36),return_type: row.get(37),image: row.get(38),package_type_id: row.get(39),is_completed: row.get(40),is_fetched: row.get(41),warehouse_id: row.get(42),merch_staff_id: row.get(43),fetched_item_id: row.get(44),} }, mapper: |it| { <Item>::from(it) },
    }
} }}}