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
    samples: [],
  },
};

const colors = Plotly.d3.scale.category20();

const selectFirstDiseaseCode = () => {
  const $option = document.querySelector("#plot option:first-child");
  state.diseaseCode = $option.value;
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

const buildSignatureTraces = (signatures, samples, diseaseCode) => {
  const diseaseTotals = new Array(signatures.length).fill(0);

  for (let sample of samples) {
    if (sample.diseaseCode === diseaseCode) {
      for (let i = 0; i < sample.contributions.length; i++) {
        diseaseTotals[i] += sample.contributions[i];
      }
    }
  }

  const total = diseaseTotals.reduce((sum, value) => sum + value, 0);

  return signatures.map((name, i) => ({
    x: [diseaseTotals[i] / total],
    y: [diseaseCode],
    text: `${diseaseTotals[i]}<br>${name}`,
    hoverinfo: "text",
    orientation: "h",
    type: "bar",
    showlegend: false,
    marker: {
      color: colors(i),
    },
  }));
};

const buildSampleTraces = (signatures, samples, diseaseCode) => {
  samples = samples
    .filter((s) => s.diseaseCode === diseaseCode)
    .map((sample) => ({
      sample,
      total: sample.contributions.reduce((sum, value) => sum + value, 0),
    }));

  samples.sort((a, b) => a.total - b.total);

  const sampleNames = samples.map(({ sample }) => sample.name);

  const traces = signatures
    .map((name, i) => ({
      x: samples.map(
        ({ sample }, j) => sample.contributions[i] / samples[j].total
      ),
      y: sampleNames,
      xaxis: "x2",
      yaxis: "y2",
      name: `<b>${name}</b>${
        ETIOLOGIES[name] ? `<br>${ETIOLOGIES[name]}` : ""
      }`,
      text: samples.map(
        ({ sample }) => `${sample.contributions[i]}<br>${name}`
      ),
      hoverinfo: "text",
      orientation: "h",
      type: "bar",
      marker: {
        color: colors(i),
      },
    }))
    .filter((trace) => !trace.x.every((value) => value == 0.0));

  let contributionsTrace = {
    x: samples.map(({ total }) => total),
    y: sampleNames,
    xaxis: "x3",
    yaxis: "y3",
    text: samples.map((e) => `${e.total}<br>${e.sample.name}`),
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
    data: { samples, signatures },
    diseaseCode,
  } = state;

  const signatureTraces = buildSignatureTraces(
    signatures,
    samples,
    diseaseCode
  );
  const sampleTraces = buildSampleTraces(signatures, samples, diseaseCode);
  const data = [...signatureTraces, ...sampleTraces];

  const filteredSamples = samples.filter((s) => s.diseaseCode === diseaseCode);

  renderChart(diseaseCode, data, filteredSamples.length);
};

const renderChart = (title, data, sampleCount) => {
  const layout = {
    title,
    barmode: "stack",
    hovermode: "closest",
    annotations: [
      {
        text: `Signature contribution means (n = ${sampleCount})`,
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
        text: "Sample signature contributions",
        xref: "paper",
        yref: "paper",
        xanchor: "center",
        yanchor: "bottom",
        x: 0.45,
        y: 0.81,
        showarrow: false,
        font: {
          size: 14,
        },
      },
      {
        text: "Sample signature activities",
        xref: "paper",
        yref: "paper",
        xanchor: "center",
        yanchor: "bottom",
        x: 0.95,
        y: 0.81,
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
      anchor: "y1",
      domain: [0.025, 0.9],
    },
    yaxis: {
      anchor: "x1",
      domain: [0.9, 1.0],
      ticklen: 8,
    },
    xaxis2: {
      anchor: "y2",
      domain: [0.025, 0.9],
      title: "Percent contribution",
    },
    yaxis2: {
      anchor: "x2",
      domain: [0.0, 0.8],
      ticklen: 8,
    },
    xaxis3: {
      anchor: "y3",
      domain: [0.9, 1.0],
      title: "Total absolute contribution",
    },
    yaxis3: {
      anchor: "x3",
      domain: [0.0, 0.8],
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
  selectFirstDiseaseCode();
  addEventListeners();
  render();
});
