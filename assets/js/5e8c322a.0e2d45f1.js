"use strict";(self.webpackChunkauto_traffic_control=self.webpackChunkauto_traffic_control||[]).push([[597],{3905:(e,t,r)=>{r.d(t,{Zo:()=>s,kt:()=>m});var a=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},i=Object.keys(e);for(a=0;a<i.length;a++)r=i[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)r=i[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var c=a.createContext({}),p=function(e){var t=a.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},s=function(e){var t=p(e.components);return a.createElement(c.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,i=e.originalType,c=e.parentName,s=l(e,["components","mdxType","originalType","parentName"]),d=p(r),m=n,f=d["".concat(c,".").concat(m)]||d[m]||u[m]||i;return r?a.createElement(f,o(o({ref:t},s),{},{components:r})):a.createElement(f,o({ref:t},s))}));function m(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var i=r.length,o=new Array(i);o[0]=d;var l={};for(var c in t)hasOwnProperty.call(t,c)&&(l[c]=t[c]);l.originalType=e,l.mdxType="string"==typeof e?e:n,o[1]=l;for(var p=2;p<i;p++)o[p]=r[p];return a.createElement.apply(null,o)}return a.createElement.apply(null,r)}d.displayName="MDXCreateElement"},7926:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>o,default:()=>u,frontMatter:()=>i,metadata:()=>l,toc:()=>p});var a=r(7462),n=(r(7294),r(3905));const i={sidebar_position:1,title:"Introduction"},o="Introduction",l={unversionedId:"api/index",id:"api/index",title:"Introduction",description:"Auto Traffic Control provides a [gRPC] API that players can use to interact",source:"@site/docs/api/index.md",sourceDirName:"api",slug:"/api/",permalink:"/docs/api/",draft:!1,editUrl:"https://github.com/jdno/auto-traffic-control/tree/main/docs/docs/api/index.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Introduction"},sidebar:"api",next:{title:"Events",permalink:"/docs/api/events"}},c={},p=[{value:"Address",id:"address",level:2},{value:"Services",id:"services",level:2}],s={toc:p};function u(e){let{components:t,...r}=e;return(0,n.kt)("wrapper",(0,a.Z)({},s,r,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("h1",{id:"introduction"},"Introduction"),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"Auto Traffic Control")," provides a ",(0,n.kt)("a",{parentName:"p",href:"https://grpc.io/"},"gRPC")," API that players can use to interact\nwith the game. Official client libraries for popular programming languages are\ngenerated from the ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/jdno/auto-traffic-control/tree/main/api"},"API specification"),"."),(0,n.kt)("table",null,(0,n.kt)("thead",{parentName:"table"},(0,n.kt)("tr",{parentName:"thead"},(0,n.kt)("th",{parentName:"tr",align:null},"Language"),(0,n.kt)("th",{parentName:"tr",align:null},"Package"))),(0,n.kt)("tbody",{parentName:"table"},(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:null},"Node"),(0,n.kt)("td",{parentName:"tr",align:null},(0,n.kt)("a",{parentName:"td",href:"https://www.npmjs.com/package/auto-traffic-control"},(0,n.kt)("img",{parentName:"a",src:"https://img.shields.io/npm/v/auto-traffic-control",alt:"npm"})))),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:null},"Rust"),(0,n.kt)("td",{parentName:"tr",align:null},(0,n.kt)("a",{parentName:"td",href:"https://crates.io/crates/auto-traffic-control"},(0,n.kt)("img",{parentName:"a",src:"https://img.shields.io/crates/v/auto-traffic-control",alt:"Crates.io"})))))),(0,n.kt)("h2",{id:"address"},"Address"),(0,n.kt)("p",null,"The API is served at the following location:"),(0,n.kt)("pre",null,(0,n.kt)("code",{parentName:"pre",className:"language-text"},"http://localhost:4747\n")),(0,n.kt)("h2",{id:"services"},"Services"),(0,n.kt)("p",null,"The API is organized into different services, each with its own area of\nresponsibility."),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"The ",(0,n.kt)("a",{parentName:"li",href:"/docs/api/Services/airplane-service"},(0,n.kt)("inlineCode",{parentName:"a"},"AirplaneService"))," provides\ninformation about the airplanes on the map, and allows updating their flight\nplans."),(0,n.kt)("li",{parentName:"ul"},"The ",(0,n.kt)("a",{parentName:"li",href:"/docs/api/Services/atc-service"},(0,n.kt)("inlineCode",{parentName:"a"},"AtcService"))," provides information about\n",(0,n.kt)("strong",{parentName:"li"},"Auto Traffic Control")," itself, for example its current version."),(0,n.kt)("li",{parentName:"ul"},"The ",(0,n.kt)("a",{parentName:"li",href:"/docs/api/Services/event-service"},(0,n.kt)("inlineCode",{parentName:"a"},"EventService"))," streams every change in\nthe game to the player."),(0,n.kt)("li",{parentName:"ul"},"The ",(0,n.kt)("a",{parentName:"li",href:"/docs/api/Services/game-service"},(0,n.kt)("inlineCode",{parentName:"a"},"GameService"))," can be used to start a\nnew game."),(0,n.kt)("li",{parentName:"ul"},"The ",(0,n.kt)("a",{parentName:"li",href:"/docs/api/Services/map-service"},(0,n.kt)("inlineCode",{parentName:"a"},"MapService"))," provides information about\nthe map and can convert between coordinate systems.")))}u.isMDXComponent=!0}}]);