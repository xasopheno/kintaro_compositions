import fs from "fs";
import { reduce, sortBy, min, groupBy, uniqBy } from "lodash";
import { Datum, filenameToData } from "./process";

const events = ["homes", "rooms", "friends", "messages"] as const;
type EventType = typeof events[number];
type Data = { [K in EventType]: Datum[] };

const normalizeBy = 100000.0;
let data = {} as Data;
events.forEach((e) => {
  data[e] = filenameToData(`${e}.json`);
});

const firstEvents = Object.values(data).map((d) => d[0]);
const firstTimes = firstEvents.map((e) => e.received_at);
const minStartTime = Math.min(...firstTimes);
console.log(minStartTime);

type CsvInput = {
  event: EventType;
  minStartTime: number;
};

function deleteFile(filename: string) {
  fs.rmSync(filename, {
    force: true,
  });
}

function makeCsv(input: CsvInput, data: Data) {
  let events = data[input.event];
  let normalized = events.map((e) => {
    return { ...e, received_at: (e.received_at - minStartTime) / normalizeBy };
  });
  console.log("total events", normalized.length);

  let uniqued = uniqBy(normalized, "received_at");
  let i = 0;
  let byLength = uniqued.map((t) => {
    t.received_at = i === 0 ? 1.0 : t.received_at - i;
    i = t.received_at + i;
    return t;
  });

  console.log("after filtering", byLength.length);

  const outputFilename = `${input.event}.csv`;
  deleteFile(outputFilename);
  console.log(`writing: ${outputFilename}`);
  const writer = fs.createWriteStream(outputFilename, {
    flags: "a",
  });

  byLength.forEach((v) => {
    writer.write(`0.0,${v.received_at}\n`);
  });
}

events.forEach((event) => {
  console.log(`___${event}___`, event);
  makeCsv({ event, minStartTime }, data);
});
