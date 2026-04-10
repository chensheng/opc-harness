use rusqlite::{Connection, Result};
use std::marker::PhantomData;

/// 实体 Trait - 所有数据库模型的基类 Trait
#[allow(dead_code)]
pub trait Entity: Sized + Clone {
    /// 获取表名
    fn table_name() -> &'static str;
    
    /// 获取主键字段名
    fn primary_key() -> &'static str;
    
    /// 获取主键值
    fn get_primary_key(&self) -> &str;
    
    /// 从行中解析实体
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>;
}

/// 泛型 Repository - 提供基础查询和删除操作
#[allow(dead_code)]
pub struct Repository<T: Entity> {
    _marker: PhantomData<T>,
}

impl<T: Entity> Repository<T> {
    /// 按 ID 查询实体
    #[allow(dead_code)]
    pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<T>> {
        let sql = format!(
            "SELECT * FROM {} WHERE {} = ?1",
            T::table_name(),
            T::primary_key()
        );
        
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query_map([id], |row| {
            T::from_row(row)
        })?;
        
        if let Some(row) = rows.next() {
            return Ok(Some(row?));
        }
        Ok(None)
    }
    
    /// 删除实体
    #[allow(dead_code)]
    pub fn delete(conn: &Connection, id: &str) -> Result<()> {
        let sql = format!(
            "DELETE FROM {} WHERE {} = ?1",
            T::table_name(),
            T::primary_key()
        );
        
        conn.execute(&sql, [id])?;
        Ok(())
    }
}
