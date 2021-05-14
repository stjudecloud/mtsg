# syntax=docker/dockerfile:1

FROM python:3.8.8 AS base

ENV POETRY_VERSION=1.1.6 \
      POETRY_HOME=/opt/poetry \
      POETRY_VIRTUALENVS_IN_PROJECT=true \
      MTSG_HOME=/opt/mtsg \
      PATH=/opt/poetry/bin:$PATH

RUN sed -i "s/ main/ main contrib/g" /etc/apt/sources.list \
      && apt-get update \
      && apt-get --yes install ttf-mscorefonts-installer \
      && rm -rf /var/lib/apt/lists/*

RUN curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python

WORKDIR $MTSG_HOME

COPY poetry.lock pyproject.toml ./
RUN poetry install --no-dev

RUN poetry run python -c 'from SigProfilerMatrixGenerator.install import install; install("GRCh38")'

COPY scripts/ ./scripts/

RUN wget --output-document /tmp/signatures.tsv https://cancer.sanger.ac.uk/signatures/documents/442/COSMIC_v3.1_SBS_GRCh38.txt \
      && echo "2c3a4cf0ebde8ae2a163d4a4196c097fa762d2fb6ce8330ee4c462bc71b388a3  /tmp/signatures.tsv" | sha256sum --check \
      && poetry run python ${MTSG_HOME}/scripts/generate_inputs.py \
            /tmp/signatures.tsv \
            ${MTSG_HOME}/.venv/lib/python3.8/site-packages/sigproSS/input \
      && rm /tmp/signatures.tsv


FROM base as development

COPY mtsg/main.py ./mtsg/

COPY --from=base $POETRY_HOME $POETRY_HOME
COPY --from=base $MTSG_HOME $MTSG_HOME

VOLUME ["/opt/mtsg/mtsg", "/opt/mtsg/tests"]

RUN poetry install

ENTRYPOINT ["/opt/mtsg/.venv/bin/mtsg"]


FROM base as release

COPY --from=base $MTSG_HOME $MTSG_HOME
COPY mtsg/ ./mtsg/

RUN poetry install --no-dev

ENTRYPOINT ["/opt/mtsg/.venv/bin/mtsg"]
