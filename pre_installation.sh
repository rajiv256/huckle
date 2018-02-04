curl https://sh.rustup.rs -sSf | sh
# Add $HOME/.cargo/bin to the PATH in .bashrc
export PATH=$HOME/.cargo/bin:$PATH
rustup toolchain install nightly-2017-05-04-x86_64-unknown-linux-gnu
git clone https://github.com/rajiv256/ouros.git
cd ouros
rustup override add nightly-2017-04-05
sudo apt-get install uml-utilities
sudo apt-get install bridge-utils
