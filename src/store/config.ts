import {defineStore} from 'pinia'
import {invoke} from "@tauri-apps/api/tauri";

export const useConfigStore = defineStore('counter', {
    state: () => ({
        version: "",
        third_party_image_viewer: "",
        third_party_open: false,
        delete_source_file: false
    } as Config),
})

interface Config {
    version: string,
    third_party_image_viewer: string,
    third_party_open: boolean,
    delete_source_file: boolean,

    [key: string]: any
}

interface ConfigMap {
    key: string,
    value: string,
}

const config = useConfigStore()
await getConfig(config)
export async function getConfig(config: Config) {
    let res = await invoke("get_config") as Array<ConfigMap>
    console.log(res)
    for (let re of res) {
        let key = re.key
        config[key] = re.value
    }
}