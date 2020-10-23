FROM python:3.8.6

ENV POETRY_VERSION=1.1.3 \
      POETRY_HOME=/opt/poetry \
      POETRY_VIRTUALENVS_IN_PROJECT=true \
      MTSG_HOME=/opt/mtsg \
      PATH=/opt/poetry/bin:$PATH

RUN sed -i "s/ main/ main contrib/g" /etc/apt/sources.list \
      && apt-get update \
      && apt-get --yes install ttf-mscorefonts-installer \
      && rm -rf /var/lib/apt/lists/*

RUN curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python

WORKDIR ${MTSG_HOME}

COPY poetry.lock pyproject.toml ./
RUN poetry install --no-dev

RUN poetry run python -c 'from SigProfilerMatrixGenerator.install import install; install("GRCh38")'

COPY mtsg/ ./mtsg/

ENTRYPOINT ["poetry", "run", "mtsg"]
