use crate::column::Column;
use crate::sql_builder::{SqlArg, SqlBuilder};

#[derive(Debug, Clone)]
pub enum Function<'a, COLUMN: Column> {
    // Postgres Functions
    Count(COLUMN),
    Sum(COLUMN),
    Avg(COLUMN),
    Max(COLUMN),
    Min(COLUMN),
    Abs(COLUMN),
    Ceil(COLUMN),
    Floor(COLUMN),
    Round(COLUMN, SqlArg<'a>),
    Coalesce(COLUMN, SqlArg<'a>),
    Lower(COLUMN),
    Upper(COLUMN),
    Length(COLUMN),
    Now,

    // PostGIS Functions
    STAsText(COLUMN),
    STAsGeoJSON(COLUMN),
    STDistance(COLUMN, SqlArg<'a>),
    STDistanceSphere(COLUMN, SqlArg<'a>),
    STDWithin(COLUMN, SqlArg<'a>, SqlArg<'a>),
    STIntersects(COLUMN, SqlArg<'a>),
    STContains(COLUMN, SqlArg<'a>),
    STWithin(COLUMN, SqlArg<'a>),
    STBuffer(COLUMN, SqlArg<'a>),
    STTransform(COLUMN, SqlArg<'a>),
    STSetSRID(COLUMN, SqlArg<'a>),
    STGeomFromText(SqlArg<'a>),
    STGeomFromTextWithSRID(SqlArg<'a>, SqlArg<'a>),
    STMakePoint(SqlArg<'a>, SqlArg<'a>),
    STX(COLUMN),
    STY(COLUMN),
    STZ(COLUMN),
    STSRID(COLUMN),
    STIsClosed(COLUMN),
    STIsEmpty(COLUMN),
    STIsRing(COLUMN),
    STIsSimple(COLUMN),
    STIsValid(COLUMN),
    STLength(COLUMN),
    STArea(COLUMN),
    STCentroid(COLUMN),
}

impl<'a, COLUMN: Column> Function<'a, COLUMN> {
    pub fn column(&self, builder: &mut SqlBuilder<'a>) {
        match self {
            Function::Count(column) => {
                builder.write_sql("count(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Sum(column) => {
                builder.write_sql("sum(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Avg(column) => {
                builder.write_sql("avg(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Max(column) => {
                builder.write_sql("max(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Min(column) => {
                builder.write_sql("min(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Abs(column) => {
                builder.write_sql("abs(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Ceil(column) => {
                builder.write_sql("ceil(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Floor(column) => {
                builder.write_sql("floor(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Round(column, precision) => {
                builder.write_sql("round(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*precision);
                builder.write_sql(")");
            }
            Function::Coalesce(column, default) => {
                builder.write_sql("coalesce(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*default);
                builder.write_sql(")");
            }
            Function::Lower(column) => {
                builder.write_sql("lower(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Upper(column) => {
                builder.write_sql("upper(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Length(column) => {
                builder.write_sql("length(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::Now => {
                builder.write_sql("now()");
            }
            Function::STAsText(column) => {
                builder.write_sql("ST_AsText(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STAsGeoJSON(column) => {
                builder.write_sql("ST_AsGeoJSON(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STDistance(column, geom) => {
                builder.write_sql("ST_Distance(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*geom);
                builder.write_sql(")");
            }
            Function::STDistanceSphere(column, geom) => {
                builder.write_sql("ST_DistanceSphere(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*geom);
                builder.write_sql(")");
            }
            Function::STDWithin(column, geom, distance) => {
                builder.write_sql("ST_DWithin(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*geom);
                builder.write_sql(", ");
                builder.push_arg(*distance);
                builder.write_sql(")");
            }
            Function::STIntersects(column, geom) => {
                builder.write_sql("ST_Intersects(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*geom);
                builder.write_sql(")");
            }
            Function::STContains(column, geom) => {
                builder.write_sql("ST_Contains(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*geom);
                builder.write_sql(")");
            }
            Function::STWithin(column, geom) => {
                builder.write_sql("ST_Within(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*geom);
                builder.write_sql(")");
            }
            Function::STBuffer(column, radius) => {
                builder.write_sql("ST_Buffer(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*radius);
                builder.write_sql(")");
            }
            Function::STTransform(column, srid) => {
                builder.write_sql("ST_Transform(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*srid);
                builder.write_sql(")");
            }
            Function::STSetSRID(column, srid) => {
                builder.write_sql("ST_SetSRID(");
                column.column(builder);
                builder.write_sql(", ");
                builder.push_arg(*srid);
                builder.write_sql(")");
            }
            Function::STGeomFromText(text) => {
                builder.write_sql("ST_GeomFromText(");
                builder.push_arg(*text);
                builder.write_sql(")");
            }
            Function::STGeomFromTextWithSRID(text, srid) => {
                builder.write_sql("ST_GeomFromText(");
                builder.push_arg(*text);
                builder.write_sql(", ");
                builder.push_arg(*srid);
                builder.write_sql(")");
            }
            Function::STMakePoint(x, y) => {
                builder.write_sql("ST_MakePoint(");
                builder.push_arg(*x);
                builder.write_sql(", ");
                builder.push_arg(*y);
                builder.write_sql(")");
            }
            Function::STX(column) => {
                builder.write_sql("ST_X(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STY(column) => {
                builder.write_sql("ST_Y(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STZ(column) => {
                builder.write_sql("ST_Z(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STSRID(column) => {
                builder.write_sql("ST_SRID(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STIsClosed(column) => {
                builder.write_sql("ST_IsClosed(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STIsEmpty(column) => {
                builder.write_sql("ST_IsEmpty(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STIsRing(column) => {
                builder.write_sql("ST_IsRing(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STIsSimple(column) => {
                builder.write_sql("ST_IsSimple(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STIsValid(column) => {
                builder.write_sql("ST_IsValid(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STLength(column) => {
                builder.write_sql("ST_Length(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STArea(column) => {
                builder.write_sql("ST_Area(");
                column.column(builder);
                builder.write_sql(")");
            }
            Function::STCentroid(column) => {
                builder.write_sql("ST_Centroid(");
                column.column(builder);
                builder.write_sql(")");
            }
        }
    }
}

impl<'a, COLUMN: Column> crate::column::TColumns<'a> for Function<'a, COLUMN> {
    fn columns(&self, builder: &mut SqlBuilder<'a>) {
        self.column(builder);
    }
}

impl<'a, COLUMN: Column> crate::column::TColumns<'a> for Vec<Function<'a, COLUMN>> {
    fn columns(&self, builder: &mut SqlBuilder<'a>) {
        self.iter().enumerate().for_each(|(index, column)| {
            if index != 0 {
                builder.write_sql(", ")
            }
            column.column(builder);
        });
    }
}

pub trait FunctionTrait: Column + Sized {
    fn count(self) -> Function<'static, Self> {
        Function::Count(self)
    }
    fn sum(self) -> Function<'static, Self> {
        Function::Sum(self)
    }
    fn avg(self) -> Function<'static, Self> {
        Function::Avg(self)
    }
    fn max(self) -> Function<'static, Self> {
        Function::Max(self)
    }
    fn min(self) -> Function<'static, Self> {
        Function::Min(self)
    }
    fn abs(self) -> Function<'static, Self> {
        Function::Abs(self)
    }
    fn ceil(self) -> Function<'static, Self> {
        Function::Ceil(self)
    }
    fn floor(self) -> Function<'static, Self> {
        Function::Floor(self)
    }
    fn round<'a>(self, precision: SqlArg<'a>) -> Function<'a, Self> {
        Function::Round(self, precision)
    }
    fn coalesce<'a>(self, default: SqlArg<'a>) -> Function<'a, Self> {
        Function::Coalesce(self, default)
    }
    fn lower(self) -> Function<'static, Self> {
        Function::Lower(self)
    }
    fn upper(self) -> Function<'static, Self> {
        Function::Upper(self)
    }
    fn length(self) -> Function<'static, Self> {
        Function::Length(self)
    }

    // PostGIS
    fn st_as_text(self) -> Function<'static, Self> {
        Function::STAsText(self)
    }
    fn st_as_geojson(self) -> Function<'static, Self> {
        Function::STAsGeoJSON(self)
    }
    fn st_distance<'a>(self, geom: SqlArg<'a>) -> Function<'a, Self> {
        Function::STDistance(self, geom)
    }
    fn st_distance_sphere<'a>(self, geom: SqlArg<'a>) -> Function<'a, Self> {
        Function::STDistanceSphere(self, geom)
    }
    fn st_d_within<'a>(self, geom: SqlArg<'a>, distance: SqlArg<'a>) -> Function<'a, Self> {
        Function::STDWithin(self, geom, distance)
    }
    fn st_intersects<'a>(self, geom: SqlArg<'a>) -> Function<'a, Self> {
        Function::STIntersects(self, geom)
    }
    fn st_contains<'a>(self, geom: SqlArg<'a>) -> Function<'a, Self> {
        Function::STContains(self, geom)
    }
    fn st_within<'a>(self, geom: SqlArg<'a>) -> Function<'a, Self> {
        Function::STWithin(self, geom)
    }
    fn st_buffer<'a>(self, radius: SqlArg<'a>) -> Function<'a, Self> {
        Function::STBuffer(self, radius)
    }
    fn st_transform<'a>(self, srid: SqlArg<'a>) -> Function<'a, Self> {
        Function::STTransform(self, srid)
    }
    fn st_set_srid<'a>(self, srid: SqlArg<'a>) -> Function<'a, Self> {
        Function::STSetSRID(self, srid)
    }
    fn st_x(self) -> Function<'static, Self> {
        Function::STX(self)
    }
    fn st_y(self) -> Function<'static, Self> {
        Function::STY(self)
    }
    fn st_z(self) -> Function<'static, Self> {
        Function::STZ(self)
    }
    fn st_srid(self) -> Function<'static, Self> {
        Function::STSRID(self)
    }
    fn st_is_closed(self) -> Function<'static, Self> {
        Function::STIsClosed(self)
    }
    fn st_is_empty(self) -> Function<'static, Self> {
        Function::STIsEmpty(self)
    }
    fn st_is_ring(self) -> Function<'static, Self> {
        Function::STIsRing(self)
    }
    fn st_is_simple(self) -> Function<'static, Self> {
        Function::STIsSimple(self)
    }
    fn st_is_valid(self) -> Function<'static, Self> {
        Function::STIsValid(self)
    }
    fn st_length(self) -> Function<'static, Self> {
        Function::STLength(self)
    }
    fn st_area(self) -> Function<'static, Self> {
        Function::STArea(self)
    }
    fn st_centroid(self) -> Function<'static, Self> {
        Function::STCentroid(self)
    }
}

impl<T: Column> FunctionTrait for T {}
