FROM python:3.8.6 AS base

ENV POETRY_VERSION=1.1.4 \
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

RUN wget --output-document /tmp/signatures.xlsx https://cancer.sanger.ac.uk/signatures/COSMIC_Mutational_Signatures_v3.1.xlsx \
      && echo "de2b0f99aed16d04491b3314bf11063aec72c4da531c63a04e641f08420047ab  /tmp/signatures.xlsx" | sha256sum --check \
      && poetry run python ${MTSG_HOME}/scripts/generate_inputs.py \
            /tmp/signatures.xlsx \
            ${MTSG_HOME}/.venv/lib/python3.8/site-packages/sigproSS/input \
      && rm /tmp/signatures.xlsx


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
