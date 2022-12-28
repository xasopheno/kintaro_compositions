import fs from "fs";
console.log("hello logs");

const eventTypes = [
  "createHome",
  "createRoom",
  "acceptFriendRequest",
  "createUserGroup",
  "SendMessageMutation",
];

type RawEvent = {
  id: string;
  source_ip: string;
  program: string;
  message: string;
  received_at: string;
  generated_at: string;
  display_received_at: string;
  source_id: number;
  source_name: string;
  hostname: string;
  severity: "Info" | "Warn" | "Debug" | "Error";
  facility: string;
};

export type Datum = {
  program: string;
  operation: string;
  received_at: number;
};

function parseMessage(program: string, message: string): string {
  const parsed = JSON.parse(message);
  try {
    switch (program) {
      case "svc-social": {
        // const isError = parsed.status
        if (parsed.err) {
          return "error";
        } else {
          return parsed.auditPayload.data.operation;
        }
      }
      case "svc-messaging": {
        const operation = parsed.msg.split("operation name: ")[1];
        return operation;
      }
      default:
        throw new Error(`unhandled program type: ${program}`);
    }
  } catch (error) {
    console.log(message);
    throw new Error(`unable to parse message: ${message} with error: ${error}`);
  }
}

function eventToDatum(event: RawEvent): Datum {
  return {
    program: event.program,
    operation: parseMessage(event.program, event.message),
    received_at: new Date(event.received_at).getTime(),
  };
}

export function filenameToData(filename: string) {
  console.log(`${filename}`);
  const events_raw = fs.readFileSync(filename, "utf8");
  const events_raw_split = events_raw.split("\n");

  console.log(`found ${events_raw_split.length} raw events`);
  const events_json_array = events_raw_split
    .slice(0, -1)
    .map((event: string) => {
      try {
        const result = JSON.parse(event);
        return result;
      } catch (error) {
        throw new Error(
          `------ error proccessing ${event}. Got error: ${error}`
        );
      }
    });

  let data: Datum[] = events_json_array.map((e: RawEvent) => eventToDatum(e));
  data = data.filter((d) => eventTypes.includes(d.operation));
  console.log(`found ${data.length} events after filtering`);
  // console.log(data.slice(0, 3));
  return data;
}

// const filename = "./homes.json";

// filenameToData(filename);
