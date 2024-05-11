expression    -> unary | binary | grouping | literal

unary         -> ("-" | "not") expression
binary        -> expression operator expression
grouping      -> "(" expression ")"
literal       -> NUMBER | STRING | "True" | "False" | "None"

operator      -> "+" | "-" | "*" | "/"
              | "==" | "!=" | "<" | "<=" | ">" | ">="
