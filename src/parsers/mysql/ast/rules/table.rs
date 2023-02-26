pub mod RuleTable {
    #[derive(Debug)]
    pub enum ColumnType {
        TinyInt(i32),
        SmallInt(i32),
        MediumInt(i32),
        Int(i32),
        BigInt(i64),
        Float(f32),
        Double(f64),
        Decimal(f64),
        Char(i32),
        VarChar(i32),
        Binary(i32),
        VarBinary(i32),
        TinyBlob,
        Blob,
        MediumBlob,
        LongBlob,
        TinyText,
        Text,
        MediumText,
        LongText,
        Date,
        DateTime,
        Time,
        Timestamp,
        Year(i32),
    }
    
    #[derive(Debug)]
    pub struct Column {
        pub name: String,
        pub col_type: ColumnType,
        pub not_null: bool,
        pub auto_increment: bool,
        pub unique: bool,
        pub primary_key: bool,
    }
    
    #[derive(Debug)]
    pub struct Table {
        pub name: String,
        pub columns: Vec<Column>,
        pub primary_key: Option<Vec<String>>,
        pub foreign_keys: Vec<ForeignKey>,
    }
    
    #[derive(Debug)]
    pub struct ForeignKey {
        pub column: String,
        pub ref_table: String,
        pub ref_column: String,
    }
    
    #[derive(Debug)]
    pub enum TableConstraint {
        PrimaryKey(Vec<String>),
        ForeignKey(ForeignKey),
        Unique(Vec<String>),
    }
}