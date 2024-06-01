# EdiCLI

A.K.A. Editor Command Line Parser is a lightweight CLI text editor,
after all, *a quick editor for quick changes*. You don't have to use
a full featured text editor or IDE to just delete or add a line on a file.
If you are on my side then, download and install EdiCLI and run `edicli --help`
to see a list with the all awesome commands availables.

## Setup

First of all, clone the repository with the following command:
```
git clone https://github.com/Wachamuli/EdiCLI.git
```
Move the project folder to ``~/.local/bin/`` directory (optional):
```
mv EdiCLI ~/.local/bin/
```
Now create a symbolic link to the binay file:
```
ln -s EdiCLI/target/realease/edicli edicli
```
Try a command to check if EdiCLI is working properly, for example:
```
edicli --help
```
![alt text](./extra/image-1.png)

## Showcase

Edicli is based on three simple commands: `write`, `rewrite` and `delete`.
And one aditional flag `--show` or `-s` in short. Every command needs an input file but the 
arguments may vary.

![alt text](./extra/image.png)
```
edicli write <file> <text>
```

![alt text](./extra/image-2.png)
![alt text](./extra/image-3.png)
```
edicli delete <file> <line>
```

![alt text](./extra/image-4.png)
```
edicli rewrite <file> <line> <text>
```