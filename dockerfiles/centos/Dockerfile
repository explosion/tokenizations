FROM centos:7
ENV HOME /root
ENV PATH $HOME/.pyenv/bin:$HOME/.pyenv/shims:$HOME/.cargo/bin:$HOME/.local/bin:$PATH
COPY . /app
WORKDIR /app/python
RUN yum update -y && yum install -y git gcc make zlib-devel && \
    curl https://pyenv.run | bash
# pyenv prequisits. see https://github.com/pyenv/pyenv/wiki/common-build-problems
RUN yum install @development zlib-devel bzip2 bzip2-devel readline-devel sqlite sqlite-devel openssl-devel xz xz-devel libffi-devel findutils -y
RUN pyenv install 3.8.2 && \
    pyenv install 3.7.6 && \
    pyenv install 3.6.10 && \
    pyenv install 3.5.9 && \
    pyenv global 3.5.9 3.6.10 3.7.6 3.8.2 && \
    python3 -m pip install maturin poetry tox --user && \
    # maturin requires nightly rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs  > install.sh && sh install.sh -y && \
    rustup default nightly
RUN tox