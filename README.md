#Refactor svg for fontello format:



##HOW TO USE - 

```
 
factorsvg --input=src/input/  --output=src/output/ --multi --multithread

```

###WHERE:

- -- positioning arguments
- --input - path to file or directory (if multi flag is exists, then directory)
- --output - path to export file or directory (if multi flag is exists, then directory)
- --mutli - flag, that check is file or directory (if is file, then it will fix only this file, if dir - fix all files with .svg ext)
- --multithread - run operation in multithreading