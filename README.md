### Credits 

- The Unikernel I used for this project has been built in its entirety excluding the Interrupt Handling by following this well written blog  [Writing an OS in Rust](https://os.phil-opp.com/), by Phillip Oppermann. 

- The following blogs helped me in organizing my project and also helped by pointing me in the right direction when I was stuck in a quagmire of triple faults. 
  - https://jvns.ca/blog/2014/03/12/the-rust-os-story/
  - https://github.com/ryanra/RustOS (Helped me a lot in organizing my code and understanding how PCI devices work)

Will add instructions as to how to download and run this server on a virtual-box. 

Long live Open Source!

#### Installing Requirements

Install qemu, xorriso and nasm. 

#### Setting up the environment 

Install Rust using the following command :

  > curl https://sh.rustup.rs -sSf | sh

This will install rustup(the tool chain installer), rustc(the compiler), cargo(the package manager). Now we are using so many functions that are unstable, so we need to use the nightly versions of the compiler and the package manager. So, we need to change the version of the rust used in the project directory. This is where Rustup comes to aid. Using Rustup you can install tool chain of any version given that you know the date of release. Fortunately we have that info. Take a look below.

`rajiv@rajiv-Inspiron-3537:~/CodingIsFun/ouros$ rustc --version`
> rustc 1.18.0-nightly (91ae22a01 2017-04-05)
`rajiv@rajiv-Inspiron-3537:~/CodingIsFun/ouros$ cargo --version`
> cargo 0.19.0-nightly (4e95c6b41 2017-03-23)


First fork the repository huckle-final to your github. And then clone it into your computer.
Assuming you already have Rust installed.

Now go to the project root directory. Inside it, override the versions in this directory to the above nightly versions. Use the below command.

​​rustup override add nightly-2017-04-05​

I installed the same thing again by creating a new clone from github. I built the code and it worked under this version.
Here are the final versions that you might get.
'''
rajiv@rajiv-Inspiron-3537:/huckle$ rustc --version`
rustc 1.18.0-nightly (2564711e8 2017-04-04)

rajiv@rajiv-Inspiron-3537:/huckle$ cargo --version`
cargo 0.19.0-nightly (4e95c6b41 2017-03-23)
''' 
This sets your system up and you are good to go!!

#### How to run?
  - `make iso`

  - `make run`

  - `make clean`
