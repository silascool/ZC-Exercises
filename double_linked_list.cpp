#include <stdio.h>


//
typedef struct node {
    int data;
    struct node * right, * left;
} NODE;

void insert (NODE *Prev, NODE *New) {
    New->right = Prev->right;
    New->left = Prev;
    if (New->right) {
        New->right->left = New;
    }
    Prev->right = New;
}

int main() {
    NODE f1 = {
        1,
        nullptr,
        nullptr,
    };
    NODE f2 = {
        1,
        nullptr,
        nullptr,
    };
    //f1.right = &f2;
    //f2.left = &f2;

    //NODE f3;
    //f3.data = 3;
    insert(&f1, &f2);
    
    printf("%d\n", f1.right->data);
}