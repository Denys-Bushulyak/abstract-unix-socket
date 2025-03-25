import { Record } from "./fable_modules/fable-library-js.4.24.0/Types.js";
import { record_type, string_type } from "./fable_modules/fable-library-js.4.24.0/Reflection.js";

export class Message extends Record {
    constructor(Name, Message) {
        super();
        this.Name = Name;
        this.Message = Message;
    }
}

export function Message_$reflection() {
    return record_type("Program.Message", [], Message, () => [["Name", string_type], ["Message", string_type]]);
}

export function handle(message) {
    return new Message("Denys", message);
}

