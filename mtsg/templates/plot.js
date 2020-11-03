const ETIOLOGIES = {
  SBS1: "Clock-like",
  SBS2: "APOBEC",
  SBS3: "HR-deficiency",
  SBS4: "Tobacco",
  SBS5: "Clock-like",
  SBS6: "MMR-deficiency",
  SBS7a: "UV",
  SBS7b: "UV",
  SBS7c: "UV",
  SBS7d: "UV",
  SBS9: "POLH",
  SBS10a: "POLE",
  SBS10b: "POLE",
  SBS11: "Temozolomide",
  SBS13: "APOBEC",
  SBS15: "MMR-deficiency",
  SBS18: "ROS",
  SBS20: "MMR-deficiency",
  SBS22: "Aristolochic acid",
  SBS24: "Aflatoxin",
  SBS26: "MMR-deficiency",
  SBS29: "Tobacco",
};

const state = {
  diseaseCode: "",
  data: {
    signatures: [],
    reference: [],
    query: [],
  },
};

const colors = Plotly.d3.scale.category20();

const formatSnv = (count) => {
  if (count === 1) {
    return `${count} SNV`;
  } else {
    return `${count} SNVs`;
  }
};

const populateDiseases = () => {
  let uniqueDiseases = {};

  for (let sample of state.data.reference) {
    uniqueDiseases[sample.disease.name] = sample.disease.code;
  }

  let names = Object.keys(uniqueDiseases);
  names.sort();

  const $plot = document.getElementById("plot");

  for (let name of names) {
    $plot.add(new Option(name, uniqueDiseases[name]));
  }

  state.diseaseCode = uniqueDiseases[names[0]];
};

const addEventListeners = () => {
  document.getElementById("plot").addEventListener("change", (event) => {
    state.diseaseCode = event.target.value;
    render();
  });
};

const loadData = () => {
  const payload = document.getElementById("payload").innerText;
  state.data = JSON.parse(payload).data;
};

const buildSignatureTraces = (
  signatures,
  querySamples,
  title,
  xaxis,
  yaxis,
  marker = {}
) => {
  const totals = new Array(signatures.length).fill(0);

  for (let sample of querySamples) {
    for (let i = 0; i < sample.contributions.length; i++) {
      totals[i] += sample.contributions[i];
    }
  }

  const total = totals.reduce((sum, value) => sum + value, 0);

  return signatures
    .map((name, i) => {
      let etiology = ETIOLOGIES[name] ? `<br>${ETIOLOGIES[name]}` : "";

      return {
        x: [totals[i] / total],
        y: [title],
        xaxis,
        yaxis,
        name: `<b>${name}</b>${etiology}`,
        text: [`${formatSnv(totals[i])}<br>${name}${etiology}`],
        hoverinfo: "text",
        orientation: "h",
        type: "bar",
        showlegend: false,
        marker: {
          color: colors(i),
          ...marker,
        },
      };
    })
    .filter((trace) => !trace.x.every((value) => value == 0.0));
};

const buildSampleTraces = (signatures, samples, activeSignatures) => {
  samples = samples.map((sample) => ({
    sample,
    total: sample.contributions.reduce((sum, value) => sum + value, 0),
  }));

  samples.sort((a, b) => a.total - b.total);

  const sampleNames = samples.map(({ sample }) => sample.name);

  const traces = signatures
    .map((name, i) => {
      let etiology = ETIOLOGIES[name] ? `<br>${ETIOLOGIES[name]}` : "";

      return {
        x: samples.map(
          ({ sample }, j) => sample.contributions[i] / samples[j].total
        ),
        y: sampleNames,
        xaxis: "x3",
        yaxis: "y3",
        name: `<b>${name}</b>${etiology}`,
        text: samples.map(
          ({ sample }) =>
            `${formatSnv(sample.contributions[i])}<br>${name}${etiology}`
        ),
        hoverinfo: "text",
        orientation: "h",
        type: "bar",
        marker: {
          color: colors(i),
        },
      };
    })
    .filter((trace) => activeSignatures.has(trace.name));

  let contributionsTrace = {
    x: samples.map(({ total }) => total),
    y: sampleNames,
    xaxis: "x4",
    yaxis: "y4",
    text: samples.map((e) => `${formatSnv(e.total)}<br>${e.sample.name}`),
    hoverinfo: "text",
    orientation: "h",
    type: "bar",
    showlegend: false,
    marker: {
      color: "#911938",
    },
  };

  traces.push(contributionsTrace);

  return traces;
};

const render = () => {
  const {
    data: { query: querySamples, reference: referenceSamples, signatures },
    diseaseCode,
  } = state;

  const filteredReferenceSamples = referenceSamples.filter(
    (sample) => sample.disease.code === diseaseCode
  );

  const referenceSignatureTraces = buildSignatureTraces(
    signatures,
    filteredReferenceSamples,
    `<b>Reference<br>${diseaseCode} (n=${filteredReferenceSamples.length})</b>`,
    "x",
    "y",
    {
      line: {
        width: 2,
      },
    }
  );

  const querySignatureTraces = buildSignatureTraces(
    signatures,
    querySamples,
    `Query<br>(n=${querySamples.length})`,
    "x2",
    "y2"
  );

  let activeSignatures = new Set();

  for (let traces of [referenceSignatureTraces, querySignatureTraces]) {
    for (let trace of traces) {
      activeSignatures.add(trace.name);
    }
  }

  const sampleTraces = buildSampleTraces(
    signatures,
    querySamples,
    activeSignatures
  );

  const data = [
    ...referenceSignatureTraces,
    ...querySignatureTraces,
    ...sampleTraces,
  ];

  renderChart(data);
};

const renderChart = (data) => {
  const layout = {
    margin: {
      t: 40,
    },
    barmode: "stack",
    hovermode: "closest",
    annotations: [
      {
        text: "Cohort Signature Contribution Means",
        xref: "paper",
        yref: "paper",
        xanchor: "center",
        yanchor: "bottom",
        x: 0.45,
        y: 1.01,
        showarrow: false,
        font: {
          size: 14,
        },
      },
      {
        text: "Sample Signature Contributions",
        xref: "paper",
        yref: "paper",
        xanchor: "center",
        yanchor: "bottom",
        x: 0.45,
        y: 0.71,
        showarrow: false,
        font: {
          size: 14,
        },
      },
      {
        text: "Sample Signature Activities",
        xref: "paper",
        yref: "paper",
        xanchor: "center",
        yanchor: "bottom",
        x: 0.95,
        y: 0.71,
        showarrow: false,
        font: {
          size: 14,
        },
      },
    ],
    legend: {
      orientation: "h",
      traceorder: "normal",
      valign: "top",
    },
    xaxis: {
      anchor: "y",
      domain: [0.0, 0.9],
      showticklabels: false,
    },
    yaxis: {
      anchor: "x",
      domain: [0.9, 1.0],
      ticklen: 8,
      automargin: true,
    },
    xaxis2: {
      anchor: "y2",
      domain: [0.0, 0.9],
    },
    yaxis2: {
      anchor: "x2",
      domain: [0.8, 0.9],
      ticklen: 8,
    },
    xaxis3: {
      anchor: "y3",
      domain: [0.0, 0.9],
      title: "Proportion of SNVs",
    },
    yaxis3: {
      anchor: "x3",
      domain: [0.0, 0.7],
      ticklen: 8,
      automargin: true,
    },
    xaxis4: {
      anchor: "y4",
      domain: [0.9, 1.0],
      title: "Total Mutational Burden",
    },
    yaxis4: {
      anchor: "x4",
      domain: [0.0, 0.7],
      showticklabels: false,
    },
  };

  const config = {
    responsive: true,
  };

  Plotly.newPlot("chart", data, layout, config);
};

document.addEventListener("DOMContentLoaded", () => {
  loadData();
  populateDiseases();
  addEventListeners();
  render();
});
