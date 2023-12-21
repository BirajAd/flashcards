This is a Rust based CLI application I built to go through my flashcards. Running the program is relatively easier and requires that you only have ```cargo``` installed.

The program takes one argument, which is the path of the file where your flashcard content is, currently I only support csv file. However, down the line I may expand to accept more file formats. 

You can run the program by executing the following command:

```cargo run <file-path>```

The program is interactive and will ask you the number of terms you want to review, and if you didn't know the answer to some of the terms, you can simply say no and it will ask you if you want to see the terms you got wrong. If you say yes, it will give you the list of those terms

Here is how your csv file should look like:


| term | meaning |
| ------|------- |
| probity | honesty, integrity |
| cupidity | greed |

As you can see the csv file will have 2 headers/columns. One is term and other one is for meaning.
