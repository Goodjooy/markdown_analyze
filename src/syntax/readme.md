文法

Items -> Item Items
    | Nil
item ->  parg |title | ref | unorder_list | Div_line

// 一整行
full_line -> line_start_stament statments  

// 表达式

statments -> statment staments
        |   Nil

statment -> plain | link | image | code_snippet | trans | blob

// 表达式块

block -> block_meta block 
        | Nil
 
block_meta -> statments \n |
           -> statments

// 标题
title -> title_token  statments

// 段落
parg -> block ·  \n· | block

// link
link -> [ statments ] ( plain ` ` statments )
|     [ statments ] ( plain )

// image
image -> ![ statments ] ( plain ` ` statments )
|   [ statments ] ( plain )

//引用

ref -> >* statments

// 列表部分
unorder_list -> *| - statments \n  indent listinner
            |  *| - statments \n  

order_list -> \d. statments \n  indent listinner
        |  \d. statments \n  

list_inner -> unorder_list | order_list | ref | parg | indent block

//强调文法

blob -> * binner *
binner -> * sinner * | statments
sinner -> * statments * | statments