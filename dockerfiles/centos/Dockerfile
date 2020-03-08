FROM centos:7
ARG pythonver
ENV PYTHON python${pythonver}
ENV HOME /root
ENV PATH $HOME/.pyenv/bin:$HOME/.pyenv/shims:$HOME/.cargo/bin:$HOME/.local/bin:$PATH
COPY . /app
WORKDIR /app/python
RUN yum update -y && yum install -y git gcc make zlib-devel && \
    curl https://pyenv.run | bash
# pyenv prequisits. see https://github.com/pyenv/pyenv/wiki/common-build-problems
RUN yum install @development zlib-devel bzip2 bzip2-devel readline-devel sqlite sqlite-devel openssl-devel xz xz-devel libffi-devel findutils -y
RUN pyenv install $pythonver && pyenv global $pythonver
