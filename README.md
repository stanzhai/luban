## LuBan

鲁班：产品原型托管服务

## Dev

基于数据表生成sea-orm的entity：

```
# sea-orm-cli仅需安装一次
cargo install sea-orm-cli
sea-orm-cli generate entity -u postgres://postgres:postgres@localhost/luban -o src/entity 
```

表设计：

```
-- Table Definition
CREATE TABLE IF NOT EXISTS "project" (
    "id" bpchar(21) NOT NULL,
    "name" varchar(64) NOT NULL,
    "owner" varchar(64) NOT NULL,
    "comment" text,
    "ctime" timestamp NOT NULL DEFAULT now(),
    "is_del" bool NOT NULL DEFAULT false,
    PRIMARY KEY ("id")
);

-- Column Comment
COMMENT ON COLUMN "project"."id" IS 'nanoid，唯一id';
COMMENT ON COLUMN "project"."name" IS '项目名称';
COMMENT ON COLUMN "project"."owner" IS '项目创建人';
COMMENT ON COLUMN "project"."comment" IS '备注';
COMMENT ON COLUMN "project"."ctime" IS '创建时间';
COMMENT ON COLUMN "project"."is_del" IS '是否被删除';

-- Table Definition
CREATE TABLE IF NOT EXISTS "prototype" (
    "id" bpchar(21) NOT NULL,
    "project_id" bpchar(21) NOT NULL,
    "name" varchar(128) NOT NULL,
    "owner" varchar(64) NOT NULL,
    "comment" text,
    "ctime" timestamp NOT NULL DEFAULT now(),
    "version" int4 NOT NULL,
    "is_del" bool NOT NULL,
    PRIMARY KEY ("id")
);

-- Column Comment
COMMENT ON COLUMN "prototype"."id" IS 'nanoid，唯一id';
COMMENT ON COLUMN "prototype"."project_id" IS '所属项目id';
COMMENT ON COLUMN "prototype"."name" IS '产品原型名称';
COMMENT ON COLUMN "prototype"."owner" IS '所有者';
COMMENT ON COLUMN "prototype"."comment" IS '备注';
COMMENT ON COLUMN "prototype"."ctime" IS '创建时间';
COMMENT ON COLUMN "prototype"."version" IS '版本';
COMMENT ON COLUMN "prototype"."is_del" IS '是否被删除';
```