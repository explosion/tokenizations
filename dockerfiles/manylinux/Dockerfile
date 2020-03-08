FROM quay.io/pypa/manylinux1_x86_64:2020-03-07-9c5ba95
COPY . /app
ENV HOME /root
ENV PATH $HOME/.cargo/bin:$HOME/.local/bin:$PATH:/opt/python/cp35-cp35m/bin:/opt/python/cp36-cp36m/bin:/opt/python/cp37-cp37m/bin:/opt/python/cp38-cp38/bin
RUN python3 -m pip install maturin poetry tox auditwheel twine --user && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --profile minimal -y && \
    rustup default nightly

WORKDIR /app
RUN cargo test
WORKDIR /app/python
RUN tox
RUN maturin build --no-sdist --release --strip && find target/ -type f -name *whl | xargs -I {} python3 -m auditwheel repair {}
