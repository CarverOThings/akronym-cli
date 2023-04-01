# akronym-cli
Akronym is a Rust command-line tool for managing a dictionary of IT/Technology definitions. It allows users to input new definitions and query existing definitions using acronyms. The program uses the rusqlite library for database interaction and is designed with a simple and intuitive user interface.

-This project came from the need to quickly find definitions to acronyms without opening the browser. Learning to program on my own I see a lot of acronyms and after starting my career in IT/Cybersecurity I was swamped with acronyms. Thus Akronym. CLI is the most straightforward way since I spend a good deal of time on the terminal. Or at least one hotkey away...  
-I must preface early on to anyone who stumbles across this project. I barely know how to program. I took Harvards Cs50 last year and learned some c along the way. I do not know rust much at all at the time of writing this. I chose rust because it is a powerful language that I'd like to know. I am reading the books and completing the rustlings course. Of course I am using chat-GPT to help me with the planning side of things but I am shying away from copy pasting code since that is not what this project is about. I want to learn rust and learn it well. Oh and have a cool tool I can use with my studies.

Planned Features: 
  - CLI
  - Simple CLI usage 
  - akrym [OPTIONS] <ARGUMENT>
    -Ex: $ akrym -r IT 
       - $ IT - Informaion Technology, is the use of computers to create, process, store, retrieve and exchange all kinds of data and information. IT forms part of information and communications technology (ICT). An information technology system (IT system) is generally an information system, a communications system, or, more specifically speaking, a computer system — including all hardware, software, and peripheral equipment — operated by a limited group of IT users. 
  - Query an acronym, return definition
  - Insert acronym
