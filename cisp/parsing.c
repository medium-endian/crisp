#include <stdio.h>
#include <stdlib.h>

#include <editline/readline.h>
#include <histedit.h>

#include "mpc.h"

long eval_op(long x, char* op, long y) {
    switch(op[0]) {
        case '+': return x + y;
        case '-': return x - y;
        case '*': return x * y;
        case '/': return x / y;
        case '%': return x % y;
    }
    printf("Unknown operator: %s", op);
    return 0;
}

int main(int argc, char** argv) {
    printf("4 * 2: %li\n", eval_op(4, "*", 2));

    /* Create some parsers */
    mpc_parser_t* Number     = mpc_new("number");
    mpc_parser_t* Operator   = mpc_new("operator");
    mpc_parser_t* Expr       = mpc_new("expr");
    mpc_parser_t* Lispy      = mpc_new("lispy");

    /* Define the parsers with the following language */
    mpca_lang(MPCA_LANG_DEFAULT,
            "                                                     \
              number   : /(-?[1-9][0-9]*)/ ;                      \
              operator : '+' | '-' | '*' | '/' | '%' ;            \
              expr     : <number> | '(' <operator> <expr>+ ')' ;  \
              lispy    : /^/ <operator> <expr>+ /$/ ;             \
            ",
            Number, Operator, Expr, Lispy);

    /* Version and exit information */
    puts("Lispy version 0.0.0.0.1\n");
    puts("Press CTRL-C to exit\n");

    puts("Example expression: * 2 2 or * (+ 1 5) (* 1 3 7)");

    while (1) {
        char* input = readline("cispy >> ");

        add_history(input);

        printf("%s\n", input);

        /* Attempt to parse the input */
        mpc_result_t r;
        if (mpc_parse("<stdin>", input, Lispy, &r)) {
            /* Success - print the AST */
            mpc_ast_print(r.output);
            mpc_ast_delete(r.output);
        } else {
            /* Not parsed. Print error */
            mpc_err_print(r.error);
            mpc_err_delete(r.error);
        }

        /* readline does malloc */
        free(input);
    }

    /* Undefine and delete the parsers */
    mpc_cleanup(4, Number, Operator, Expr, Lispy);

    return 0;
}

