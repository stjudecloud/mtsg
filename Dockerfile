FROM python:3.8.5

ENV POETRY_VERSION=1.0.10 \
      POETRY_HOME=/opt/poetry \
      POETRY_VIRTUALENVS_IN_PROJECT=true \
      MTSG_HOME=/opt/mtsg \
      PATH=/opt/poetry/bin:$PATH

RUN curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python

WORKDIR ${MTSG_HOME}

COPY poetry.lock pyproject.toml ./
RUN poetry install --no-dev

RUN poetry run python -c 'from SigProfilerMatrixGenerator.install import install; install("GRCh38")'

COPY mtsg/ ./mtsg/

ENTRYPOINT ["poetry", "run", "mtsg"]
