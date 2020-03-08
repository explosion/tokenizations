FROM quay.io/pypa/manylinux1_x86_64:latest
ENV PATH=$HOME/.cargo/bin:$HOME/.local/bin:$PATH \
    PATH=/opt/python/cp35-cp35m/bin:$PATH \
    PATH=/opt/python/cp36-cp36m/bin:$PATH \
    PATH=/opt/python/cp37-cp37m/bin:$PATH  \
    PATH=/opt/python/cp38-cp38m/bin:$PATH
RUN python3 -m pip install maturin poetry tox --user && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs  > install.sh && sh install.sh -y && \
    rustup default nightly
RUN tox