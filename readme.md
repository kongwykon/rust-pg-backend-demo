# 安全统一稳定的Rust后端实践
> 安全来自Rust的严格的内存所有权规则，最大程度避免出现内存泄漏而服务挂机。   
> 高并发和稳定与其Go，C++相同性能的媲美。   
> 统一来自于Rust优秀的类型系统、强静态类型语言的设定，代码规范和模式比较显式展现。

## 项目技术栈
语言和生态： Rust、Postgres  
数据库：Postgres  
服务框架：Axum（in rust）  
工具拓展：Diesel（ORM in rust-postgres）  
>（附）前端展示： HTMX, tailwind, Hyperscript

## 已实现功能特性
JWT登陆  
session校验  
中间件模式  
Database Migration (by Diesel)  
可支持SSR（server side rendering）  
DDD（领域模型代码结构）

## CRUD示例
| table ||||
|--|--|--|--|
| user | id | username | pw_hash | 
|server| id | name     | ip      |


## API
user -
-  sign_in, sign_up, sign_out, 

server -
- create, delete, list

middleware -
-  auth JWT
