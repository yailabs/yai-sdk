// Public configuration is owner input; query does not select an endpoint or grant authority.
import type { ResourceImportInput, ResourceAction, WebSearchResult } from "../../typescript/workflows.js";
export const connection:ResourceImportInput={case_ref:"case:isolated",definition:{schema:"yai.resource_definition.v1",attachment_id:"resource:search",policy_owner:"participant:human",participant_ids:["participant:model"],operations:["web_search"],read_prefixes:[],names:[],max_output_bytes:65536,max_items:8,address:{kind:"search_api",provider:"tavily",endpoint:{endpoint:"https://api.tavily.com/search",allowed_ip_addresses:["1.1.1.1"],credential_ref:"YAI_TAVILY_TOKEN"}}}};
export const query:ResourceAction={action:"web_search",query:"YVEX public contracts",limit:4};
// @ts-expect-error Search queries cannot select a private endpoint.
export const arbitrary:ResourceAction={action:"web_search",query:"private",limit:4,url:"http://192.168.1.70/"};
export function sources(result:WebSearchResult):string[]{return result.hits.map(hit=>hit.url);}
