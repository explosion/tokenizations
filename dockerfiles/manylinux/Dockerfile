FROM quay.io/pypa/manylinux1_x86_64:2020-03-07-9c5ba95
COPY . /app
ENV HOME /root
ENV PATH $HOME/.cargo/bin:$HOME/.local/bin:$PATH:/opt/python/cp35-cp35m/bin:/opt/python/cp36-cp36m/bin:/opt/python/cp37-cp37m/bin:/opt/python/cp38-cp38/bin
RUN python3 -m pip install maturin poetry tox auditwheel --user && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs  > install.sh && sh install.sh -y && \
    rustup default nightly

WORKDIR /app/python
RUN tox
RUN maturin build --no-sdist --release --strip && python3 -m auditwheel repair target/wheels/*.whl
