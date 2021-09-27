# ZC-Exercises

C/C++ libvirt exercise\
To compile:\
  gcc -g -Wall ex_libvirt.c -o ex_libvirt -lvirt\
Then run\
  ./ex_libvirt\
    
\
\ 
\
C/C++ Double linked list exercise\
There is two things to be aware off, when implementing insert.\
  First that we dont overide the Prev-right before we have updated the right node.\
  Secondly that we are aware off that Prev-right could be NULL.\
\
void insert (NODE *Prev, NODE *New) {\
    New->right = Prev->right;\
    New->left = Prev;\
    if (New->right) {\
        New->right->left = New;\
    }\
    Prev->right = New;\
}\
\
RUST Library exercise\
There are 4 commands you can preform in the libary program:\
  quit or q, quits libary.\
  list, list libarys selection off books and their status.\
  borrow [book title], borrows the specified book from the libary.\
  return [book title], returns the specified book from the libary.\
  (all words in a book title start with capital lettters)
  example:  borrow War And Peace
