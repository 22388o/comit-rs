import { ChildProcess, spawn } from "child_process";
import { MetaBtsieveConfig } from "./btsieve";
import * as fs from "fs";

export class BtsieveRunner {
    running_btsieves: { [key: string]: ChildProcess };
    private readonly log_dir: string;
    private readonly btsive_bin: string;
    private readonly project_root: string;

    constructor(project_root: string, btsieve_bin: string, log_dir: string) {
        this.running_btsieves = {};
        this.log_dir = log_dir;
        this.btsive_bin = btsieve_bin;
        this.project_root = project_root;
    }

    async ensureBtsievesRunning(btsieves: [string, MetaBtsieveConfig][]) {
        for (let [name, btsieve_config] of btsieves) {
            console.log("Starting Btsieve: " + name);

            if (this.running_btsieves[name]) {
                continue;
            }

            this.running_btsieves[name] = await spawn(this.btsive_bin, [], {
                cwd: this.project_root,
                env: btsieve_config.env,
                stdio: [
                    "ignore",
                    fs.openSync(
                        this.log_dir + "/btsieve-" + name + ".log",
                        "w"
                    ),
                    fs.openSync(
                        this.log_dir + "/btsieve-" + name + ".log",
                        "w"
                    ),
                ],
            });
        }
    }

    async stopBtsieves() {
        let names = Object.keys(this.running_btsieves);

        if (names.length > 0) {
            console.log("Stopping Btsieve(s): " + names.join(", "));
            for (let process of Object.values(this.running_btsieves)) {
                process.kill();
            }
        }

        this.running_btsieves = {};
    }
}
