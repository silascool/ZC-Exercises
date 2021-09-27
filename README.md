# ZC-Exercises

C/C++ libvirt exercise
----------------------
To compile:\
  gcc -g -Wall ex_libvirt.c -o ex_libvirt -lvirt\
Then run\
  ./ex_libvirt \
  
  Here can you see the output compared to virsh list --all

![comparison to virsh listt --all](https://github.com/silascool/ZC-Exercises/blob/main/Screenshot%20from%202021-09-27%2022-12-32.png)
 

C/C++ Double linked list exercise
--------------------------------
There is two things to be aware off, when implementing insert.
  - First that we dont overide the Prev-right before we have updated the right node.
  - Secondly that we are aware off that Prev-right could be NULL.


RUST Library exercise
--------------------
There are 4 commands you can preform in the libary program:
  - quit or q, quits libary.
  - list, list libarys selection off books and their status.
  - borrow [book title], borrows the specified book from the libary.
  - return [book title], returns the specified book from the libary.
  (all words in a book title start with capital lettters)\
  example:  borrow War And Peace
